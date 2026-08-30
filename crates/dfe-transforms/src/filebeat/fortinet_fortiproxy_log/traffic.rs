// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `traffic` pipeline.
pub struct Traffic;

impl Transform for Traffic {
    fn name(&self) -> &str {
        "traffic"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("_fields_.clientip") {
                if let Some(val) = event.get("_fields_.clientip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.clientip".into(),
                            message,
                        })?;
                    event.set("_fields_.clientip", converted)?;
                }
            }

            if event.has_value("_fields_.countapp") {
                if let Some(val) = event.get("_fields_.countapp") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countapp".into(),
                            message,
                        })?;
                    event.set("_fields_.countapp", converted)?;
                }
            }

            if event.has_value("_fields_.countav") {
                if let Some(val) = event.get("_fields_.countav") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countav".into(),
                            message,
                        })?;
                    event.set("_fields_.countav", converted)?;
                }
            }

            if event.has_value("_fields_.countcasb") {
                if let Some(val) = event.get("_fields_.countcasb") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countcasb".into(),
                            message,
                        })?;
                    event.set("_fields_.countcasb", converted)?;
                }
            }

            if event.has_value("_fields_.countcifs") {
                if let Some(val) = event.get("_fields_.countcifs") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countcifs".into(),
                            message,
                        })?;
                    event.set("_fields_.countcifs", converted)?;
                }
            }

            if event.has_value("_fields_.countdlp") {
                if let Some(val) = event.get("_fields_.countdlp") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countdlp".into(),
                            message,
                        })?;
                    event.set("_fields_.countdlp", converted)?;
                }
            }

            if event.has_value("_fields_.countdns") {
                if let Some(val) = event.get("_fields_.countdns") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countdns".into(),
                            message,
                        })?;
                    event.set("_fields_.countdns", converted)?;
                }
            }

            if event.has_value("_fields_.countemail") {
                if let Some(val) = event.get("_fields_.countemail") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countemail".into(),
                            message,
                        })?;
                    event.set("_fields_.countemail", converted)?;
                }
            }

            if event.has_value("_fields_.countff") {
                if let Some(val) = event.get("_fields_.countff") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countff".into(),
                            message,
                        })?;
                    event.set("_fields_.countff", converted)?;
                }
            }

            if event.has_value("_fields_.counticap") {
                if let Some(val) = event.get("_fields_.counticap") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.counticap".into(),
                            message,
                        })?;
                    event.set("_fields_.counticap", converted)?;
                }
            }

            if event.has_value("_fields_.countips") {
                if let Some(val) = event.get("_fields_.countips") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countips".into(),
                            message,
                        })?;
                    event.set("_fields_.countips", converted)?;
                }
            }

            if event.has_value("_fields_.countsctpf") {
                if let Some(val) = event.get("_fields_.countsctpf") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countsctpf".into(),
                            message,
                        })?;
                    event.set("_fields_.countsctpf", converted)?;
                }
            }

            if event.has_value("_fields_.countssh") {
                if let Some(val) = event.get("_fields_.countssh") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countssh".into(),
                            message,
                        })?;
                    event.set("_fields_.countssh", converted)?;
                }
            }

            if event.has_value("_fields_.countssl") {
                if let Some(val) = event.get("_fields_.countssl") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countssl".into(),
                            message,
                        })?;
                    event.set("_fields_.countssl", converted)?;
                }
            }

            if event.has_value("_fields_.countwaf") {
                if let Some(val) = event.get("_fields_.countwaf") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countwaf".into(),
                            message,
                        })?;
                    event.set("_fields_.countwaf", converted)?;
                }
            }

            if event.has_value("_fields_.countweb") {
                if let Some(val) = event.get("_fields_.countweb") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.countweb".into(),
                            message,
                        })?;
                    event.set("_fields_.countweb", converted)?;
                }
            }

            if event.has_value("_fields_.dstreputation") {
                if let Some(val) = event.get("_fields_.dstreputation") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.dstreputation".into(),
                            message,
                        })?;
                    event.set("_fields_.dstreputation", converted)?;
                }
            }

            if event.has_value("_fields_.dstserver") {
                if let Some(val) = event.get("_fields_.dstserver") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.dstserver".into(),
                            message,
                        })?;
                    event.set("_fields_.dstserver", converted)?;
                }
            }

            if event.has_value("_fields_.lanin") {
                if let Some(val) = event.get("_fields_.lanin") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.lanin".into(),
                            message,
                        })?;
                    event.set("_fields_.lanin", converted)?;
                }
            }

            if event.has_value("_fields_.lanout") {
                if let Some(val) = event.get("_fields_.lanout") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.lanout".into(),
                            message,
                        })?;
                    event.set("_fields_.lanout", converted)?;
                }
            }

            if event.has_value("_fields_.prefetch") {
                if let Some(val) = event.get("_fields_.prefetch") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.prefetch".into(),
                            message,
                        })?;
                    event.set("_fields_.prefetch", converted)?;
                }
            }

            if event.has_value("_fields_.rcvddelta") {
                if let Some(val) = event.get("_fields_.rcvddelta") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.rcvddelta".into(),
                            message,
                        })?;
                    event.set("_fields_.rcvddelta", converted)?;
                }
            }

            if event.has_value("_fields_.reqlength") {
                if let Some(val) = event.get("_fields_.reqlength") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.reqlength".into(),
                            message,
                        })?;
                    event.set("_fields_.reqlength", converted)?;
                }
            }

            if event.has_value("_fields_.reqtime") {
                if let Some(val) = event.get("_fields_.reqtime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.reqtime".into(),
                            message,
                        })?;
                    event.set("_fields_.reqtime", converted)?;
                }
            }

            if event.has_value("_fields_.respfinishtime") {
                if let Some(val) = event.get("_fields_.respfinishtime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.respfinishtime".into(),
                            message,
                        })?;
                    event.set("_fields_.respfinishtime", converted)?;
                }
            }

            if event.has_value("_fields_.resplength") {
                if let Some(val) = event.get("_fields_.resplength") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.resplength".into(),
                            message,
                        })?;
                    event.set("_fields_.resplength", converted)?;
                }
            }

            if event.has_value("_fields_.resptime") {
                if let Some(val) = event.get("_fields_.resptime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.resptime".into(),
                            message,
                        })?;
                    event.set("_fields_.resptime", converted)?;
                }
            }

            if event.has_value("_fields_.sentdelta") {
                if let Some(val) = event.get("_fields_.sentdelta") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.sentdelta".into(),
                            message,
                        })?;
                    event.set("_fields_.sentdelta", converted)?;
                }
            }

            if event.has_value("_fields_.sentpkt") {
                if let Some(val) = event.get("_fields_.sentpkt") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.sentpkt".into(),
                            message,
                        })?;
                    event.set("_fields_.sentpkt", converted)?;
                }
            }

            if event.has_value("_fields_.srcreputation") {
                if let Some(val) = event.get("_fields_.srcreputation") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.srcreputation".into(),
                            message,
                        })?;
                    event.set("_fields_.srcreputation", converted)?;
                }
            }

            if event.has_value("_fields_.srcserver") {
                if let Some(val) = event.get("_fields_.srcserver") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.srcserver".into(),
                            message,
                        })?;
                    event.set("_fields_.srcserver", converted)?;
                }
            }

            if event.has_value("_fields_.statuscode") {
                if let Some(val) = event.get("_fields_.statuscode") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.statuscode".into(),
                            message,
                        })?;
                    event.set("_fields_.statuscode", converted)?;
                }
            }

            if event.has_value("_fields_.tranip") {
                if let Some(val) = event.get("_fields_.tranip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.tranip".into(),
                            message,
                        })?;
                    event.set("_fields_.tranip", converted)?;
                }
            }

            if event.has_value("_fields_.tranport") {
                if let Some(val) = event.get("_fields_.tranport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.tranport".into(),
                            message,
                        })?;
                    event.set("_fields_.tranport", converted)?;
                }
            }

            if event.has_value("_fields_.transip") {
                if let Some(val) = event.get("_fields_.transip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.transip".into(),
                            message,
                        })?;
                    event.set("_fields_.transip", converted)?;
                }
            }

            if event.has_value("_fields_.transport") {
                if let Some(val) = event.get("_fields_.transport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_fields_.transport".into(),
                            message,
                        })?;
                    event.set("_fields_.transport", converted)?;
                }
            }

            if event.has_value("_fields_.dstmac") {
                gsub_field(event, "_fields_.dstmac", "_fields_.dstmac", cached_regex!(":"), "-")?;
            }

            if event.has_value("_fields_.dstmac") {
                map_strings(event, "_fields_.dstmac", "_fields_.dstmac", str::to_uppercase)?;
            }

            if event.has_value("_fields_.srcmac") {
                gsub_field(event, "_fields_.srcmac", "_fields_.srcmac", cached_regex!(":"), "-")?;
            }

            if event.has_value("_fields_.srcmac") {
                map_strings(event, "_fields_.srcmac", "_fields_.srcmac", str::to_uppercase)?;
            }

                if event.has_value("_fields_.dstmac") {
                    event.rename("_fields_.dstmac", "destination.mac")?;
                }

                if event.has_value("_fields_.dstname") {
                    event.rename("_fields_.dstname", "destination.address")?;
                }

                if event.has_value("_fields_.policyname") {
                    event.rename("_fields_.policyname", "rule.name")?;
                }

                if event.has_value("_fields_.reqlength") {
                    event.rename("_fields_.reqlength", "http.request.bytes")?;
                }

                if event.has_value("_fields_.resplength") {
                    event.rename("_fields_.resplength", "http.response.bytes")?;
                }

                if event.has_value("_fields_.scheme") {
                    event.rename("_fields_.scheme", "url.scheme")?;
                }

                if event.has_value("_fields_.sentpkt") {
                    event.rename("_fields_.sentpkt", "source.packets")?;
                }

                if event.has_value("_fields_.srcthreatfeed") {
                    event.rename("_fields_.srcthreatfeed", "threat.feed.name")?;
                }

                if event.has_value("_fields_.statuscode") {
                    event.rename("_fields_.statuscode", "http.response.status_code")?;
                }

                if event.has_value("_fields_.tranip") {
                    event.rename("_fields_.tranip", "destination.nat.ip")?;
                }

                if event.has_value("_fields_.tranport") {
                    event.rename("_fields_.tranport", "destination.nat.port")?;
                }

                if event.has_value("_fields_.transip") {
                    event.rename("_fields_.transip", "source.nat.ip")?;
                }

                if event.has_value("_fields_.transport") {
                    event.rename("_fields_.transport", "source.nat.port")?;
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
