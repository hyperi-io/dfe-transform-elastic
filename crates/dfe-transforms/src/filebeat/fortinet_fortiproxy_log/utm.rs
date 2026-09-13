// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `utm` pipeline.
pub struct Utm;

impl Transform for Utm {
    fn name(&self) -> &str {
        "utm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("_fields_.botnetip") {
                if let Some(val) = event.get("_fields_.botnetip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.botnetip".into(),
                            message,
                        })?;
                    event.set("_fields_.botnetip", converted)?;
                }
            }

            if event.has_value("_fields_.c-bytes") {
                if let Some(val) = event.get("_fields_.c-bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.c-bytes".into(),
                            message,
                        })?;
                    event.set("_fields_.c-bytes", converted)?;
                }
            }

            if event.has_value("_fields_.c-ggsn") {
                if let Some(val) = event.get("_fields_.c-ggsn") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.c-ggsn".into(),
                            message,
                        })?;
                    event.set("_fields_.c-ggsn", converted)?;
                }
            }

            if event.has_value("_fields_.c-gsn") {
                if let Some(val) = event.get("_fields_.c-gsn") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.c-gsn".into(),
                            message,
                        })?;
                    event.set("_fields_.c-gsn", converted)?;
                }
            }

            if event.has_value("_fields_.c-pkts") {
                if let Some(val) = event.get("_fields_.c-pkts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.c-pkts".into(),
                            message,
                        })?;
                    event.set("_fields_.c-pkts", converted)?;
                }
            }

            if event.has_value("_fields_.c-sgsn") {
                if let Some(val) = event.get("_fields_.c-sgsn") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.c-sgsn".into(),
                            message,
                        })?;
                    event.set("_fields_.c-sgsn", converted)?;
                }
            }

            if event.has_value("_fields_.cfseidaddr") {
                if let Some(val) = event.get("_fields_.cfseidaddr") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cfseidaddr".into(),
                            message,
                        })?;
                    event.set("_fields_.cfseidaddr", converted)?;
                }
            }

            if event.has_value("_fields_.cggsn6") {
                if let Some(val) = event.get("_fields_.cggsn6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cggsn6".into(),
                            message,
                        })?;
                    event.set("_fields_.cggsn6", converted)?;
                }
            }

            if event.has_value("_fields_.cgsn6") {
                if let Some(val) = event.get("_fields_.cgsn6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cgsn6".into(),
                            message,
                        })?;
                    event.set("_fields_.cgsn6", converted)?;
                }
            }

            if event.has_value("_fields_.clashtunnelidx") {
                if let Some(val) = event.get("_fields_.clashtunnelidx") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.clashtunnelidx".into(),
                            message,
                        })?;
                    event.set("_fields_.clashtunnelidx", converted)?;
                }
            }

            if event.has_value("_fields_.cpaddr") {
                if let Some(val) = event.get("_fields_.cpaddr") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpaddr".into(),
                            message,
                        })?;
                    event.set("_fields_.cpaddr", converted)?;
                }
            }

            if event.has_value("_fields_.cpaddr6") {
                if let Some(val) = event.get("_fields_.cpaddr6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpaddr6".into(),
                            message,
                        })?;
                    event.set("_fields_.cpaddr6", converted)?;
                }
            }

            if event.has_value("_fields_.cpdladdr") {
                if let Some(val) = event.get("_fields_.cpdladdr") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpdladdr".into(),
                            message,
                        })?;
                    event.set("_fields_.cpdladdr", converted)?;
                }
            }

            if event.has_value("_fields_.cpdladdr6") {
                if let Some(val) = event.get("_fields_.cpdladdr6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpdladdr6".into(),
                            message,
                        })?;
                    event.set("_fields_.cpdladdr6", converted)?;
                }
            }

            if event.has_value("_fields_.cpdlisraddr") {
                if let Some(val) = event.get("_fields_.cpdlisraddr") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpdlisraddr".into(),
                            message,
                        })?;
                    event.set("_fields_.cpdlisraddr", converted)?;
                }
            }

            if event.has_value("_fields_.cpdlisraddr6") {
                if let Some(val) = event.get("_fields_.cpdlisraddr6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpdlisraddr6".into(),
                            message,
                        })?;
                    event.set("_fields_.cpdlisraddr6", converted)?;
                }
            }

            if event.has_value("_fields_.cpuladdr") {
                if let Some(val) = event.get("_fields_.cpuladdr") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpuladdr".into(),
                            message,
                        })?;
                    event.set("_fields_.cpuladdr", converted)?;
                }
            }

            if event.has_value("_fields_.cpuladdr6") {
                if let Some(val) = event.get("_fields_.cpuladdr6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.cpuladdr6".into(),
                            message,
                        })?;
                    event.set("_fields_.cpuladdr6", converted)?;
                }
            }

            if event.has_value("_fields_.csgsn6") {
                if let Some(val) = event.get("_fields_.csgsn6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.csgsn6".into(),
                            message,
                        })?;
                    event.set("_fields_.csgsn6", converted)?;
                }
            }

            if event.has_value("_fields_.dlpfilteridx") {
                if let Some(val) = event.get("_fields_.dlpfilteridx") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.dlpfilteridx".into(),
                            message,
                        })?;
                    event.set("_fields_.dlpfilteridx", converted)?;
                }
            }

            if event.has_value("_fields_.domainfilteridx") {
                if let Some(val) = event.get("_fields_.domainfilteridx") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.domainfilteridx".into(),
                            message,
                        })?;
                    event.set("_fields_.domainfilteridx", converted)?;
                }
            }

            if event.has_value("_fields_.end-usr-address") {
                if let Some(val) = event.get("_fields_.end-usr-address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.end-usr-address".into(),
                            message,
                        })?;
                    event.set("_fields_.end-usr-address", converted)?;
                }
            }

            if event.has_value("_fields_.endusraddress6") {
                if let Some(val) = event.get("_fields_.endusraddress6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.endusraddress6".into(),
                            message,
                        })?;
                    event.set("_fields_.endusraddress6", converted)?;
                }
            }

            if event.has_value("_fields_.epoch") {
                if let Some(val) = event.get("_fields_.epoch") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.epoch".into(),
                            message,
                        })?;
                    event.set("_fields_.epoch", converted)?;
                }
            }

            if event.has_value("_fields_.filteridx") {
                if let Some(val) = event.get("_fields_.filteridx") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.filteridx".into(),
                            message,
                        })?;
                    event.set("_fields_.filteridx", converted)?;
                }
            }

            if event.has_value("_fields_.from4") {
                if let Some(val) = event.get("_fields_.from4") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.from4".into(),
                            message,
                        })?;
                    event.set("_fields_.from4", converted)?;
                }
            }

            if event.has_value("_fields_.from6") {
                if let Some(val) = event.get("_fields_.from6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.from6".into(),
                            message,
                        })?;
                    event.set("_fields_.from6", converted)?;
                }
            }

            if event.has_value("_fields_.ietype") {
                if let Some(val) = event.get("_fields_.ietype") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ietype".into(),
                            message,
                        })?;
                    event.set("_fields_.ietype", converted)?;
                }
            }

            if event.has_value("_fields_.incidentserialno") {
                if let Some(val) = event.get("_fields_.incidentserialno") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.incidentserialno".into(),
                            message,
                        })?;
                    event.set("_fields_.incidentserialno", converted)?;
                }
            }

            if event.has_value("_fields_.infectedfilelevel") {
                if let Some(val) = event.get("_fields_.infectedfilelevel") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.infectedfilelevel".into(),
                            message,
                        })?;
                    event.set("_fields_.infectedfilelevel", converted)?;
                }
            }

            if event.has_value("_fields_.infectedfilesize") {
                if let Some(val) = event.get("_fields_.infectedfilesize") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.infectedfilesize".into(),
                            message,
                        })?;
                    event.set("_fields_.infectedfilesize", converted)?;
                }
            }

            if event.has_value("_fields_.keysize") {
                if let Some(val) = event.get("_fields_.keysize") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.keysize".into(),
                            message,
                        })?;
                    event.set("_fields_.keysize", converted)?;
                }
            }

            if event.has_value("_fields_.linked-nsapi") {
                if let Some(val) = event.get("_fields_.linked-nsapi") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.linked-nsapi".into(),
                            message,
                        })?;
                    event.set("_fields_.linked-nsapi", converted)?;
                }
            }

            if event.has_value("_fields_.msg-type") {
                if let Some(val) = event.get("_fields_.msg-type") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.msg-type".into(),
                            message,
                        })?;
                    event.set("_fields_.msg-type", converted)?;
                }
            }

            if event.has_value("_fields_.nsapi") {
                if let Some(val) = event.get("_fields_.nsapi") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.nsapi".into(),
                            message,
                        })?;
                    event.set("_fields_.nsapi", converted)?;
                }
            }

            if event.has_value("_fields_.ocrlog") {
                if let Some(val) = event.get("_fields_.ocrlog") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ocrlog".into(),
                            message,
                        })?;
                    event.set("_fields_.ocrlog", converted)?;
                }
            }

            if event.has_value("_fields_.qtypeval") {
                if let Some(val) = event.get("_fields_.qtypeval") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.qtypeval".into(),
                            message,
                        })?;
                    event.set("_fields_.qtypeval", converted)?;
                }
            }

            if event.has_value("_fields_.quotamax") {
                if let Some(val) = event.get("_fields_.quotamax") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.quotamax".into(),
                            message,
                        })?;
                    event.set("_fields_.quotamax", converted)?;
                }
            }

            if event.has_value("_fields_.quotaused") {
                if let Some(val) = event.get("_fields_.quotaused") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.quotaused".into(),
                            message,
                        })?;
                    event.set("_fields_.quotaused", converted)?;
                }
            }

            if event.has_value("_fields_.rcode") {
                if let Some(val) = event.get("_fields_.rcode") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.rcode".into(),
                            message,
                        })?;
                    event.set("_fields_.rcode", converted)?;
                }
            }

            if event.has_value("_fields_.seqnum") {
                if let Some(val) = event.get("_fields_.seqnum") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.seqnum".into(),
                            message,
                        })?;
                    event.set("_fields_.seqnum", converted)?;
                }
            }

            if event.has_value("_fields_.timeoutdelete") {
                if let Some(val) = event.get("_fields_.timeoutdelete") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.timeoutdelete".into(),
                            message,
                        })?;
                    event.set("_fields_.timeoutdelete", converted)?;
                }
            }

            if event.has_value("_fields_.to4") {
                if let Some(val) = event.get("_fields_.to4") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.to4".into(),
                            message,
                        })?;
                    event.set("_fields_.to4", converted)?;
                }
            }

            if event.has_value("_fields_.to6") {
                if let Some(val) = event.get("_fields_.to6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.to6".into(),
                            message,
                        })?;
                    event.set("_fields_.to6", converted)?;
                }
            }

            if event.has_value("_fields_.trueclntip") {
                if let Some(val) = event.get("_fields_.trueclntip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.trueclntip".into(),
                            message,
                        })?;
                    event.set("_fields_.trueclntip", converted)?;
                }
            }

            if event.has_value("_fields_.tunnel-idx") {
                if let Some(val) = event.get("_fields_.tunnel-idx") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.tunnel-idx".into(),
                            message,
                        })?;
                    event.set("_fields_.tunnel-idx", converted)?;
                }
            }

            if event.has_value("_fields_.u-bytes") {
                if let Some(val) = event.get("_fields_.u-bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.u-bytes".into(),
                            message,
                        })?;
                    event.set("_fields_.u-bytes", converted)?;
                }
            }

            if event.has_value("_fields_.u-ggsn") {
                if let Some(val) = event.get("_fields_.u-ggsn") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.u-ggsn".into(),
                            message,
                        })?;
                    event.set("_fields_.u-ggsn", converted)?;
                }
            }

            if event.has_value("_fields_.u-gsn") {
                if let Some(val) = event.get("_fields_.u-gsn") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.u-gsn".into(),
                            message,
                        })?;
                    event.set("_fields_.u-gsn", converted)?;
                }
            }

            if event.has_value("_fields_.u-pkts") {
                if let Some(val) = event.get("_fields_.u-pkts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.u-pkts".into(),
                            message,
                        })?;
                    event.set("_fields_.u-pkts", converted)?;
                }
            }

            if event.has_value("_fields_.u-sgsn") {
                if let Some(val) = event.get("_fields_.u-sgsn") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.u-sgsn".into(),
                            message,
                        })?;
                    event.set("_fields_.u-sgsn", converted)?;
                }
            }

            if event.has_value("_fields_.ufseidaddr") {
                if let Some(val) = event.get("_fields_.ufseidaddr") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ufseidaddr".into(),
                            message,
                        })?;
                    event.set("_fields_.ufseidaddr", converted)?;
                }
            }

            if event.has_value("_fields_.uggsn6") {
                if let Some(val) = event.get("_fields_.uggsn6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.uggsn6".into(),
                            message,
                        })?;
                    event.set("_fields_.uggsn6", converted)?;
                }
            }

            if event.has_value("_fields_.ugsn6") {
                if let Some(val) = event.get("_fields_.ugsn6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ugsn6".into(),
                            message,
                        })?;
                    event.set("_fields_.ugsn6", converted)?;
                }
            }

            if event.has_value("_fields_.ulimcc") {
                if let Some(val) = event.get("_fields_.ulimcc") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ulimcc".into(),
                            message,
                        })?;
                    event.set("_fields_.ulimcc", converted)?;
                }
            }

            if event.has_value("_fields_.ulimnc") {
                if let Some(val) = event.get("_fields_.ulimnc") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.ulimnc".into(),
                            message,
                        })?;
                    event.set("_fields_.ulimnc", converted)?;
                }
            }

            if event.has_value("_fields_.urlfilteridx") {
                if let Some(val) = event.get("_fields_.urlfilteridx") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.urlfilteridx".into(),
                            message,
                        })?;
                    event.set("_fields_.urlfilteridx", converted)?;
                }
            }

            if event.has_value("_fields_.usgsn6") {
                if let Some(val) = event.get("_fields_.usgsn6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.usgsn6".into(),
                            message,
                        })?;
                    event.set("_fields_.usgsn6", converted)?;
                }
            }

            if event.has_value("_fields_.violatescore") {
                if let Some(val) = event.get("_fields_.violatescore") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.violatescore".into(),
                            message,
                        })?;
                    event.set("_fields_.violatescore", converted)?;
                }
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
