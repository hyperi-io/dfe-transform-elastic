// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_windows_registry` pipeline.
pub struct IndicatorWindowsRegistry;

impl Transform for IndicatorWindowsRegistry {
    fn name(&self) -> &str {
        "indicator_windows_registry"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_path}'\\]?
                // Grok pattern: ^\\[?windows-registry-key:key%{SPACE}LIKE%{SPACE}'%{DATA:_tmp.reg_path}'\\]?
                // Grok pattern: ^\\[?windows-registry-value-type:name%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_key}'\\]?
                // Grok pattern: ^\\[?windows-registry-value-type:data%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_value}'\\]?
                let _ = extract_first_match(
                    &[
                        cached_grok!("^\\[?windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_path}'\\]?"),
                        cached_grok!("^\\[?windows-registry-key:key%{SPACE}LIKE%{SPACE}'%{DATA:_tmp.reg_path}'\\]?"),
                        cached_grok!("^\\[?windows-registry-value-type:name%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_key}'\\]?"),
                        cached_grok!("^\\[?windows-registry-value-type:data%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_value}'\\]?"),
                    ],
                    &input,
                    event,
                )?;
            }
            Ok(())
        })();

        let _cond = { event.has_value("_tmp.reg_path") };
        if _cond {
            event.append("threat.indicator.registry.path", json!(event.get("_tmp.reg_path").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.reg_key") };
        if _cond {
            event.append("threat.indicator.registry.key", json!(event.get("_tmp.reg_key").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.reg_value") };
        if _cond {
            event.append("threat.indicator.registry.value", json!(event.get("_tmp.reg_value").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("_tmp");

        Ok(TransformResult::Continue)
    }
}
