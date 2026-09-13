// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_x509_certificate` pipeline.
pub struct EcsFromX509Certificate;

impl Transform for EcsFromX509Certificate {
    fn name(&self) -> &str {
        "ecs_from_x509_certificate"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("_tmp_x509.alternative_names", json!(event.get("_ingest._value.subject_alternative_name").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.issuer.common_name", json!(event.get("_ingest._value.issuer").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.not_after", json!(event.get("_ingest._value.validity_not_after").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.not_before", json!(event.get("_ingest._value.validity_not_before").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.public_key_algorithm", json!(event.get("_ingest._value.subject_public_key_algorithm").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.public_key_exponent", json!(event.get("_ingest._value.subject_public_key_exponent").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.serial_number", json!(event.get("_ingest._value.serial_number").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.signature_algorithm", json!(event.get("_ingest._value.signature_algorithm").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.subject.common_name", json!(event.get("_ingest._value.subject").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_x509.version_number", json!(event.get("_ingest._value.version").map_or_else(String::new, template_to_string)))?;

            // Painless script, resolved to its runners at generation time
            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.x509 = ctx.threat.indicator.x509 ?: [];\nctx.threat.indicator.x509.add(ctx._tmp_x509);\n
            ensure_append(event, &EnsureAppend::new("_tmp_x509", "threat.indicator.x509"));

            if event.remove("_tmp_x509").is_none() {
                return Err(TransformError::FieldNotFound { path: "_tmp_x509".into() });
            }

        Ok(TransformResult::Continue)
    }
}
