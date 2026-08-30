// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `rest` pipeline.
pub struct Rest;

impl Transform for Rest {
    fn name(&self) -> &str {
        "rest"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if event.has_value("tychon.windows_defender.service.nis.signature_age") {
            if let Some(val) = event.get("tychon.windows_defender.service.nis.signature_age") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.windows_defender.service.nis.signature_age".into(),
                        message,
                    })?;
                event.set("tychon.windows_defender.service.nis.signature_age", converted)?;
            }
        }

        if event.has_value("tychon.windows_defender.service.antivirus.signature_age") {
            if let Some(val) = event.get("tychon.windows_defender.service.antivirus.signature_age") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.windows_defender.service.antivirus.signature_age".into(),
                        message,
                    })?;
                event.set("tychon.windows_defender.service.antivirus.signature_age", converted)?;
            }
        }

        if event.has_value("tychon.windows_defender.service.antispyware.signature_age") {
            if let Some(val) = event.get("tychon.windows_defender.service.antispyware.signature_age") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.windows_defender.service.antispyware.signature_age".into(),
                        message,
                    })?;
                event.set("tychon.windows_defender.service.antispyware.signature_age", converted)?;
            }
        }

        if event.has_value("tychon.windows_defender.service.firewall.domain.enabled") {
            map_strings(event, "tychon.windows_defender.service.firewall.domain.enabled", "tychon.windows_defender.service.firewall.domain.enabled", str::to_lowercase)?;
        }

        if event.has_value("tychon.windows_defender.service.firewall.domain.log_blocked") {
            map_strings(event, "tychon.windows_defender.service.firewall.domain.log_blocked", "tychon.windows_defender.service.firewall.domain.log_blocked", str::to_lowercase)?;
        }

        if event.has_value("tychon.windows_defender.service.firewall.private.enabled") {
            map_strings(event, "tychon.windows_defender.service.firewall.private.enabled", "tychon.windows_defender.service.firewall.private.enabled", str::to_lowercase)?;
        }

        if event.has_value("tychon.windows_defender.service.firewall.private.log_blocked") {
            map_strings(event, "tychon.windows_defender.service.firewall.private.log_blocked", "tychon.windows_defender.service.firewall.private.log_blocked", str::to_lowercase)?;
        }

        if event.has_value("tychon.windows_defender.service.firewall.public.enabled") {
            map_strings(event, "tychon.windows_defender.service.firewall.public.enabled", "tychon.windows_defender.service.firewall.public.enabled", str::to_lowercase)?;
        }

        if event.has_value("tychon.windows_defender.service.firewall.public.log_blocked") {
            map_strings(event, "tychon.windows_defender.service.firewall.public.log_blocked", "tychon.windows_defender.service.firewall.public.log_blocked", str::to_lowercase)?;
        }

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        Ok(TransformResult::Continue)
    }
}
