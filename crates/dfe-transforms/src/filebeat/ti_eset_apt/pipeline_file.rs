// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_file` pipeline.
pub struct PipelineFile;

impl Transform for PipelineFile {
    fn name(&self) -> &str {
        "pipeline_file"
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
                        // Grok pattern: ^\\[?file:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?
                        // Grok pattern: ^\\[?file:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?
                        // Grok pattern: ^\\[?file:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?
                        // Grok pattern: ^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'\\]?
                        if !extract_first_match(
                        &[
                        cached_grok!("^\\[?file:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?"),
                        cached_grok!("^\\[?file:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?"),
                        cached_grok!("^\\[?file:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?"),
                        cached_grok!("^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'\\]?"),
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

        Ok(TransformResult::Continue)
    }
}
