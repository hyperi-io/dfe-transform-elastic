// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_file` pipeline.
pub struct PipelineFile;

impl Transform for PipelineFile {
    fn name(&self) -> &str {
        "pipeline_file"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            foreach_array(event, "eti._patterns", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: ^\\[?file:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?
                // Grok pattern: ^\\[?file:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?
                // Grok pattern: ^\\[?file:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?
                // Grok pattern: ^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'\\]?
                let _ = extract_first_match(
                &[
                cached_grok!("^\\[?file:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?"),
                cached_grok!("^\\[?file:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?"),
                cached_grok!("^\\[?file:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?"),
                cached_grok!("^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'\\]?"),
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
