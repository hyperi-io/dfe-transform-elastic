// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_alarm` pipeline.
pub struct PipelineAlarm;

impl Transform for PipelineAlarm {
    fn name(&self) -> &str {
        "pipeline_alarm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("message") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("message") {
                    let mut kv_gap = false;
                    for pair in kv_str.split("; ") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "message".into(),
                                split: "=".into(),
                            });
                        };
                        {
                            let key = key.trim_matches(|c: char| matches!(c, ' '));
                            let value = value.trim_matches(|c: char| matches!(c, ' '));
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.log_details_raw.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details_raw") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: def c = [:];\nctx.cisco_ise.log.log_details_raw.forEach((k, v) -> c[k.replace(' ', '_').toLowerCase()] = v);\nctx.cisco_ise.log.log_details_raw = c;
                rewrite_keys(event, &RewriteKeys::new("cisco_ise.log.log_details_raw".into(), "cisco_ise.log.log_details_raw".into(), vec![KeyRewriteStep::ReplaceChars(" ".into(), Some('_')), KeyRewriteStep::Lowercase]));
            }

            let _cond = { event.has_value("cisco_ise.log.log_details_raw.message") && event.get_str("event.action") == Some("Queue Link Error") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details_raw.message") {
                    // Grok pattern: ^(%{DATA} )?From %{DATA:source.address} To %{DATA:destination.address}$
                    if !cached_grok!("^(%{DATA} )?From %{DATA:source.address} To %{DATA:destination.address}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_alarm_queue_link_error_host_details")?;
                        event.append("error.message", json!(format!("{}: {} with \"{}\"", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string), event.get("message").map_or_else(String::new, template_to_string))))?;
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
                event.rename("cisco_ise.log.log_details_raw.cause", "cisco_ise.log.cause")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details_raw.server") {
                    event.rename("cisco_ise.log.log_details_raw.server", "server.address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details_raw.nad_address") {
                if let Some(val) = event.get("cisco_ise.log.log_details_raw.nad_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details_raw.nad_address".into(),
                            message,
                        })?;
                    event.set("_tmp.nad_ip", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.nad_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("_tmp.nad_ip").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("cisco_ise.log.log_details_raw.error_message", "cisco_ise.log.error_message")?;
                Ok(())
            })();

            let _cond = { event.has_value("source.address") && event.get_str("source.address") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("source.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.address") && event.get_str("destination.address") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("destination.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("server.address") && event.get_str("server.address") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("server.address").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details_raw.nas_ip_address") {
                    event.rename("cisco_ise.log.log_details_raw.nas_ip_address", "cisco_ise.log.nas_ip_address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.nas_ip_address") {
                if let Some(val) = event.get("cisco_ise.log.nas_ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.nas_ip_address".into(),
                            message,
                        })?;
                    event.set("_tmp.nas_ip", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.nas_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("_tmp.nas_ip").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details_raw.nas_identifier") {
                    event.rename("cisco_ise.log.log_details_raw.nas_identifier", "cisco_ise.log.nas_identifier")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details_raw.failure_reason") {
                    event.rename("cisco_ise.log.log_details_raw.failure_reason", "cisco_ise.log.failure_reason")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details_raw.network_device_name") {
                    event.rename("cisco_ise.log.log_details_raw.network_device_name", "cisco_ise.log.network_device_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details_raw.calling_station_id") {
                    event.rename("cisco_ise.log.log_details_raw.calling_station_id", "cisco_ise.log.calling_station_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.calling_station_id") {
                gsub_field(event, "cisco_ise.log.calling_station_id", "cisco_ise.log.calling_station_id", cached_regex!("[-:.]"), "-")?;
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.calling_station_id") {
                map_strings(event, "cisco_ise.log.calling_station_id", "cisco_ise.log.calling_station_id", str::to_uppercase)?;
            }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.calling_station_id") };
            if _cond {
                event.append_unique("server.mac", json!(event.get("cisco_ise.log.calling_station_id").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details_raw.network_device_ip") {
                if let Some(val) = event.get("cisco_ise.log.log_details_raw.network_device_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details_raw.network_device_ip".into(),
                            message,
                        })?;
                    event.set("_tmp.dev_ip", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.dev_ip") {
                    event.rename("_tmp.dev_ip", "cisco_ise.log.network_device_ip")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.network_device_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("cisco_ise.log.network_device_ip").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.set("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
