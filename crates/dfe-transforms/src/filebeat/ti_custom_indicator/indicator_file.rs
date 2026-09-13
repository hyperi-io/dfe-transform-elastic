// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_file` pipeline.
pub struct IndicatorFile;

impl Transform for IndicatorFile {
    fn name(&self) -> &str {
        "indicator_file"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: (?i:^\\[?file:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?)
                // Grok pattern: (?i:^\\[?file:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?)
                // Grok pattern: (?i:^\\[?file:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?)
                // Grok pattern: ^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:_tmp.filename}'\\]?
                if !extract_first_match(
                    &[
                        cached_grok!("(?i:^\\[?file:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?)"),
                        cached_grok!("(?i:^\\[?file:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?)"),
                        cached_grok!("(?i:^\\[?file:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?)"),
                        cached_grok!("^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:_tmp.filename}'\\]?"),
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
            event.append("threat.indicator.file.hash.md5", json!(event.get("_tmp.md5").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.sha1") };
        if _cond {
            event.append("threat.indicator.file.hash.sha1", json!(event.get("_tmp.sha1").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.sha256") };
        if _cond {
            event.append("threat.indicator.file.hash.sha256", json!(event.get("_tmp.sha256").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.filename") };
        if _cond {
            event.append("threat.indicator.file.name", json!(event.get("_tmp.filename").map_or_else(String::new, template_to_string)))?;
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

            event.remove("_tmp");

        Ok(TransformResult::Continue)
    }
}
