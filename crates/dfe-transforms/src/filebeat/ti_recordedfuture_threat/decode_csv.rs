// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `decode_csv` pipeline.
pub struct DecodeCsv;

impl Transform for DecodeCsv {
    fn name(&self) -> &str {
        "decode_csv"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("_tmp_.col0", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("_tmp_.col1", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("_tmp_.col2", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("_tmp_.col3", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("_tmp_.col4", val)?;
                            }
                        }
                    }
                }

            let _cond = { event.get_str("_tmp_.col0") == Some("Name") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

                // Painless script
                // Source: def cols = params[ ctx._tmp_.col4 == null? \"default\" : \"hash\" ]; def src = ctx._tmp_; def dst = new HashMap(); for (entry in cols.entrySet()) {\n  dst[entry.getValue()] = src[entry.getKey()];\n} ctx['json'] = dst;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def cols = params[ ctx._tmp_.col4 == null? \"default\" : \"hash\" ]; def src = ctx._tmp_; def dst = new HashMap(); for (entry in cols.entrySet()) {\n  dst[entry.getValue()] = src[entry.getKey()];\n} ctx['json'] = dst;\n"#), cached_params!("{\"default\":{\"col0\":\"Name\",\"col1\":\"Risk\",\"col2\":\"RiskString\",\"col3\":\"EvidenceDetails\"},\"hash\":{\"col0\":\"Name\",\"col1\":\"Algorithm\",\"col2\":\"Risk\",\"col3\":\"RiskString\",\"col4\":\"EvidenceDetails\"}}"))?;

                if event.remove("_tmp_").is_none() {
                    return Err(TransformError::FieldNotFound { path: "_tmp_".into() });
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
