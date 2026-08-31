// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_url` pipeline.
pub struct IndicatorUrl;

impl Transform for IndicatorUrl {
    fn name(&self) -> &str {
        "indicator_url"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?
                if !cached_grok!("^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
            Ok(())
        })();

        let _cond = { event.has_value("_tmp.url") };
        if _cond {
            event.append("threat.indicator.url.original", json!(event.get("_tmp.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.url") };
        if _cond {
            event.append("threat.indicator.name", json!(event.get("_tmp.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.url") };
        if _cond {
            event.append("threat.indicator.url.full", json!(event.get("_tmp.url").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("_tmp");

        Ok(TransformResult::Continue)
    }
}
