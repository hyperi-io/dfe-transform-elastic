// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_extract_metadata` pipeline.
pub struct PipelineExtractMetadata;

impl Transform for PipelineExtractMetadata {
    fn name(&self) -> &str {
        "pipeline_extract_metadata"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            parse_json_field(event, "resource_tags", "aws_billing.cur.resource_tags")?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            parse_json_field(event, "cost_category", "aws_billing.cur.cost_category")?;
            Ok(())
        })();

        Ok(TransformResult::Continue)
    }
}
