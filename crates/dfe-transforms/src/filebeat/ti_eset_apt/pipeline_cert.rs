// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_cert` pipeline.
pub struct PipelineCert;

impl Transform for PipelineCert {
    fn name(&self) -> &str {
        "pipeline_cert"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("eti._patterns").cloned();
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
                        if let Some(input) = event.get_string("_ingest._value") {
                        // Grok pattern: ^\\[?x509-certificate:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.serial_number}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.signature_algorithm}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.version_number}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_after}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_before}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.issuer.distinguished_name}'\\]?
                        // Grok pattern: ^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.subject.distinguished_name}'\\]?
                        if !extract_first_match(
                        &[
                        cached_grok!("^\\[?x509-certificate:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.serial_number}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.signature_algorithm}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.version_number}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_after}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_before}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.issuer.distinguished_name}'\\]?"),
                        cached_grok!("^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.subject.distinguished_name}'\\]?"),
                        ],
                        &input,
                        event,
                        )? {
                        return Err(TransformError::GrokNoMatch { value: input });
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
                    event.set("eti._patterns", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
            }

        let _cond = { event.has_value("threat.indicator.x509.not_after") };
        if _cond {
            if let Some(date_str) = event.get_as_string("threat.indicator.x509.not_after") {
                match parse_date_out(&date_str, &["ISO8601", "yyyy-MM-dd HH:mm:ssz"], None, None) {
                    Some(parsed) => event.set("threat.indicator.x509.not_after", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "threat.indicator.x509.not_after".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
        }

        let _cond = { event.has_value("threat.indicator.x509.not_before") };
        if _cond {
            if let Some(date_str) = event.get_as_string("threat.indicator.x509.not_before") {
                match parse_date_out(&date_str, &["ISO8601", "yyyy-MM-dd HH:mm:ssz"], None, None) {
                    Some(parsed) => event.set("threat.indicator.x509.not_before", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "threat.indicator.x509.not_before".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
        }

        let _cond = { event.has_value("threat.indicator.x509.issuer.distinguished_name") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(kv_str) = event.get_string("threat.indicator.x509.issuer.distinguished_name") {
                let mut kv_gap = false;
                for pair in cached_regex!("(?<!\\\\),").split(&kv_str).into_iter() {
                    if pair.is_empty() {
                        kv_gap = true;
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                        return Err(TransformError::KvValueSplit {
                            field: "threat.indicator.x509.issuer.distinguished_name".into(),
                            split: "=".into(),
                        });
                    };
                    {
                        let key = key.trim_matches(|c: char| matches!(c, ' '));
                        if !key.is_empty() {
                            kv_put(event, &format!("eti._issuer_fields.{}", key), value)?;
                        }
                    }
                }
            }
            Ok(())
        })();
        }

        let _cond = { event.has_value("eti._issuer_fields.CN") };
        if _cond {
            event.append("threat.indicator.x509.issuer.common_name", json!(event.get("eti._issuer_fields.CN").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._issuer_fields.C") };
        if _cond {
            event.append("threat.indicator.x509.issuer.country", json!(event.get("eti._issuer_fields.C").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._issuer_fields.L") };
        if _cond {
            event.append("threat.indicator.x509.issuer.locality", json!(event.get("eti._issuer_fields.L").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._issuer_fields.O") };
        if _cond {
            event.append("threat.indicator.x509.issuer.organization", json!(event.get("eti._issuer_fields.O").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._issuer_fields.OU") };
        if _cond {
            event.append("threat.indicator.x509.issuer.organizational_unit", json!(event.get("eti._issuer_fields.OU").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._issuer_fields.S") };
        if _cond {
            event.append("threat.indicator.x509.issuer.state_or_province", json!(event.get("eti._issuer_fields.S").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._issuer_fields.ST") };
        if _cond {
            event.append("threat.indicator.x509.issuer.state_or_province", json!(event.get("eti._issuer_fields.ST").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._issuer_fields.P") };
        if _cond {
            event.append("threat.indicator.x509.issuer.state_or_province", json!(event.get("eti._issuer_fields.P").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("threat.indicator.x509.subject.distinguished_name") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(kv_str) = event.get_string("threat.indicator.x509.subject.distinguished_name") {
                let mut kv_gap = false;
                for pair in cached_regex!("(?<!\\\\),").split(&kv_str).into_iter() {
                    if pair.is_empty() {
                        kv_gap = true;
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                        return Err(TransformError::KvValueSplit {
                            field: "threat.indicator.x509.subject.distinguished_name".into(),
                            split: "=".into(),
                        });
                    };
                    {
                        let key = key.trim_matches(|c: char| matches!(c, ' '));
                        if !key.is_empty() {
                            kv_put(event, &format!("eti._subject_fields.{}", key), value)?;
                        }
                    }
                }
            }
            Ok(())
        })();
        }

        let _cond = { event.has_value("eti._subject_fields.CN") };
        if _cond {
            event.append("threat.indicator.x509.subject.common_name", json!(event.get("eti._subject_fields.CN").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._subject_fields.C") };
        if _cond {
            event.append("threat.indicator.x509.subject.country", json!(event.get("eti._subject_fields.C").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._subject_fields.L") };
        if _cond {
            event.append("threat.indicator.x509.subject.locality", json!(event.get("eti._subject_fields.L").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._subject_fields.O") };
        if _cond {
            event.append("threat.indicator.x509.subject.organization", json!(event.get("eti._subject_fields.O").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._subject_fields.OU") };
        if _cond {
            event.append("threat.indicator.x509.subject.organizational_unit", json!(event.get("eti._subject_fields.OU").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._subject_fields.S") };
        if _cond {
            event.append("threat.indicator.x509.subject.state_or_province", json!(event.get("eti._subject_fields.S").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._subject_fields.ST") };
        if _cond {
            event.append("threat.indicator.x509.subject.state_or_province", json!(event.get("eti._subject_fields.ST").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("eti._subject_fields.P") };
        if _cond {
            event.append("threat.indicator.x509.subject.state_or_province", json!(event.get("eti._subject_fields.P").map_or_else(String::new, template_to_string)))?;
        }

        Ok(TransformResult::Continue)
    }
}
