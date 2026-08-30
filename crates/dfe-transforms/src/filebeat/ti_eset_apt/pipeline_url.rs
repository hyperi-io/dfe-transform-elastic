// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_url` pipeline.
pub struct PipelineUrl;

impl Transform for PipelineUrl {
    fn name(&self) -> &str {
        "pipeline_url"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            foreach_array(event, "eti._patterns", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?
                // Grok pattern: ^\\[?url:x_misp_scheme%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.scheme}'\\]?
                // Grok pattern: ^\\[?url:x_misp_port%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.port:int}'\\]?
                // Grok pattern: ^\\[?url:x_misp_resource_path%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.path}'\\]?
                let _ = extract_first_match(
                &[
                cached_grok!("^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?"),
                cached_grok!("^\\[?url:x_misp_scheme%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.scheme}'\\]?"),
                cached_grok!("^\\[?url:x_misp_port%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.port:int}'\\]?"),
                cached_grok!("^\\[?url:x_misp_resource_path%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.path}'\\]?"),
                ],
                &input,
                event,
                )?;
                }
                Ok(())
                })();
                Ok(())
            })?;

        Ok(TransformResult::Continue)
    }
}
