// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_email` pipeline.
pub struct IndicatorEmail;

impl Transform for IndicatorEmail {
    fn name(&self) -> &str {
        "indicator_email"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?email-addr:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                // Grok pattern: ^\\[?email-message:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                // Grok pattern: ^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                // Grok pattern: ^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                let _ = extract_first_match(
                    &[
                        cached_grok!("^\\[?email-addr:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"),
                        cached_grok!("^\\[?email-message:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"),
                        cached_grok!("^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"),
                        cached_grok!("^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"),
                    ],
                    &input,
                    event,
                )?;
            }
            Ok(())
        })();

        let _cond = { event.has_value("_tmp.email_addr") };
        if _cond {
            event.append("threat.indicator.email.address", json!(event.get("_tmp.email_addr").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("_tmp");

        Ok(TransformResult::Continue)
    }
}
