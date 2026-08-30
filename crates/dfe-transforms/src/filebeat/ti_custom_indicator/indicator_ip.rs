// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_ip` pipeline.
pub struct IndicatorIp;

impl Transform for IndicatorIp {
    fn name(&self) -> &str {
        "indicator_ip"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?
                // Grok pattern: ^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                let _ = extract_first_match(
                    &[
                        cached_grok!("^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"),
                        cached_grok!("^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?"),
                        cached_grok!("^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"),
                    ],
                    &input,
                    event,
                )?;
            }
            Ok(())
        })();

        let _cond = { event.has_value("_tmp.ip") };
        if _cond {
            event.append("threat.indicator.ip", json!(event.get("_tmp.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("_tmp.ip").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("_tmp");

        Ok(TransformResult::Continue)
    }
}
