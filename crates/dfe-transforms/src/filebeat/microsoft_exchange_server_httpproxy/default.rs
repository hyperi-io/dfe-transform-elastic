// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| cached_regex!(r"^[^0-9]").is_match(s))
                    || event
                        .get_str("message")
                        .is_some_and(|s| cached_regex!(r"^#").is_match(s))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("event.original") {
                    if let Some(csv_str) = event.get_string("event.original") {
                        let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                        let mut rdr = csv::ReaderBuilder::new()
                            .delimiter(b',')
                            .quote(b'\"')
                            .has_headers(false)
                            .from_reader(csv_str.as_bytes());
                        if let Some(Ok(record)) = rdr.records().next() {
                            if let Some(val) = record.get(0) {
                                if !val.is_empty() {
                                    event.set("@timestamp", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.requestid", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.majorversion", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.minorversion", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.buildversion", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.revisionversion", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.clientrequestid", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.protocol", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.urlhost", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.urlstem", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.protocolaction", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.authenticationtype", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.isauthenticated", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.authenticateduser", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.organization", val)?;
                                }
                            }
                            if let Some(val) = record.get(15) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.anchormailbox", val)?;
                                }
                            }
                            if let Some(val) = record.get(16) {
                                if !val.is_empty() {
                                    event.set("user_agent.original", val)?;
                                }
                            }
                            if let Some(val) = record.get(17) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.clientipaddress", val)?;
                                }
                            }
                            if let Some(val) = record.get(18) {
                                if !val.is_empty() {
                                    event.set("observer.hostname", val)?;
                                }
                            }
                            if let Some(val) = record.get(19) {
                                if !val.is_empty() {
                                    event.set("http.response.status_code", val)?;
                                }
                            }
                            if let Some(val) = record.get(20) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.backendstatus", val)?;
                                }
                            }
                            if let Some(val) = record.get(21) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.errorcode", val)?;
                                }
                            }
                            if let Some(val) = record.get(22) {
                                if !val.is_empty() {
                                    event.set("http.request.method", val)?;
                                }
                            }
                            if let Some(val) = record.get(23) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.proxyaction", val)?;
                                }
                            }
                            if let Some(val) = record.get(24) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.targetserver", val)?;
                                }
                            }
                            if let Some(val) = record.get(25) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.targetserverversion", val)?;
                                }
                            }
                            if let Some(val) = record.get(26) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.routingtype", val)?;
                                }
                            }
                            if let Some(val) = record.get(27) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.routinghint", val)?;
                                }
                            }
                            if let Some(val) = record.get(28) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.backendcookie", val)?;
                                }
                            }
                            if let Some(val) = record.get(29) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.serverlocatorhost", val)?;
                                }
                            }
                            if let Some(val) = record.get(30) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.serverlocatorlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(31) {
                                if !val.is_empty() {
                                    event.set("http.request.bytes", val)?;
                                }
                            }
                            if let Some(val) = record.get(32) {
                                if !val.is_empty() {
                                    event.set("http.response.bytes", val)?;
                                }
                            }
                            if let Some(val) = record.get(33) {
                                if !val.is_empty() {
                                    event
                                        .set("microsoft.exchange.targetoutstandingrequests", val)?;
                                }
                            }
                            if let Some(val) = record.get(34) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.authmoduleperfcontext", val)?;
                                }
                            }
                            if let Some(val) = record.get(35) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.httppipelinelatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(36) {
                                if !val.is_empty() {
                                    event.set(
                                        "microsoft.exchange.calculatetargetbackendlatency",
                                        val,
                                    )?;
                                }
                            }
                            if let Some(val) = record.get(37) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.glslatencybreakup", val)?;
                                }
                            }
                            if let Some(val) = record.get(38) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.totalglslatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(39) {
                                if !val.is_empty() {
                                    event.set(
                                        "microsoft.exchange.accountforestlatencybreakup",
                                        val,
                                    )?;
                                }
                            }
                            if let Some(val) = record.get(40) {
                                if !val.is_empty() {
                                    event
                                        .set("microsoft.exchange.totalaccountforestlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(41) {
                                if !val.is_empty() {
                                    event.set(
                                        "microsoft.exchange.resourceforestlatencybreakup",
                                        val,
                                    )?;
                                }
                            }
                            if let Some(val) = record.get(42) {
                                if !val.is_empty() {
                                    event.set(
                                        "microsoft.exchange.totalresourceforestlatency",
                                        val,
                                    )?;
                                }
                            }
                            if let Some(val) = record.get(43) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.adlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(44) {
                                if !val.is_empty() {
                                    event
                                        .set("microsoft.exchange.sharedcachelatencybreakup", val)?;
                                }
                            }
                            if let Some(val) = record.get(45) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.totalsharedcachelatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(46) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.activitycontextlifetime", val)?;
                                }
                            }
                            if let Some(val) = record.get(47) {
                                if !val.is_empty() {
                                    event.set(
                                        "microsoft.exchange.moduletohandlerswitchinglatency",
                                        val,
                                    )?;
                                }
                            }
                            if let Some(val) = record.get(48) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.clientreqstreamlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(49) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.backendreqinitlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(50) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.backendreqstreamlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(51) {
                                if !val.is_empty() {
                                    event
                                        .set("microsoft.exchange.backendprocessinglatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(52) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.backendrespinitlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(53) {
                                if !val.is_empty() {
                                    event
                                        .set("microsoft.exchange.backendrespstreamlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(54) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.clientrespstreamlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(55) {
                                if !val.is_empty() {
                                    event
                                        .set("microsoft.exchange.kerberosauthheaderlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(56) {
                                if !val.is_empty() {
                                    event
                                        .set("microsoft.exchange.handlercompletionlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(57) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.requesthandlerlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(58) {
                                if !val.is_empty() {
                                    event.set(
                                        "microsoft.exchange.handlertomoduleswitchinglatency",
                                        val,
                                    )?;
                                }
                            }
                            if let Some(val) = record.get(59) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.proxytime", val)?;
                                }
                            }
                            if let Some(val) = record.get(60) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.corelatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(61) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.routinglatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(62) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.httpproxyoverhead", val)?;
                                }
                            }
                            if let Some(val) = record.get(63) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.totalrequesttime", val)?;
                                }
                            }
                            if let Some(val) = record.get(64) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.routerefresherlatency", val)?;
                                }
                            }
                            if let Some(val) = record.get(65) {
                                if !val.is_empty() {
                                    event.set("url.query", val)?;
                                }
                            }
                            if let Some(val) = record.get(66) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.backendgenericinfo", val)?;
                                }
                            }
                            if let Some(val) = record.get(67) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.genericinfo", val)?;
                                }
                            }
                            if let Some(val) = record.get(68) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.genericerrors", val)?;
                                }
                            }
                            if let Some(val) = record.get(69) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.edgetraceid", val)?;
                                }
                            }
                            if let Some(val) = record.get(70) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.databaseguid", val)?;
                                }
                            }
                            if let Some(val) = record.get(71) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.useradobjectguid", val)?;
                                }
                            }
                            if let Some(val) = record.get(72) {
                                if !val.is_empty() {
                                    event.set(
                                        "microsoft.exchange.partitionendpointlookuplatency",
                                        val,
                                    )?;
                                }
                            }
                            if let Some(val) = record.get(73) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.routingstatus", val)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("httpproxy");
                if !painless_is_empty_value(&v) {
                    event.set("microsoft.exchange.logtype", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.authenticateduser") {
                    if let Some(input) = event.get_string("microsoft.exchange.authenticateduser") {
                        // Grok pattern: %{DATA}\\\\%{NOTSPACE:user.name}
                        if !cached_grok!("%{DATA}\\\\%{NOTSPACE:user.name}")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.clientipaddress") {
                    if let Some(input) = event.get_string("microsoft.exchange.clientipaddress") {
                        // Grok pattern: ^%{IP:microsoft.exchange.clientipaddress_external}%{SPACE}%{IP:microsoft.exchange.clientipaddress_internal}$
                        if !cached_grok!("^%{IP:microsoft.exchange.clientipaddress_external}%{SPACE}%{IP:microsoft.exchange.clientipaddress_internal}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.clientipaddress") {
                    if let Some(s) = event.get_string("microsoft.exchange.clientipaddress") {
                        let mut parts: Vec<Value> = s.split("  ").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("microsoft.exchange.clientipaddress", Value::Array(parts))?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.request.bytes") {
                    if let Some(val) = event.get("http.request.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "http.request.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("http.request.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.response.bytes") {
                    if let Some(val) = event.get("http.response.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "http.response.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.response.status_code") {
                    if let Some(val) = event.get("http.response.status_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "http.response.status_code".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.status_code", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.adlatency") {
                    if let Some(val) = event.get("microsoft.exchange.adlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.adlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.adlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.backendprocessinglatency") {
                    if let Some(val) = event.get("microsoft.exchange.backendprocessinglatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.backendprocessinglatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.backendprocessinglatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.backendreqinitlatency") {
                    if let Some(val) = event.get("microsoft.exchange.backendreqinitlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.backendreqinitlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.backendreqinitlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.backendreqstreamlatency") {
                    if let Some(val) = event.get("microsoft.exchange.backendreqstreamlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.backendreqstreamlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.backendreqstreamlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.backendrespinitlatency") {
                    if let Some(val) = event.get("microsoft.exchange.backendrespinitlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.backendrespinitlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.backendrespinitlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.backendrespstreamlatency") {
                    if let Some(val) = event.get("microsoft.exchange.backendrespstreamlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.backendrespstreamlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.backendrespstreamlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.backendstatus") {
                    if let Some(val) = event.get("microsoft.exchange.backendstatus") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.backendstatus".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.backendstatus", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.buildversion") {
                    if let Some(val) = event.get("microsoft.exchange.buildversion") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.buildversion".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.buildversion", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.calculatetargetbackendlatency") {
                    if let Some(val) = event.get("microsoft.exchange.calculatetargetbackendlatency")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.calculatetargetbackendlatency".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft.exchange.calculatetargetbackendlatency",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.clientreqstreamlatency") {
                    if let Some(val) = event.get("microsoft.exchange.clientreqstreamlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.clientreqstreamlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.clientreqstreamlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.clientrespstreamlatency") {
                    if let Some(val) = event.get("microsoft.exchange.clientrespstreamlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.clientrespstreamlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.clientrespstreamlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.corelatency") {
                    if let Some(val) = event.get("microsoft.exchange.corelatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.corelatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.corelatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.handlercompletionlatency") {
                    if let Some(val) = event.get("microsoft.exchange.handlercompletionlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.handlercompletionlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.handlercompletionlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.handlertomoduleswitchinglatency") {
                    if let Some(val) =
                        event.get("microsoft.exchange.handlertomoduleswitchinglatency")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.handlertomoduleswitchinglatency".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft.exchange.handlertomoduleswitchinglatency",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.httppipelinelatency") {
                    if let Some(val) = event.get("microsoft.exchange.httppipelinelatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.httppipelinelatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.httppipelinelatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.kerberosauthheaderlatency") {
                    if let Some(val) = event.get("microsoft.exchange.kerberosauthheaderlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.kerberosauthheaderlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.kerberosauthheaderlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.majorversion") {
                    if let Some(val) = event.get("microsoft.exchange.majorversion") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.majorversion".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.majorversion", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.minorversion") {
                    if let Some(val) = event.get("microsoft.exchange.minorversion") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.minorversion".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.minorversion", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.moduletohandlerswitchinglatency") {
                    if let Some(val) =
                        event.get("microsoft.exchange.moduletohandlerswitchinglatency")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.moduletohandlerswitchinglatency".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft.exchange.moduletohandlerswitchinglatency",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.requesthandlerlatency") {
                    if let Some(val) = event.get("microsoft.exchange.requesthandlerlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.requesthandlerlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.requesthandlerlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.revisionversion") {
                    if let Some(val) = event.get("microsoft.exchange.revisionversion") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.revisionversion".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.revisionversion", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.routinglatency") {
                    if let Some(val) = event.get("microsoft.exchange.routinglatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.routinglatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.routinglatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.totalaccountforestlatency") {
                    if let Some(val) = event.get("microsoft.exchange.totalaccountforestlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.totalaccountforestlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.totalaccountforestlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.totalglslatency") {
                    if let Some(val) = event.get("microsoft.exchange.totalglslatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.totalglslatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.totalglslatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.totalrequesttime") {
                    if let Some(val) = event.get("microsoft.exchange.totalrequesttime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.totalrequesttime".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.totalrequesttime", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.totalresourceforestlatency") {
                    if let Some(val) = event.get("microsoft.exchange.totalresourceforestlatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.totalresourceforestlatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.totalresourceforestlatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.totalsharedcachelatency") {
                    if let Some(val) = event.get("microsoft.exchange.totalsharedcachelatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.totalsharedcachelatency".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.totalsharedcachelatency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.activitycontextlifetime") {
                    if let Some(val) = event.get("microsoft.exchange.activitycontextlifetime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.activitycontextlifetime".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.activitycontextlifetime", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.httpproxyoverhead") {
                    if let Some(val) = event.get("microsoft.exchange.httpproxyoverhead") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.httpproxyoverhead".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.httpproxyoverhead", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.proxytime") {
                    if let Some(val) = event.get("microsoft.exchange.proxytime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.proxytime".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.proxytime", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("event.ingested", v)?;
                }
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
