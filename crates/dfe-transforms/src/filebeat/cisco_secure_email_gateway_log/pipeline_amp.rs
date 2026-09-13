// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_amp` pipeline.
pub struct PipelineAmp;

impl Transform for PipelineAmp {
    fn name(&self) -> &str {
        "pipeline_amp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^File reputation query initiating. %{GREEDYDATA:_tmp.new_message}$
                    // Grok pattern: ^Response received for file reputation query from (Cloud|Cache). %{GREEDYDATA:_tmp.new_message}$
                    // Grok pattern: ^File Analysis complete. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256}, Submit Timestamp: %{GREEDYDATA:_tmp.submit.timestamp}, Update Timestamp: %{GREEDYDATA:_tmp.update.timestamp}, Disposition: %{DATA:cisco_secure_email_gateway.log.disposition} Score: %{NUMBER:cisco_secure_email_gateway.log.score:long}, run_id: %{NUMBER:cisco_secure_email_gateway.log.run_id} Details: %{DATA:cisco_secure_email_gateway.log.details} Spyname:\\[%{GREEDYDATA:cisco_secure_email_gateway.log.spy_name}\\]$
                    // Grok pattern: ^(?i)File not uploaded for analysis.\\s+MID = %{NUMBER:email.message_id},? File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\],? File mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\],? Reason: %{GREEDYDATA:event.reason}$
                    // Grok pattern: ^File analysis upload skipped. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:_tmp.cisco_secure_email_gateway.log.remaining_details}\\]$
                    // Grok pattern: ^SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:cisco_secure_email_gateway.log.server_error_details}\\]$
                    // Grok pattern: ^Retrospective verdict received. %{GREEDYDATA:_tmp.new_message}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    if !extract_first_match_traced(
                        &[
                            cached_grok!("^File reputation query initiating. %{GREEDYDATA:_tmp.new_message}$"),
                            cached_grok!("^Response received for file reputation query from (Cloud|Cache). %{GREEDYDATA:_tmp.new_message}$"),
                            cached_grok!("^File Analysis complete. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256}, Submit Timestamp: %{GREEDYDATA:_tmp.submit.timestamp}, Update Timestamp: %{GREEDYDATA:_tmp.update.timestamp}, Disposition: %{DATA:cisco_secure_email_gateway.log.disposition} Score: %{NUMBER:cisco_secure_email_gateway.log.score:long}, run_id: %{NUMBER:cisco_secure_email_gateway.log.run_id} Details: %{DATA:cisco_secure_email_gateway.log.details} Spyname:\\[%{GREEDYDATA:cisco_secure_email_gateway.log.spy_name}\\]$"),
                            cached_grok!("^(?i)File not uploaded for analysis.\\s+MID = %{NUMBER:email.message_id},? File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\],? File mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\],? Reason: %{GREEDYDATA:event.reason}$"),
                            cached_grok!("^File analysis upload skipped. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:_tmp.cisco_secure_email_gateway.log.remaining_details}\\]$"),
                            cached_grok!("^SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:cisco_secure_email_gateway.log.server_error_details}\\]$"),
                            cached_grok!("^Retrospective verdict received. %{GREEDYDATA:_tmp.new_message}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            event.set("_tmp.grok_match_index", json!(event.get("_ingest._grok_match_index").map_or_else(String::new, template_to_string)))?;

            let _cond = { event.has_value("_tmp.new_message") && event.get_str("_tmp.grok_match_index") != Some("1") };
            if _cond {
                if let Some(kv_str) = event.get_string("_tmp.new_message") {
                    for pair in cached_regex!(",\\s*(?=(File Name|FileName|File Size|File Type|Reputation Score|Analysis Score|verdict_source|upload_action|Disposition|Malware|Verdict|Spyname|Timestamp|SHA256|sha256|MID)\\s*[=:])").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = ({ let parts = cached_regex!("\\s*=\\s*|:\\s*").splitn(&pair, 2); match (parts.first(), parts.get(1)) { (Some(k), Some(v)) => Some((k.clone(), v.clone())), _ => None } }) else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.new_message".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = &key[..];
                            if !key.is_empty() {
                                kv_put(event, key, value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.new_message") && event.get_str("_tmp.grok_match_index") == Some("1") };
            if _cond {
                if let Some(kv_str) = event.get_string("_tmp.new_message") {
                    for pair in cached_regex!(",\\s*(?=(File Name|FileName|File Size|File Type|Reputation Score|Analysis Score|verdict_source|upload_action|Disposition|Malware|Verdict|Spyname|Timestamp|SHA256|sha256|MID)\\s*=)").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = ({ let parts = cached_regex!("\\s*=\\s*").splitn(&pair, 2); match (parts.first(), parts.get(1)) { (Some(k), Some(v)) => Some((k.clone(), v.clone())), _ => None } }) else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.new_message".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = &key[..];
                            if !key.is_empty() {
                                kv_put(event, key, value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.cisco_secure_email_gateway.log.remaining_details") };
            if _cond {
                if let Some(input) = event.get_string("_tmp.cisco_secure_email_gateway.log.remaining_details") {
                    // Grok pattern: ^File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\] file mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\], upload priority\\[%{GREEDYDATA:cisco_secure_email_gateway.log.upload.priority}\\] not uploaded, re-tries\\[%{GREEDYDATA:cisco_secure_email_gateway.log.retries:long}\\], backoff\\[%{GREEDYDATA:cisco_secure_email_gateway.log.backoff:long}\\] %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                    if !cached_grok!("^File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\] file mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\], upload priority\\[%{GREEDYDATA:cisco_secure_email_gateway.log.upload.priority}\\] not uploaded, re-tries\\[%{GREEDYDATA:cisco_secure_email_gateway.log.retries:long}\\], backoff\\[%{GREEDYDATA:cisco_secure_email_gateway.log.backoff:long}\\] %{GREEDYDATA:cisco_secure_email_gateway.log.details}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

                if event.has_value("Timestamp") {
                    event.rename("Timestamp", "_tmp.submit.timestamp")?;
                }

            let _cond = { event.has_value("_tmp.submit.timestamp") && event.get_str("cisco_secure_email_gateway.log._tmp.submit.timestamp") != Some("0") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.submit.timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("cisco_secure_email_gateway.log.submit.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.submit.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_submit_timestamp_to_cisco_secure_email_gateway_log_submit_timestamp_dd83046a")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("_tmp.update.timestamp") && event.get_str("cisco_secure_email_gateway.log._tmp.update.timestamp") != Some("0") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.update.timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("cisco_secure_email_gateway.log.update.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.update.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_update_timestamp_to_cisco_secure_email_gateway_log_update_timestamp_5d6d9d79")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("File Name") {
                    event.rename("File Name", "email.attachments.file.name")?;
                }

                if event.has_value("MID") {
                    event.rename("MID", "email.message_id")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(event, "File Size", "File Size", cached_regex!("\\ bytes"), "")?;
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("File Size") {
                if let Some(val) = event.get("File Size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "File Size".into(),
                            message,
                        })?;
                    event.set("email.attachments.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_File_Size_to_email_attachments_file_size_1e170071")?;
                        if event.remove("File Size").is_none() {
                            return Err(TransformError::FieldNotFound { path: "File Size".into() });
                        }
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("File Type") {
                    event.rename("File Type", "email.content_type")?;
                }

                if event.has_value("FileName") {
                    event.rename("FileName", "email.attachments.file.name")?;
                }

                if event.has_value("Malware") {
                    event.rename("Malware", "cisco_secure_email_gateway.log.malware")?;
                }

                if event.has_value("Disposition") {
                    event.rename("Disposition", "cisco_secure_email_gateway.log.disposition")?;
                }

            if event.has_value("Analysis Score") {
                if let Some(val) = event.get("Analysis Score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "Analysis Score".into(),
                            message,
                        })?;
                    event.set("Analysis Score", converted)?;
                }
            }

                if event.has_value("Analysis Score") {
                    event.rename("Analysis Score", "cisco_secure_email_gateway.log.score")?;
                }

                if event.has_value("sha256") {
                    event.rename("sha256", "email.attachments.file.hash.sha256")?;
                }

                if event.has_value("upload_action") {
                    event.rename("upload_action", "cisco_secure_email_gateway.log.upload.action")?;
                }

                if event.has_value("Reputation Score") {
                    event.rename("Reputation Score", "cisco_secure_email_gateway.log.reputation_score")?;
                }

                if event.has_value("SHA256") {
                    event.rename("SHA256", "email.attachments.file.hash.sha256")?;
                }

                if event.has_value("Spyname") {
                    event.rename("Spyname", "cisco_secure_email_gateway.log.spy_name")?;
                }

                if event.has_value("Verdict") {
                    event.rename("Verdict", "cisco_secure_email_gateway.log.verdict")?;
                }

                if event.has_value("verdict_source") {
                    event.rename("verdict_source", "cisco_secure_email_gateway.log.verdict_source")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(event, "email.attachments.file.name", "email.attachments.file.name", cached_regex!("\\'"), "")?;
                Ok(())
            })();

            let _cond = { event.has_value("email.attachments.file.hash.sha256") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("email.attachments.file.hash.sha256").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                event.remove("_tmp");
                event.remove("File Size");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        event.remove("_ingest._grok_match_index");
        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
            event.remove("_ingest");
        }
        Ok(TransformResult::Continue)
    }
}
