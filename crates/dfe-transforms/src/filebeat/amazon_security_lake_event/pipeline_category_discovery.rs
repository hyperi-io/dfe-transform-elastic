// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_category_discovery` pipeline.
pub struct PipelineCategoryDiscovery;

impl Transform for PipelineCategoryDiscovery {
    fn name(&self) -> &str {
        "pipeline_category_discovery"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if let Some(v) = event.get("ocsf.cis_benchmark_result.rule.category").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("rule.category", v)?;
        }

        if let Some(v) = event.get("ocsf.cis_benchmark_result.rule.desc").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("rule.description", v)?;
        }

        if let Some(v) = event.get("ocsf.cis_benchmark_result.rule.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("rule.name", v)?;
        }

        if let Some(v) = event.get("ocsf.cis_benchmark_result.rule.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("rule.uuid", v)?;
        }

        if let Some(v) = event.get("ocsf.cis_benchmark_result.rule.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("rule.version", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
