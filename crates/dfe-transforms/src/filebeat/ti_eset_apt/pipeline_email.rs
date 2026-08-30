// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_email` pipeline.
pub struct PipelineEmail;

impl Transform for PipelineEmail {
    fn name(&self) -> &str {
        "pipeline_email"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            foreach_array(event, "eti._patterns", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?
                let _ = cached_grok!("^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?").extract_into(&input, event)?;
                }
                Ok(())
                })();
                Ok(())
            })?;

        let _cond = { event.has_value("threat.indicator.email.address") };
        if _cond {
            foreach_array(event, "eti._patterns", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?
                let _ = cached_grok!("^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?").extract_into(&input, event)?;
                }
                Ok(())
                })();
                Ok(())
            })?;
        }

        let _cond = { event.has_value("threat.indicator.email.address") };
        if _cond {
            // Painless script
            // Source: ctx.threat.indicator.email.address = ctx.threat.indicator.email.address.splitOnToken(' ');\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.threat.indicator.email.address = ctx.threat.indicator.email.address.splitOnToken(' ');\n"#))?;
        }

        Ok(TransformResult::Continue)
    }
}
