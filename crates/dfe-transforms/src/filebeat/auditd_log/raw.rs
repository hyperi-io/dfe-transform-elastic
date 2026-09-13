// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `raw` pipeline.
pub struct Raw;

impl Transform for Raw {
    fn name(&self) -> &str {
        "raw"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = { !event.has_value("auditd.log.record_type") && event.has_value("event.original") && event.get("event.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(" old auid=")), serde_json::Value::String(s) => s.contains(" old auid="), _ => false }) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?= old auid=) old auid=%{NUMBER:auditd.log.old_auid} new auid=%{NUMBER:auditd.log.new_auid} old ses=%{NUMBER:auditd.log.old_ses} new ses=%{NUMBER:auditd.log.new_ses}$
                if !cached_grok_mapped!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?= old auid=) old auid=%{NUMBER:auditd.log.old_auid} new auid=%{NUMBER:auditd.log.new_auid} old ses=%{NUMBER:auditd.log.old_ses} new ses=%{NUMBER:auditd.log.new_ses}$", [("auditd_log_kv", "auditd.log.kv")]).extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
            Ok(())
        })();
        }

        let _cond = { !event.has_value("auditd.log.record_type") && event.has_value("event.original") && (event.get("event.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(" msg='")), serde_json::Value::String(s) => s.contains(" msg='"), _ => false }) || event.get("event.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(" msg=\"")), serde_json::Value::String(s) => s.contains(" msg=\""), _ => false })) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?P<auditd_log_kv>.*?)(?= msg=) msg=(?:(?:'(?:(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})|[^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^']*))'|\"([^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^\"]*))\"))(?:(?:\\x1d)(?P<auditd_log_sub_kv_enriched>.*))?$
                if !cached_grok_mapped!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?P<auditd_log_kv>.*?)(?= msg=) msg=(?:(?:'(?:(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})|[^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^']*))'|\"([^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^\"]*))\"))(?:(?:\\x1d)(?P<auditd_log_sub_kv_enriched>.*))?$", [("auditd_log_sub_kv", "auditd.log.sub_kv"), ("auditd_log_sub_kv", "auditd.log.sub_kv"), ("auditd_log_kv", "auditd.log.kv"), ("auditd_log_sub_kv_enriched", "auditd.log.sub_kv_enriched")]).extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
            Ok(())
        })();
        }

        let _cond = { !event.has_value("auditd.log.record_type") && event.has_value("event.original") && event.get("event.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("={ ")), serde_json::Value::String(s) => s.contains("={ "), _ => false }) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?=\\x1d%{WORD}=\\{ )\\x1d%{WORD:auditd.log.original_field}={ (?P<auditd_log_sub_kv>.*) }$
                if !cached_grok_mapped!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?=\\x1d%{WORD}=\\{ )\\x1d%{WORD:auditd.log.original_field}={ (?P<auditd_log_sub_kv>.*) }$", [("auditd_log_kv", "auditd.log.kv"), ("auditd_log_sub_kv", "auditd.log.sub_kv")]).extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
            Ok(())
        })();
        }

        let _cond = { !event.has_value("auditd.log.record_type") };
        if _cond {
            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})?(?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))
                // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*.*$
                // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*$
                // Grok pattern: (?:type=%{NOTSPACE:auditd.log.record_type}) (?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))
                if !extract_first_match(
                    &[
                        cached_grok_mapped!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})?(?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))", [("auditd_log_kv", "auditd.log.kv")]),
                        cached_grok!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*.*$"),
                        cached_grok!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*$"),
                        cached_grok_mapped!("(?:type=%{NOTSPACE:auditd.log.record_type}) (?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))", [("auditd_log_kv", "auditd.log.kv")]),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("auditd.log.sub_kv") {
            gsub_field(event, "auditd.log.sub_kv", "auditd.log.sub_kv", cached_regex!("(res=[a-z]+)([A-Z][A-Za-z_]+=)"), "$1 $2")?;
        }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("auditd.log.kv") {
            gsub_field(event, "auditd.log.kv", "auditd.log.kv", cached_regex!("(res=[a-z]+)([A-Z][A-Za-z_]+=)"), "$1 $2")?;
        }
            Ok(())
        })();

        let _cond = { event.has_value("auditd.log.kv") };
        if _cond {
            if let Some(kv_str) = event.get_string("auditd.log.kv") {
                for pair in cached_regex!("(?:\\s+|\\x1d)(?=[^\\s\\x1d]+=)").split(&kv_str).into_iter() {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = ({ let parts = cached_regex!("(?<!\\\\)=").splitn(&pair, 2); match (parts.first(), parts.get(1)) { (Some(k), Some(v)) => Some((k.clone(), v.clone())), _ => None } }) else {
                        return Err(TransformError::ParseError {
                            path: "auditd.log.kv".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        let key = &key[..];
                        if !key.is_empty() {
                            kv_put(event, &format!("auditd.log.{}", key), value)?;
                        }
                    }
                }
            }
        }

        if event.has_value("auditd.log.sub_kv") {
            if let Some(kv_str) = event.get_string("auditd.log.sub_kv") {
                for pair in cached_regex!("\\s+(?=[^\\s]+=)").split(&kv_str).into_iter() {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "auditd.log.sub_kv".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        if !key.is_empty() {
                            kv_put(event, &format!("auditd.log.{}", key), value)?;
                        }
                    }
                }
            }
        }

        if event.has_value("auditd.log.sub_kv_enriched") {
            if let Some(kv_str) = event.get_string("auditd.log.sub_kv_enriched") {
                for pair in cached_regex!("\\s+(?=[^\\s]+=)").split(&kv_str).into_iter() {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "auditd.log.sub_kv_enriched".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        if !key.is_empty() {
                            kv_put(event, &format!("auditd.log.{}", key), value)?;
                        }
                    }
                }
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("auditd.log.epoch") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "auditd.log.epoch".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })();

        if event.has_value("auditd.log.original_field") {
            map_strings(event, "auditd.log.original_field", "auditd.log.original_field", str::to_lowercase)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.rename("auditd.log.old-auid", "auditd.log.old_auid")?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.rename("auditd.log.old-ses", "auditd.log.old_ses")?;
            Ok(())
        })();

        Ok(TransformResult::Continue)
    }
}
