// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_x509` pipeline.
pub struct IndicatorX509;

impl Transform for IndicatorX509 {
    fn name(&self) -> &str {
        "indicator_x509"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_ingest._value") {
                    // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?$)
                    // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?$)
                    // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?$)
                    // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?SHA-?384'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha384}'\\]?$)
                    // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?SHA-?512'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha512}'\\]?$)
                    // Grok pattern: ^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:_tmp.serial_number}'\\]?$
                    // Grok pattern: ^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:_tmp.signature_algorithm}'\\]?$
                    // Grok pattern: ^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:_tmp.version_number}'\\]?$
                    // Grok pattern: ^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_after}'\\]?$
                    // Grok pattern: ^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_before}'\\]?$
                    // Grok pattern: ^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:_tmp.issuer}'\\]?$
                    // Grok pattern: ^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:_tmp.subject}'\\]?$
                    if !extract_first_match(
                        &[
                            cached_grok!("(?i:^\\[?x509-certificate:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?$)"),
                            cached_grok!("(?i:^\\[?x509-certificate:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?$)"),
                            cached_grok!("(?i:^\\[?x509-certificate:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?$)"),
                            cached_grok!("(?i:^\\[?x509-certificate:hashes\\.'?SHA-?384'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha384}'\\]?$)"),
                            cached_grok!("(?i:^\\[?x509-certificate:hashes\\.'?SHA-?512'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha512}'\\]?$)"),
                            cached_grok!("^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:_tmp.serial_number}'\\]?$"),
                            cached_grok!("^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:_tmp.signature_algorithm}'\\]?$"),
                            cached_grok!("^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:_tmp.version_number}'\\]?$"),
                            cached_grok!("^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_after}'\\]?$"),
                            cached_grok!("^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_before}'\\]?$"),
                            cached_grok!("^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:_tmp.issuer}'\\]?$"),
                            cached_grok!("^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:_tmp.subject}'\\]?$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.md5") };
            if _cond {
                event.append_unique("threat.indicator.file.hash.md5", json!(event.get("_tmp.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha1") };
            if _cond {
                event.append_unique("threat.indicator.file.hash.sha1", json!(event.get("_tmp.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha256") };
            if _cond {
                event.append_unique("threat.indicator.file.hash.sha256", json!(event.get("_tmp.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha384") };
            if _cond {
                event.append_unique("threat.indicator.file.hash.sha384", json!(event.get("_tmp.sha384").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha512") };
            if _cond {
                event.append_unique("threat.indicator.file.hash.sha512", json!(event.get("_tmp.sha512").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.serial_number") };
            if _cond {
                event.append_unique("threat.indicator.x509.serial_number", json!(event.get("_tmp.serial_number").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.signature_algorithm") };
            if _cond {
                event.append_unique("threat.indicator.x509.signature_algorithm", json!(event.get("_tmp.signature_algorithm").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.version_number") };
            if _cond {
                event.append_unique("threat.indicator.x509.version_number", json!(event.get("_tmp.version_number").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.distinguished_name", json!(event.get("_tmp.issuer").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.distinguished_name", json!(event.get("_tmp.subject").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("_tmp.issuer") {
                    for pair in cached_regex!("(?<!\\\\),").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.issuer".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("_tmp.issuer_fields.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("_tmp.issuer_fields.CN") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.common_name", json!(event.get("_tmp.issuer_fields.CN").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer_fields.C") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.country", json!(event.get("_tmp.issuer_fields.C").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer_fields.L") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.locality", json!(event.get("_tmp.issuer_fields.L").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer_fields.O") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.organization", json!(event.get("_tmp.issuer_fields.O").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer_fields.OU") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.organizational_unit", json!(event.get("_tmp.issuer_fields.OU").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer_fields.S") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.state_or_province", json!(event.get("_tmp.issuer_fields.S").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer_fields.ST") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.state_or_province", json!(event.get("_tmp.issuer_fields.ST").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.issuer_fields.P") };
            if _cond {
                event.append_unique("threat.indicator.x509.issuer.state_or_province", json!(event.get("_tmp.issuer_fields.P").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("_tmp.subject") {
                    for pair in cached_regex!("(?<!\\\\),").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.subject".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("_tmp.subject_fields.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("_tmp.subject_fields.CN") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.common_name", json!(event.get("_tmp.subject_fields.CN").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject_fields.C") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.country", json!(event.get("_tmp.subject_fields.C").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject_fields.L") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.locality", json!(event.get("_tmp.subject_fields.L").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject_fields.O") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.organization", json!(event.get("_tmp.subject_fields.O").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject_fields.OU") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.organizational_unit", json!(event.get("_tmp.subject_fields.OU").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject_fields.S") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.state_or_province", json!(event.get("_tmp.subject_fields.S").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject_fields.ST") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.state_or_province", json!(event.get("_tmp.subject_fields.ST").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.subject_fields.P") };
            if _cond {
                event.append_unique("threat.indicator.x509.subject.state_or_province", json!(event.get("_tmp.subject_fields.P").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("_tmp.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("_tmp.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("_tmp.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha384") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("_tmp.sha384").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.sha512") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("_tmp.sha512").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.not_after") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.not_after") {
                    match parse_date_out(&date_str, &["ISO8601", "yyyy-MM-dd HH:mm:ssz"], None, None) {
                        Some(parsed) => event.set("threat.indicator.x509.not_after", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.not_after".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.not_before") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.not_before") {
                    match parse_date_out(&date_str, &["ISO8601", "yyyy-MM-dd HH:mm:ssz"], None, None) {
                        Some(parsed) => event.set("threat.indicator.x509.not_before", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.not_before".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

                event.remove("_tmp");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
