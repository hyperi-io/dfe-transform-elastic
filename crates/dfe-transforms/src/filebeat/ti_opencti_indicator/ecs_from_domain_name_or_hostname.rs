// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_domain_name_or_hostname` pipeline.
pub struct EcsFromDomainNameOrHostname;

impl Transform for EcsFromDomainNameOrHostname {
    fn name(&self) -> &str {
        "ecs_from_domain_name_or_hostname"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.rename("_ingest._value", "_tmp_url.domain")?;

        if event.has_value("_tmp_url.domain") {
            if let Some(domain_str) = event.get_string("_tmp_url.domain") {
                let domain = domain_str.to_string();
                event.set("_tmp_url.domain", json!(domain.clone()))?;
                // Public suffix list lookup for registered domain extraction
                if let Some(rd) = registered_domain_lookup(&domain) {
                    if let Some(registered) = rd.registered_domain {
                        event.set("_tmp_url.registered_domain", json!(registered))?;
                    }
                    event.set("_tmp_url.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("_tmp_url.subdomain", json!(sub))?;
                    }
                }
            }
        }

            // Painless script, resolved to its runners at generation time
            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.url = ctx.threat.indicator.url ?: [];\nctx.threat.indicator.url.add(ctx._tmp_url);\n
            ensure_append(event, &EnsureAppend::new("_tmp_url", "threat.indicator.url"));

            if event.remove("_tmp_url").is_none() {
                return Err(TransformError::FieldNotFound { path: "_tmp_url".into() });
            }

        Ok(TransformResult::Continue)
    }
}
