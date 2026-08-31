// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_domain_name` pipeline.
pub struct IndicatorDomainName;

impl Transform for IndicatorDomainName {
    fn name(&self) -> &str {
        "indicator_domain_name"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?
                // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?
                // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?
                // Grok pattern: ^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                if !extract_first_match(
                    &[
                        cached_grok!("^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?"),
                        cached_grok!("^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"),
                        cached_grok!("^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?"),
                        cached_grok!("^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"),
                        cached_grok!("^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?"),
                        cached_grok!("^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"),
                    ],
                    &input,
                    event,
                )? {
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
            event.append("threat.indicator.url.domain", json!(event.get("_tmp.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.ip") };
        if _cond {
            event.append("threat.indicator.ip", json!(event.get("_tmp.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.url") };
        if _cond {
            event.append("threat.indicator.name", json!(event.get("_tmp.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("_tmp.ip").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("_tmp");

        Ok(TransformResult::Continue)
    }
}
