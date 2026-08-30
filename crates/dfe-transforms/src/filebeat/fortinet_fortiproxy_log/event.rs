// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `event` pipeline.
pub struct Event;

impl Transform for Event {
    fn name(&self) -> &str {
        "event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("_fields_.advpnsc") {
                if let Some(val) = event.get("_fields_.advpnsc") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.advpnsc".into(),
                            message,
                        })?;
                    event.set("_fields_.advpnsc", converted)?;
                }
            }

            if event.has_value("_fields_.assigned") {
                if let Some(val) = event.get("_fields_.assigned") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.assigned".into(),
                            message,
                        })?;
                    event.set("_fields_.assigned", converted)?;
                }
            }

            if event.has_value("_fields_.assignip") {
                if let Some(val) = event.get("_fields_.assignip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.assignip".into(),
                            message,
                        })?;
                    event.set("_fields_.assignip", converted)?;
                }
            }

            if event.has_value("_fields_.audittime") {
                if let Some(val) = event.get("_fields_.audittime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.audittime".into(),
                            message,
                        })?;
                    event.set("_fields_.audittime", converted)?;
                }
            }

            if event.has_value("_fields_.category") {
                if let Some(val) = event.get("_fields_.category") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.category".into(),
                            message,
                        })?;
                    event.set("_fields_.category", converted)?;
                }
            }

            if event.has_value("_fields_.core") {
                if let Some(val) = event.get("_fields_.core") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.core".into(),
                            message,
                        })?;
                    event.set("_fields_.core", converted)?;
                }
            }

            if event.has_value("_fields_.cpu") {
                if let Some(val) = event.get("_fields_.cpu") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpu".into(),
                            message,
                        })?;
                    event.set("_fields_.cpu", converted)?;
                }
            }

            if event.has_value("_fields_.criticalcount") {
                if let Some(val) = event.get("_fields_.criticalcount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.criticalcount".into(),
                            message,
                        })?;
                    event.set("_fields_.criticalcount", converted)?;
                }
            }

            if event.has_value("_fields_.ddnsserver") {
                if let Some(val) = event.get("_fields_.ddnsserver") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ddnsserver".into(),
                            message,
                        })?;
                    event.set("_fields_.ddnsserver", converted)?;
                }
            }

            if event.has_value("_fields_.disk") {
                if let Some(val) = event.get("_fields_.disk") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.disk".into(),
                            message,
                        })?;
                    event.set("_fields_.disk", converted)?;
                }
            }

            if event.has_value("_fields_.disklograte") {
                if let Some(val) = event.get("_fields_.disklograte") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.disklograte".into(),
                            message,
                        })?;
                    event.set("_fields_.disklograte", converted)?;
                }
            }

            if event.has_value("_fields_.domainctrlauthstate") {
                if let Some(val) = event.get("_fields_.domainctrlauthstate") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.domainctrlauthstate".into(),
                            message,
                        })?;
                    event.set("_fields_.domainctrlauthstate", converted)?;
                }
            }

            if event.has_value("_fields_.domainctrlauthtype") {
                if let Some(val) = event.get("_fields_.domainctrlauthtype") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.domainctrlauthtype".into(),
                            message,
                        })?;
                    event.set("_fields_.domainctrlauthtype", converted)?;
                }
            }

            if event.has_value("_fields_.domainctrlip") {
                if let Some(val) = event.get("_fields_.domainctrlip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.domainctrlip".into(),
                            message,
                        })?;
                    event.set("_fields_.domainctrlip", converted)?;
                }
            }

            if event.has_value("_fields_.domainctrlprotocoltype") {
                if let Some(val) = event.get("_fields_.domainctrlprotocoltype") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.domainctrlprotocoltype".into(),
                            message,
                        })?;
                    event.set("_fields_.domainctrlprotocoltype", converted)?;
                }
            }

            if event.has_value("_fields_.fams_pause") {
                if let Some(val) = event.get("_fields_.fams_pause") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.fams_pause".into(),
                            message,
                        })?;
                    event.set("_fields_.fams_pause", converted)?;
                }
            }

            if event.has_value("_fields_.fazlograte") {
                if let Some(val) = event.get("_fields_.fazlograte") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.fazlograte".into(),
                            message,
                        })?;
                    event.set("_fields_.fazlograte", converted)?;
                }
            }

            if event.has_value("_fields_.freediskstorage") {
                if let Some(val) = event.get("_fields_.freediskstorage") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.freediskstorage".into(),
                            message,
                        })?;
                    event.set("_fields_.freediskstorage", converted)?;
                }
            }

            if event.has_value("_fields_.from_vcluster") {
                if let Some(val) = event.get("_fields_.from_vcluster") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.from_vcluster".into(),
                            message,
                        })?;
                    event.set("_fields_.from_vcluster", converted)?;
                }
            }

            if event.has_value("_fields_.gateway") {
                if let Some(val) = event.get("_fields_.gateway") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.gateway".into(),
                            message,
                        })?;
                    event.set("_fields_.gateway", converted)?;
                }
            }

            if event.has_value("_fields_.ha-prio") {
                if let Some(val) = event.get("_fields_.ha-prio") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ha-prio".into(),
                            message,
                        })?;
                    event.set("_fields_.ha-prio", converted)?;
                }
            }

            if event.has_value("_fields_.ha_group") {
                if let Some(val) = event.get("_fields_.ha_group") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ha_group".into(),
                            message,
                        })?;
                    event.set("_fields_.ha_group", converted)?;
                }
            }

            if event.has_value("_fields_.highcount") {
                if let Some(val) = event.get("_fields_.highcount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.highcount".into(),
                            message,
                        })?;
                    event.set("_fields_.highcount", converted)?;
                }
            }

            if event.has_value("_fields_.httpcode") {
                if let Some(val) = event.get("_fields_.httpcode") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.httpcode".into(),
                            message,
                        })?;
                    event.set("_fields_.httpcode", converted)?;
                }
            }

            if event.has_value("_fields_.ip") {
                if let Some(val) = event.get("_fields_.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ip".into(),
                            message,
                        })?;
                    event.set("_fields_.ip", converted)?;
                }
            }

            if event.has_value("_fields_.lease") {
                if let Some(val) = event.get("_fields_.lease") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.lease".into(),
                            message,
                        })?;
                    event.set("_fields_.lease", converted)?;
                }
            }

            if event.has_value("_fields_.limit") {
                if let Some(val) = event.get("_fields_.limit") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.limit".into(),
                            message,
                        })?;
                    event.set("_fields_.limit", converted)?;
                }
            }

            if event.has_value("_fields_.local") {
                if let Some(val) = event.get("_fields_.local") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.local".into(),
                            message,
                        })?;
                    event.set("_fields_.local", converted)?;
                }
            }

            if event.has_value("_fields_.localdevcount") {
                if let Some(val) = event.get("_fields_.localdevcount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.localdevcount".into(),
                            message,
                        })?;
                    event.set("_fields_.localdevcount", converted)?;
                }
            }

            if event.has_value("_fields_.locip") {
                if let Some(val) = event.get("_fields_.locip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.locip".into(),
                            message,
                        })?;
                    event.set("_fields_.locip", converted)?;
                }
            }

            if event.has_value("_fields_.locport") {
                if let Some(val) = event.get("_fields_.locport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.locport".into(),
                            message,
                        })?;
                    event.set("_fields_.locport", converted)?;
                }
            }

            if event.has_value("_fields_.lowcount") {
                if let Some(val) = event.get("_fields_.lowcount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.lowcount".into(),
                            message,
                        })?;
                    event.set("_fields_.lowcount", converted)?;
                }
            }

            if event.has_value("_fields_.mediumcount") {
                if let Some(val) = event.get("_fields_.mediumcount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.mediumcount".into(),
                            message,
                        })?;
                    event.set("_fields_.mediumcount", converted)?;
                }
            }

            if event.has_value("_fields_.mem") {
                if let Some(val) = event.get("_fields_.mem") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.mem".into(),
                            message,
                        })?;
                    event.set("_fields_.mem", converted)?;
                }
            }

            if event.has_value("_fields_.mtu") {
                if let Some(val) = event.get("_fields_.mtu") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.mtu".into(),
                            message,
                        })?;
                    event.set("_fields_.mtu", converted)?;
                }
            }

            if event.has_value("_fields_.newchannel") {
                if let Some(val) = event.get("_fields_.newchannel") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.newchannel".into(),
                            message,
                        })?;
                    event.set("_fields_.newchannel", converted)?;
                }
            }

            if event.has_value("_fields_.newslot") {
                if let Some(val) = event.get("_fields_.newslot") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.newslot".into(),
                            message,
                        })?;
                    event.set("_fields_.newslot", converted)?;
                }
            }

            if event.has_value("_fields_.nextstat") {
                if let Some(val) = event.get("_fields_.nextstat") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.nextstat".into(),
                            message,
                        })?;
                    event.set("_fields_.nextstat", converted)?;
                }
            }

            if event.has_value("_fields_.oldchannel") {
                if let Some(val) = event.get("_fields_.oldchannel") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.oldchannel".into(),
                            message,
                        })?;
                    event.set("_fields_.oldchannel", converted)?;
                }
            }

            if event.has_value("_fields_.oldslot") {
                if let Some(val) = event.get("_fields_.oldslot") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.oldslot".into(),
                            message,
                        })?;
                    event.set("_fields_.oldslot", converted)?;
                }
            }

            if event.has_value("_fields_.passedcount") {
                if let Some(val) = event.get("_fields_.passedcount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.passedcount".into(),
                            message,
                        })?;
                    event.set("_fields_.passedcount", converted)?;
                }
            }

            if event.has_value("_fields_.pid") {
                if let Some(val) = event.get("_fields_.pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.pid".into(),
                            message,
                        })?;
                    event.set("_fields_.pid", converted)?;
                }
            }

            if event.has_value("_fields_.port") {
                if let Some(val) = event.get("_fields_.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.port".into(),
                            message,
                        })?;
                    event.set("_fields_.port", converted)?;
                }
            }

            if event.has_value("_fields_.processtime") {
                if let Some(val) = event.get("_fields_.processtime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.processtime".into(),
                            message,
                        })?;
                    event.set("_fields_.processtime", converted)?;
                }
            }

            if event.has_value("_fields_.remip") {
                if let Some(val) = event.get("_fields_.remip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.remip".into(),
                            message,
                        })?;
                    event.set("_fields_.remip", converted)?;
                }
            }

            if event.has_value("_fields_.remote") {
                if let Some(val) = event.get("_fields_.remote") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.remote".into(),
                            message,
                        })?;
                    event.set("_fields_.remote", converted)?;
                }
            }

            if event.has_value("_fields_.remport") {
                if let Some(val) = event.get("_fields_.remport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.remport".into(),
                            message,
                        })?;
                    event.set("_fields_.remport", converted)?;
                }
            }

            if event.has_value("_fields_.scantime") {
                if let Some(val) = event.get("_fields_.scantime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.scantime".into(),
                            message,
                        })?;
                    event.set("_fields_.scantime", converted)?;
                }
            }

            if event.has_value("_fields_.serial") {
                if let Some(val) = event.get("_fields_.serial") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.serial".into(),
                            message,
                        })?;
                    event.set("_fields_.serial", converted)?;
                }
            }

            if event.has_value("_fields_.setuprate") {
                if let Some(val) = event.get("_fields_.setuprate") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.setuprate".into(),
                            message,
                        })?;
                    event.set("_fields_.setuprate", converted)?;
                }
            }

            if event.has_value("_fields_.slot") {
                if let Some(val) = event.get("_fields_.slot") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.slot".into(),
                            message,
                        })?;
                    event.set("_fields_.slot", converted)?;
                }
            }

            if event.has_value("_fields_.stage") {
                if let Some(val) = event.get("_fields_.stage") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.stage".into(),
                            message,
                        })?;
                    event.set("_fields_.stage", converted)?;
                }
            }

            if event.has_value("_fields_.sysuptime") {
                if let Some(val) = event.get("_fields_.sysuptime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.sysuptime".into(),
                            message,
                        })?;
                    event.set("_fields_.sysuptime", converted)?;
                }
            }

            if event.has_value("_fields_.to_vcluster") {
                if let Some(val) = event.get("_fields_.to_vcluster") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.to_vcluster".into(),
                            message,
                        })?;
                    event.set("_fields_.to_vcluster", converted)?;
                }
            }

            if event.has_value("_fields_.total") {
                if let Some(val) = event.get("_fields_.total") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.total".into(),
                            message,
                        })?;
                    event.set("_fields_.total", converted)?;
                }
            }

            if event.has_value("_fields_.totalsession") {
                if let Some(val) = event.get("_fields_.totalsession") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.totalsession".into(),
                            message,
                        })?;
                    event.set("_fields_.totalsession", converted)?;
                }
            }

            if event.has_value("_fields_.tunnelip") {
                if let Some(val) = event.get("_fields_.tunnelip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.tunnelip".into(),
                            message,
                        })?;
                    event.set("_fields_.tunnelip", converted)?;
                }
            }

            if event.has_value("_fields_.unit") {
                if let Some(val) = event.get("_fields_.unit") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.unit".into(),
                            message,
                        })?;
                    event.set("_fields_.unit", converted)?;
                }
            }

            if event.has_value("_fields_.used") {
                if let Some(val) = event.get("_fields_.used") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.used".into(),
                            message,
                        })?;
                    event.set("_fields_.used", converted)?;
                }
            }

            if event.has_value("_fields_.used_for_type") {
                if let Some(val) = event.get("_fields_.used_for_type") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.used_for_type".into(),
                            message,
                        })?;
                    event.set("_fields_.used_for_type", converted)?;
                }
            }

            if event.has_value("_fields_.vcluster") {
                if let Some(val) = event.get("_fields_.vcluster") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.vcluster".into(),
                            message,
                        })?;
                    event.set("_fields_.vcluster", converted)?;
                }
            }

            if event.has_value("_fields_.vcluster_member") {
                if let Some(val) = event.get("_fields_.vcluster_member") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.vcluster_member".into(),
                            message,
                        })?;
                    event.set("_fields_.vcluster_member", converted)?;
                }
            }

            if event.has_value("_fields_.wscode") {
                if let Some(val) = event.get("_fields_.wscode") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.wscode".into(),
                            message,
                        })?;
                    event.set("_fields_.wscode", converted)?;
                }
            }

                if event.has_value("_fields_.dst_int") {
                    event.rename("_fields_.dst_int", "observer.egress.interface.name")?;
                }

                if event.has_value("_fields_.logdesc") {
                    event.rename("_fields_.logdesc", "rule.description")?;
                }

                if event.has_value("_fields_.reason") {
                    event.rename("_fields_.reason", "event.reason")?;
                }

                if event.has_value("_fields_.src_int") {
                    event.rename("_fields_.src_int", "observer.ingress.interface.name")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
