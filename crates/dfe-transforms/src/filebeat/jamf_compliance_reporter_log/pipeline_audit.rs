// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_audit` pipeline.
pub struct PipelineAudit;

impl Transform for PipelineAudit {
    fn name(&self) -> &str {
        "pipeline_audit"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("jamf_compliance_reporter.log.dataset", json!("audit"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json._event_score") {
                if let Some(val) = event.get("json._event_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json._event_score".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_score", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.header.event_id") {
                if let Some(val) = event.get("json.header.event_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.header.event_id".into(),
                            message,
                        })?;
                    event.set("event.code", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.header.event_modifier") {
                if let Some(val) = event.get("json.header.event_modifier") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.header.event_modifier".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.header.event_modifier", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.header.event_name") {
                    event.rename("json.header.event_name", "event.action")?;
                }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.has_value("json.header.time_seconds_epoch") && event.get_i64("json.header.time_seconds_epoch") != Some(0) };
            if _cond {
                // Painless script
                // Source: ctx.json.time_milliseconds = (long)ctx.json.header.time_seconds_epoch * 1000;\nif (ctx.json?.header?.time_milliseconds_offset != null && ctx.json.header.time_milliseconds_offset != 0) {\n  ctx.json.time_milliseconds = ctx.json.time_milliseconds + (long)ctx.json.header.time_milliseconds_offset;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.json.time_milliseconds = (long)ctx.json.header.time_seconds_epoch * 1000;\nif (ctx.json?.header?.time_milliseconds_offset != null && ctx.json.header.time_milliseconds_offset != 0) {\n  ctx.json.time_milliseconds = ctx.json.time_milliseconds + (long)ctx.json.header.time_milliseconds_offset;\n}\n"#))?;
            }

            let _cond = { event.has_value("json.time_milliseconds") && event.get_i64("json.time_milliseconds") != Some(0) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.time_milliseconds") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.time_milliseconds".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.header.version") {
                if let Some(val) = event.get("json.header.version") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.header.version".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.header.version", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.host_info.host_name") {
                    event.rename("json.host_info.host_name", "host.hostname")?;
                }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hosts", json!(event.get("host.hostname").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                if event.has_value("json.host_info.host_uuid") {
                    event.rename("json.host_info.host_uuid", "jamf_compliance_reporter.log.host_info.host.uuid")?;
                }

                if event.has_value("json.host_info.osversion") {
                    event.rename("json.host_info.osversion", "host.os.version")?;
                }

            let _cond = { event.has_value("json.host_info.primary_mac_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("host.mac", json!(event.get("json.host_info.primary_mac_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

                if event.has_value("json.host_info.serial_number") {
                    event.rename("json.host_info.serial_number", "host.id")?;
                }

                if event.has_value("json.return.description") {
                    event.rename("json.return.description", "jamf_compliance_reporter.log.return.description")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.return.error") {
                if let Some(val) = event.get("json.return.error") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.return.error".into(),
                            message,
                        })?;
                    event.set("error.code", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.get_str("error.code") == Some("0") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("error.code") != Some("0") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.return.return_value") {
                if let Some(val) = event.get("json.return.return_value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.return.return_value".into(),
                            message,
                        })?;
                    event.set("process.exit_code", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.audit_id") {
                if let Some(val) = event.get("json.subject.audit_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.audit_id".into(),
                            message,
                        })?;
                    event.set("process.real_user.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.subject.audit_user_name") {
                    event.rename("json.subject.audit_user_name", "process.real_user.name")?;
                }

            let _cond = { event.has_value("process.real_user.name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("process.real_user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.subject.audit_user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.subject.audit_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.effective_group_id") {
                if let Some(val) = event.get("json.subject.effective_group_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.effective_group_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.effective.group.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.subject.effective_group_name") {
                    event.rename("json.subject.effective_group_name", "jamf_compliance_reporter.log.subject.effective.group.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.effective_user_id") {
                if let Some(val) = event.get("json.subject.effective_user_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.effective_user_id".into(),
                            message,
                        })?;
                    event.set("process.user.id", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.subject.effective_user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.subject.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.subject.effective_user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.subject.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                if event.has_value("json.subject.effective_user_name") {
                    event.rename("json.subject.effective_user_name", "process.user.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.group_id") {
                if let Some(val) = event.get("json.subject.group_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.group_id".into(),
                            message,
                        })?;
                    event.set("process.real_group.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.subject.group_name") {
                    event.rename("json.subject.group_name", "process.real_group.name")?;
                }

                if event.has_value("json.subject.process_hash") {
                    event.rename("json.subject.process_hash", "process.hash.sha1")?;
                }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("process.hash.sha1").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.subject.process_id") {
                if let Some(val) = event.get("json.subject.process_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.process_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.process.pid", converted)?;
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

                if event.has_value("json.subject.process_name") {
                    event.rename("json.subject.process_name", "jamf_compliance_reporter.log.subject.process.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.session_id") {
                if let Some(val) = event.get("json.subject.session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.session_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.session.id", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.terminal_id.addr") {
                if let Some(val) = event.get("json.subject.terminal_id.addr") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.terminal_id.addr".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.terminal_id.addr", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.subject.terminal_id.ip_address") {
                if let Some(val) = event.get("json.subject.terminal_id.ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.terminal_id.ip_address".into(),
                            message,
                        })?;
                    event.set("json.subject.terminal_id.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("json.subject.terminal_id.ip_address");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("host.ip", json!(event.get("json.subject.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("json.subject.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.subject.terminal_id.port") {
                if let Some(val) = event.get("json.subject.terminal_id.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.terminal_id.port".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.terminal_id.port", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.terminal_id.type") {
                if let Some(val) = event.get("json.subject.terminal_id.type") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.terminal_id.type".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.terminal_id.type", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.user_id") {
                if let Some(val) = event.get("json.subject.user_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.user_id".into(),
                            message,
                        })?;
                    event.set("user.id", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.subject.user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.subject.user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.subject.user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.subject.user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                event.append("event.type", json!("info"))?;

            event.set("event.kind", json!("event"))?;

                event.append("event.category", json!("authentication"))?;

            let _cond = { event.get_str("event.action") == Some("aue_accept") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_accept"
                if event.has_value("json.path") {
                event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.socket_unix.family") {
                if let Some(val) = event.get("json.socket_unix.family") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.socket_unix.family".into(),
                message,
                })?;
                event.set("json.inet_family", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.socket_unix.path") {
                event.rename("json.socket_unix.path", "jamf_compliance_reporter.log.socket.unix.path")?;
                }
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
                let _cond = { event.has_value("json.inet_family") };
                if _cond {
                // Painless script
                // Source: Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.unix.family = map.get(ctx.json.inet_family);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.unix.family = map.get(ctx.json.inet_family);\n"#))?;
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
                // End nested pipeline: "pipeline_aue_accept"
            }

            let _cond = { ["aue_auth_user", "aue_ssauthorize", "aue_ssauthmech"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_auth"
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
                if event.has_value("json.texts") {
                event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                }
                // End nested pipeline: "pipeline_aue_auth"
            }

            let _cond = { ["aue_bind", "aue_connect"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_bind_and_aue_connect"
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
                // End nested pipeline: "pipeline_aue_bind_and_aue_connect"
            }

            let _cond = { event.get_str("event.action") == Some("aue_chdir") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_chdir"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.device") {
                if let Some(val) = event.get("json.attributes.device") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.device".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.device", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.file_system_id") {
                if let Some(val) = event.get("json.attributes.file_system_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.file_system_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.file.system.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.node_id") {
                if let Some(val) = event.get("json.attributes.node_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.node_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.node.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_group_id") {
                if let Some(val) = event.get("json.attributes.owner_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_group_id".into(),
                message,
                })?;
                event.set("user.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.owner_group_name") {
                event.rename("json.attributes.owner_group_name", "user.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_user_id") {
                if let Some(val) = event.get("json.attributes.owner_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_user_id".into(),
                message,
                })?;
                event.set("json.attributes.owner_user_id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.attributes.owner_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.id", json!(event.get("json.attributes.owner_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
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
                if event.has_value("json.path") {
                event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }
                let _cond = { event.has_value("json.file_access_mode") };
                if _cond {
                // Painless script
                // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n"#))?;
                }
                // End nested pipeline: "pipeline_aue_chdir"
            }

            let _cond = { event.get_str("event.action") == Some("aue_chroot") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_chroot"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.device") {
                if let Some(val) = event.get("json.attributes.device") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.device".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.device", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.file_access_mode") {
                event.rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.file_system_id") {
                if let Some(val) = event.get("json.attributes.file_system_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.file_system_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.file.system.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.node_id") {
                if let Some(val) = event.get("json.attributes.node_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.node_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.node.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_group_id") {
                if let Some(val) = event.get("json.attributes.owner_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_group_id".into(),
                message,
                })?;
                event.set("user.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.owner_group_name") {
                event.rename("json.attributes.owner_group_name", "user.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_user_id") {
                if let Some(val) = event.get("json.attributes.owner_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_user_id".into(),
                message,
                })?;
                event.set("json.attributes.owner_user_id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.attributes.owner_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.id", json!(event.get("json.attributes.owner_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
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
                if event.has_value("json.path") {
                event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
                let _cond = { event.has_value("json.file_access_mode") };
                if _cond {
                // Painless script
                // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n"#))?;
                }
                // End nested pipeline: "pipeline_aue_chroot"
            }

            let _cond = { event.get_str("event.action") == Some("aue_execve") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_execve"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.device") {
                if let Some(val) = event.get("json.attributes.device") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.device".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.device", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.file_system_id") {
                if let Some(val) = event.get("json.attributes.file_system_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.file_system_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.file.system.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.node_id") {
                if let Some(val) = event.get("json.attributes.node_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.node_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.node.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_group_id") {
                if let Some(val) = event.get("json.attributes.owner_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_group_id".into(),
                message,
                })?;
                event.set("user.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.owner_group_name") {
                event.rename("json.attributes.owner_group_name", "user.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_user_id") {
                if let Some(val) = event.get("json.attributes.owner_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_user_id".into(),
                message,
                })?;
                event.set("json.attributes.owner_user_id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.attributes.owner_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.id", json!(event.get("json.attributes.owner_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
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
                if event.has_value("json.path") {
                event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_args.args") {
                event.rename("json.exec_args.args", "json.args")?;
                }
                if event.has_value("json.exec_args.args_compiled") {
                event.rename("json.exec_args.args_compiled", "jamf_compliance_reporter.log.exec_args.args_compiled")?;
                }
                let _cond = { event.has_value("json.exec_env.env.ARCH") && event.get_str("json.exec_env.env.ARCH") != Some("") };
                if _cond {
                // Painless script
                // Source: for (entry in params.replacements.entrySet()) {\n  if (ctx.json.exec_env.env.ARCH == entry.getKey()) {\n    ctx.json.exec_env.env.put('ARCH', entry.getValue());\n  }\n}\nif (!params.allowed.contains(ctx.json.exec_env.env.ARCH)) {\n  return;\n}\nif (ctx.host == null) {\n  HashMap hm = new HashMap();\n  ctx.put('host', hm);\n}\nif (ctx.host.os == null) {\n  HashMap hm = new HashMap();\n  ctx.host.put('os', hm);\n}\nctx.host.os.put('type', ctx.json.exec_env.env.ARCH);\nctx.json.exec_env.env.remove('ARCH');\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"for (entry in params.replacements.entrySet()) {\n  if (ctx.json.exec_env.env.ARCH == entry.getKey()) {\n    ctx.json.exec_env.env.put('ARCH', entry.getValue());\n  }\n}\nif (!params.allowed.contains(ctx.json.exec_env.env.ARCH)) {\n  return;\n}\nif (ctx.host == null) {\n  HashMap hm = new HashMap();\n  ctx.put('host', hm);\n}\nif (ctx.host.os == null) {\n  HashMap hm = new HashMap();\n  ctx.host.put('os', hm);\n}\nctx.host.os.put('type', ctx.json.exec_env.env.ARCH);\nctx.json.exec_env.env.remove('ARCH');\n"#), cached_params!("{\"allowed\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"],\"replacements\":{\"macintosh\":\"macos\"}}"))?;
                }
                if event.has_value("json.exec_env.env.CPU") {
                event.rename("json.exec_env.env.CPU", "host.architecture")?;
                }
                if event.has_value("json.exec_env.env.MALWAREBYTES_GROUP") {
                event.rename("json.exec_env.env.MALWAREBYTES_GROUP", "jamf_compliance_reporter.log.exec_env.env.malwarebytes_group")?;
                }
                if event.has_value("json.exec_env.env.PATH") {
                event.rename("json.exec_env.env.PATH", "jamf_compliance_reporter.log.exec_env.env.path")?;
                }
                if event.has_value("json.exec_env.env.XPC_FLAGS") {
                event.rename("json.exec_env.env.XPC_FLAGS", "jamf_compliance_reporter.log.exec_env.env.xpc.flags")?;
                }
                if event.has_value("json.exec_env.env.XPC_SERVICE_NAME") {
                event.rename("json.exec_env.env.XPC_SERVICE_NAME", "jamf_compliance_reporter.log.exec_env.env.xpc.service_name")?;
                }
                if event.has_value("json.exec_env.env_compiled") {
                event.rename("json.exec_env.env_compiled", "jamf_compliance_reporter.log.exec_env.env.compiled")?;
                }
                // Painless script
                // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#))?;
                let _cond = { event.has_value("json.file_access_mode") };
                if _cond {
                // Painless script
                // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n"#))?;
                }
                // End nested pipeline: "pipeline_aue_execve"
            }

            let _cond = { event.get_str("event.action") == Some("aue_exit") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_exit"
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
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exit.return_value") {
                if let Some(val) = event.get("json.exit.return_value") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exit.return_value".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.exit.return.value", converted)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.exit.status") {
                if let Some(val) = event.get("json.exit.status") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.exit.status".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.exit.status", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_aue_exit"
            }

            let _cond = { event.get_str("event.action") == Some("aue_kill") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_kill"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.signal") {
                if let Some(val) = event.get("json.arguments.signal") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.signal".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.signal", converted)?;
                }
                }
                Ok(())
                })();
                // Begin nested pipeline: "pipeline_process_object"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.audit_id") {
                if let Some(val) = event.get("json.process.audit_id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.audit_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.effective_group_id") {
                if let Some(val) = event.get("json.process.effective_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.effective_group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.effective.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.effective_group_name") {
                event.rename("json.process.effective_group_name", "jamf_compliance_reporter.log.process.effective.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.effective_user_id") {
                if let Some(val) = event.get("json.process.effective_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.effective_user_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.effective.user.id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.process.effective_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.effective.id", json!(event.get("json.process.effective_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.process.effective_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.effective.name", json!(event.get("json.process.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.process.effective_user_name") {
                event.rename("json.process.effective_user_name", "jamf_compliance_reporter.log.process.effective.user.name")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.effective.user.name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.effective.user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.group_id") {
                if let Some(val) = event.get("json.process.group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.group_name") {
                event.rename("json.process.group_name", "jamf_compliance_reporter.log.process.group.name")?;
                }
                let _cond = { event.has_value("json.process.process_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("process.hash.sha1", json!(event.get("json.process.process_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.process.process_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("json.process.process_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.process_id") {
                if let Some(val) = event.get("json.process.process_id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.process_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
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
                if event.has_value("json.process.process_name") {
                event.rename("json.process.process_name", "process.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.session_id") {
                if let Some(val) = event.get("json.process.session_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.session_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.session.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.addr") {
                if let Some(val) = event.get("json.process.terminal_id.addr") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.addr".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.addr", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.ip_address") {
                if let Some(val) = event.get("json.process.terminal_id.ip_address") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.ip_address".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.ip_address", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("json.process.terminal_id.ip_address");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.terminal_id.ip_address") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("jamf_compliance_reporter.log.process.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.port") {
                if let Some(val) = event.get("json.process.terminal_id.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.port", converted)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.type") {
                if let Some(val) = event.get("json.process.terminal_id.type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.type", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.user_id") {
                if let Some(val) = event.get("json.process.user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.user_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.user.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.user_name") {
                event.rename("json.process.user_name", "jamf_compliance_reporter.log.process.user.name")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.user.name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // End nested pipeline: "pipeline_process_object"
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
                // End nested pipeline: "pipeline_aue_kill"
            }

            let _cond = { event.get_str("event.action") == Some("aue_mount") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_mount"
                if event.has_value("json.texts") {
                event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                }
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
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
                if event.has_value("json.path") {
                event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.device") {
                if let Some(val) = event.get("json.attributes.device") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.device".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.device", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.file_access_mode") {
                event.rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.file_system_id") {
                if let Some(val) = event.get("json.attributes.file_system_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.file_system_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.file.system.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.node_id") {
                if let Some(val) = event.get("json.attributes.node_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.node_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.node.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_group_id") {
                if let Some(val) = event.get("json.attributes.owner_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.owner.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.owner_group_name") {
                event.rename("json.attributes.owner_group_name", "jamf_compliance_reporter.log.attributes.owner.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_user_id") {
                if let Some(val) = event.get("json.attributes.owner_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_user_id".into(),
                message,
                })?;
                event.set("json.attributes.owner_user_id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.attributes.owner_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.id", json!(event.get("json.attributes.owner_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.flags") {
                if let Some(val) = event.get("json.arguments.flags") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.flags".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.flags", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.file_access_mode") };
                if _cond {
                // Painless script
                // Source: int temp = (int)ctx.json?.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"int temp = (int)ctx.json?.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n"#))?;
                }
                // End nested pipeline: "pipeline_aue_mount"
            }

            let _cond = { event.get_str("event.action") == Some("aue_posix_spawn") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_posix_spawn"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.child_PID") {
                if let Some(val) = event.get("json.arguments.child_PID") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.child_PID".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.child.pid", converted)?;
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
                if event.has_value("json.exec_args.args") {
                event.rename("json.exec_args.args", "json.args")?;
                }
                if event.has_value("json.exec_args.args_compiled") {
                event.rename("json.exec_args.args_compiled", "jamf_compliance_reporter.log.exec_args.args_compiled")?;
                }
                if event.has_value("json.exec_env.env.XPC_FLAGS") {
                event.rename("json.exec_env.env.XPC_FLAGS", "jamf_compliance_reporter.log.exec_env.env.xpc.flags")?;
                }
                if event.has_value("json.exec_env.env_compiled") {
                event.rename("json.exec_env.env_compiled", "jamf_compliance_reporter.log.exec_env.env.compiled")?;
                }
                if event.has_value("json.path") {
                event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }
                if event.has_value("json.exec_chain_parent.uuid") {
                event.rename("json.exec_chain_parent.uuid", "jamf_compliance_reporter.log.exec_chain_parent.uuid")?;
                }
                // Painless script
                // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#))?;
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
                // End nested pipeline: "pipeline_aue_posix_spawn"
            }

            let _cond = { ["aue_remove_from_group", "aue_mac_set_proc"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_remove_from_group_and_aue_mac_set_proc"
                if event.has_value("json.texts") {
                event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                }
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
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
                // End nested pipeline: "pipeline_aue_remove_from_group_and_aue_mac_set_proc"
            }

            let _cond = { ["aue_session_end", "aue_session_update", "aue_session_close", "aue_session_start"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_session"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.am_failure") {
                if let Some(val) = event.get("json.arguments.am_failure") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.am_failure".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.am_failure", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.am_success") {
                if let Some(val) = event.get("json.arguments.am_success") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.am_success".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.am_success", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.sflags") {
                if let Some(val) = event.get("json.arguments.sflags") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.sflags".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.sflags", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_aue_session"
            }

            let _cond = { ["aue_setsockopt", "aue_shutdown"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_arguments"
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
                // End nested pipeline: "pipeline_aue_arguments"
            }

            let _cond = { event.get_str("event.action") == Some("aue_ssauthint") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_ssauthint"
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
                if event.has_value("json.texts") {
                event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.known_UID_") {
                if let Some(val) = event.get("json.arguments.known_UID_") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.known_UID_".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.known_uid", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.arguments") {
                event.rename("json.arguments", "jamf_compliance_reporter.log.arguments.flattened")?;
                }
                // End nested pipeline: "pipeline_aue_ssauthint"
            }

            let _cond = { event.get_str("event.action") == Some("aue_tasknameforpid") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_tasknameforpid"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.process") {
                if let Some(val) = event.get("json.arguments.process") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.process".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.process", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.target_port") {
                if let Some(val) = event.get("json.arguments.target_port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.target_port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.target.port", converted)?;
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.task_port") {
                if let Some(val) = event.get("json.arguments.task_port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.task_port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.task.port", converted)?;
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
                // End nested pipeline: "pipeline_aue_tasknameforpid"
            }

            let _cond = { event.get_str("event.action") == Some("aue_unmount") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_unmount"
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
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.path") {
                event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.device") {
                if let Some(val) = event.get("json.attributes.device") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.device".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.device", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.file_access_mode") {
                event.rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.file_system_id") {
                if let Some(val) = event.get("json.attributes.file_system_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.file_system_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.file.system.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.node_id") {
                if let Some(val) = event.get("json.attributes.node_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.node_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.node.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_group_id") {
                if let Some(val) = event.get("json.attributes.owner_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.attributes.owner.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.attributes.owner_group_name") {
                event.rename("json.attributes.owner_group_name", "jamf_compliance_reporter.log.attributes.owner.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.attributes.owner_user_id") {
                if let Some(val) = event.get("json.attributes.owner_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.attributes.owner_user_id".into(),
                message,
                })?;
                event.set("json.attributes.owner_user_id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.attributes.owner_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.id", json!(event.get("json.attributes.owner_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.attributes.owner_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.file_access_mode") };
                if _cond {
                // Painless script
                // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n"#))?;
                }
                // End nested pipeline: "pipeline_aue_unmount"
            }

            let _cond = { event.get_str("event.action") == Some("aue_fork") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_fork"
                if event.has_value("json.exec_chain_parent.uuid") {
                event.rename("json.exec_chain_parent.uuid", "jamf_compliance_reporter.log.exec_chain_parent.uuid")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.child_PID") {
                if let Some(val) = event.get("json.arguments.child_PID") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.child_PID".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.child.pid", converted)?;
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
                // End nested pipeline: "pipeline_aue_fork"
            }

            let _cond = { ["aue_getauid", "aue_lw_login", "aue_settimeofday"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
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
            }

            let _cond = { event.get_str("event.action") == Some("aue_listen") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_listen"
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
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
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
                // End nested pipeline: "pipeline_aue_listen"
            }

            let _cond = { event.get_str("event.action") == Some("aue_logout") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_logout"
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
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
                // End nested pipeline: "pipeline_aue_logout"
            }

            let _cond = { event.get_str("event.action") == Some("aue_pidfortask") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_pidfortask"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.pid") {
                if let Some(val) = event.get("json.arguments.pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.pid".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.pid", converted)?;
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.port") {
                if let Some(val) = event.get("json.arguments.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.port", converted)?;
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
                // End nested pipeline: "pipeline_aue_pidfortask"
            }

            let _cond = { event.get_str("event.action") == Some("aue_ptrace") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_ptrace"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.addr") {
                if let Some(val) = event.get("json.arguments.addr") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.addr".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.addr", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.data") {
                if let Some(val) = event.get("json.arguments.data") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.data".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.data", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.process") {
                if let Some(val) = event.get("json.arguments.process") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.process".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.process", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.request") {
                if let Some(val) = event.get("json.arguments.request") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.request".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.request", converted)?;
                }
                }
                Ok(())
                })();
                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"
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
                // End nested pipeline: "pipeline_aue_ptrace"
            }

            let _cond = { event.get_str("event.action") == Some("aue_setpriority") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_setpriority"
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.priority") {
                if let Some(val) = event.get("json.arguments.priority") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.priority".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.priority", converted)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.which") {
                if let Some(val) = event.get("json.arguments.which") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.which".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.which", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.who") {
                if let Some(val) = event.get("json.arguments.who") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.who".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.who", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_aue_setpriority"
            }

            let _cond = { event.get_str("event.action") == Some("aue_socketpair") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_socketpair"
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.domain") {
                if let Some(val) = event.get("json.arguments.domain") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.domain".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.domain", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.protocol") {
                if let Some(val) = event.get("json.arguments.protocol") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.protocol".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.protocol", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.arguments.type") {
                if let Some(val) = event.get("json.arguments.type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.type", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_aue_socketpair"
            }

            let _cond = { event.get_str("event.action") == Some("aue_taskforpid") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_taskforpid"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.target_port") {
                if let Some(val) = event.get("json.arguments.target_port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.target_port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.target.port", converted)?;
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.task_port") {
                if let Some(val) = event.get("json.arguments.task_port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.task_port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.arguments.task.port", converted)?;
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
                // Begin nested pipeline: "pipeline_process_object"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.audit_id") {
                if let Some(val) = event.get("json.process.audit_id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.audit_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.effective_group_id") {
                if let Some(val) = event.get("json.process.effective_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.effective_group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.effective.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.effective_group_name") {
                event.rename("json.process.effective_group_name", "jamf_compliance_reporter.log.process.effective.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.effective_user_id") {
                if let Some(val) = event.get("json.process.effective_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.effective_user_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.effective.user.id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.process.effective_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.effective.id", json!(event.get("json.process.effective_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.process.effective_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.effective.name", json!(event.get("json.process.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.process.effective_user_name") {
                event.rename("json.process.effective_user_name", "jamf_compliance_reporter.log.process.effective.user.name")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.effective.user.name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.effective.user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.group_id") {
                if let Some(val) = event.get("json.process.group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.group_name") {
                event.rename("json.process.group_name", "jamf_compliance_reporter.log.process.group.name")?;
                }
                let _cond = { event.has_value("json.process.process_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("process.hash.sha1", json!(event.get("json.process.process_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.process.process_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("json.process.process_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.process_id") {
                if let Some(val) = event.get("json.process.process_id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.process_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
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
                if event.has_value("json.process.process_name") {
                event.rename("json.process.process_name", "process.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.session_id") {
                if let Some(val) = event.get("json.process.session_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.session_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.session.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.addr") {
                if let Some(val) = event.get("json.process.terminal_id.addr") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.addr".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.addr", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.ip_address") {
                if let Some(val) = event.get("json.process.terminal_id.ip_address") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.ip_address".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.ip_address", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("json.process.terminal_id.ip_address");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.terminal_id.ip_address") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("jamf_compliance_reporter.log.process.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.port") {
                if let Some(val) = event.get("json.process.terminal_id.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.port", converted)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.type") {
                if let Some(val) = event.get("json.process.terminal_id.type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.type", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.user_id") {
                if let Some(val) = event.get("json.process.user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.user_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.user.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.user_name") {
                event.rename("json.process.user_name", "jamf_compliance_reporter.log.process.user.name")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.user.name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // End nested pipeline: "pipeline_process_object"
                // End nested pipeline: "pipeline_aue_taskforpid"
            }

            let _cond = { event.get_str("event.action") == Some("aue_wait4") };
            if _cond {
                // Begin nested pipeline: "pipeline_aue_wait4"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.arguments.pid") {
                if let Some(val) = event.get("json.arguments.pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.arguments.pid".into(),
                message,
                })?;
                event.set("process.pid", converted)?;
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
                // End nested pipeline: "pipeline_aue_wait4"
            }

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
