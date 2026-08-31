// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_asn` pipeline.
pub struct IndicatorAsn;

impl Transform for IndicatorAsn {
    fn name(&self) -> &str {
        "indicator_asn"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?autonomous-system:number%{SPACE}=%{SPACE}%{INT:_tmp.as_number}\\]?
                // Grok pattern: ^\\[?autonomous-system:number%{SPACE}=%{SPACE}'%{INT:_tmp.as_number}'\\]?
                if !extract_first_match(
                    &[
                        cached_grok!("^\\[?autonomous-system:number%{SPACE}=%{SPACE}%{INT:_tmp.as_number}\\]?"),
                        cached_grok!("^\\[?autonomous-system:number%{SPACE}=%{SPACE}'%{INT:_tmp.as_number}'\\]?"),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
            Ok(())
        })();

        let _cond = { event.has_value("_tmp.as_number") };
        if _cond {
            event.append("threat.indicator.as.number", json!(event.get("_tmp.as_number").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("_tmp");

        Ok(TransformResult::Continue)
    }
}
