// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_domain_ip` pipeline.
pub struct PipelineDomainIp;

impl Transform for PipelineDomainIp {
    fn name(&self) -> &str {
        "pipeline_domain_ip"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            foreach_array(event, "eti._patterns", |event| {
                if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?
                // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?
                let _ = extract_first_match(
                &[
                cached_grok!("^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?"),
                cached_grok!("^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?"),
                ],
                &input,
                event,
                )?;
                }
                Ok(())
            })?;

        Ok(TransformResult::Continue)
    }
}
