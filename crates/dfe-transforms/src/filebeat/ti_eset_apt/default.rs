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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.append_unique("event.category", json!("threat"))?;

            event.append_unique("event.type", json!("indicator"))?;

            event.set("threat.indicator.provider", json!("eset"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "eti")?;

            let _cond = { event.get_str("eti.type") != Some("indicator") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("threat.feed.name", json!("ESET APT stix 2.1"))?;

            let _cond = { event.has_value("eti.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("eti.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "eti.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("eti.modified") };
            if _cond {
                if let Some(date_str) = event.get_as_string("eti.modified") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "eti.modified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.last_seen") };
            if _cond {
                if let Some(v) = event.get("threat.indicator.last_seen").cloned() {
                    event.set("threat.indicator.modified_at", v)?;
                }
            }

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                // Painless script
                // Source: if (ctx.eset == null) {\n  ctx.eset = new HashMap();\n}\nctx.eset.valid_until =  ZonedDateTime.parse(ctx['@timestamp']).plusDays(365);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.eset == null) {\n  ctx.eset = new HashMap();\n}\nctx.eset.valid_until =  ZonedDateTime.parse(ctx['@timestamp']).plusDays(365);"#
                    ),
                )?;
            }

            if event.has_value("eti.id") {
                event.rename("eti.id", "eset.id")?;
            }

            let _cond = { event.has_value("eti.labels") };
            if _cond {
                foreach_array(event, "eti.labels", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_ingest._value") {
                            // Grok pattern: ^misp:name=\"%{DATA:eset.name}\"
                            // Grok pattern: ^misp:type=\"%{DATA:eset.type}\"
                            // Grok pattern: ^misp:category=\"%{DATA:eset.category}\"
                            // Grok pattern: ^misp:meta-category=\"%{DATA:eset.meta_category}\"
                            let _ = extract_first_match(
                                &[
                                    cached_grok!("^misp:name=\"%{DATA:eset.name}\""),
                                    cached_grok!("^misp:type=\"%{DATA:eset.type}\""),
                                    cached_grok!("^misp:category=\"%{DATA:eset.category}\""),
                                    cached_grok!(
                                        "^misp:meta-category=\"%{DATA:eset.meta_category}\""
                                    ),
                                ],
                                &input,
                                event,
                            )?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get_str("eset.name") == Some("x509") };
            if _cond {
                event.set("threat.indicator.type", json!("x509-certificate"))?;
            }

            let _cond = {
                event.has_value("eset.name")
                    && event
                        .get_str("eset.name")
                        .is_some_and(|s| s.starts_with("domain"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.get_str("eset.name") == Some("file")
                    || event.get_str("eset.meta_category") == Some("file")
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("eset.name") == Some("url") };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = {
                event.get_str("eset.name") == Some("email")
                    || (event.has_value("eset.type")
                        && event
                            .get_str("eset.type")
                            .is_some_and(|s| s.starts_with("email")))
            };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = { event.has_value("eti.pattern") };
            if _cond {
                if let Some(s) = event.get_string("eti.pattern") {
                    let mut parts: Vec<Value> = s.split(" AND ").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("eti._patterns", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("file") };
            if _cond {
                // Begin nested pipeline: "pipeline-file"
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
                                    cached_grok!(
                                        "^\\[?file:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?file:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?file:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'\\]?"
                                    ),
                                ],
                                &input,
                                event,
                            )?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
                // End nested pipeline: "pipeline-file"
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                // Begin nested pipeline: "pipeline-email"
                foreach_array(event, "eti._patterns", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_ingest._value") {
                            // Grok pattern: ^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?
                            let _ = cached_grok!("^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?").extract_into(&input, event)?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
                let _cond = { event.has_value("threat.indicator.email.address") };
                if _cond {
                    foreach_array(event, "eti._patterns", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(input) = event.get_string("_ingest._value") {
                                // Grok pattern: ^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?
                                let _ = cached_grok!("^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'\\]?").extract_into(&input, event)?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("threat.indicator.email.address") };
                if _cond {
                    // Painless script
                    // Source: ctx.threat.indicator.email.address = ctx.threat.indicator.email.address.splitOnToken(' ');\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.threat.indicator.email.address = ctx.threat.indicator.email.address.splitOnToken(' ');\n"#
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline-email"
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                // Begin nested pipeline: "pipeline-url"
                foreach_array(event, "eti._patterns", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_ingest._value") {
                            // Grok pattern: ^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?
                            // Grok pattern: ^\\[?url:x_misp_scheme%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.scheme}'\\]?
                            // Grok pattern: ^\\[?url:x_misp_port%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.port:int}'\\]?
                            // Grok pattern: ^\\[?url:x_misp_resource_path%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.path}'\\]?
                            let _ = extract_first_match(
                                &[
                                    cached_grok!(
                                        "^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?url:x_misp_scheme%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.scheme}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?url:x_misp_port%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.port:int}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?url:x_misp_resource_path%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.path}'\\]?"
                                    ),
                                ],
                                &input,
                                event,
                            )?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
                // End nested pipeline: "pipeline-url"
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("domain-name") };
            if _cond {
                // Begin nested pipeline: "pipeline-domain-ip"
                foreach_array(event, "eti._patterns", |event| {
                    if let Some(input) = event.get_string("_ingest._value") {
                        // Grok pattern: ^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?
                        // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?"
                                ),
                                cached_grok!(
                                    "^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })?;
                // End nested pipeline: "pipeline-domain-ip"
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("x509-certificate") };
            if _cond {
                // Begin nested pipeline: "pipeline-cert"
                foreach_array(event, "eti._patterns", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_ingest._value") {
                            // Grok pattern: ^\\[?x509-certificate:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.serial_number}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.signature_algorithm}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.version_number}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_after}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_before}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.issuer.distinguished_name}'\\]?
                            // Grok pattern: ^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.subject.distinguished_name}'\\]?
                            let _ = extract_first_match(
                                &[
                                    cached_grok!(
                                        "^\\[?x509-certificate:hashes.MD5%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:hashes.SHA1%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:hashes.SHA256%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.serial_number}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.signature_algorithm}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.version_number}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_after}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:threat.indicator.x509.not_before}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.issuer.distinguished_name}'\\]?"
                                    ),
                                    cached_grok!(
                                        "^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:threat.indicator.x509.subject.distinguished_name}'\\]?"
                                    ),
                                ],
                                &input,
                                event,
                            )?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
                let _cond = { event.has_value("threat.indicator.x509.not_after") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("threat.indicator.x509.not_after") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd HH:mm:ssz"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("threat.indicator.x509.not_after", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threat.indicator.x509.not_after".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = { event.has_value("threat.indicator.x509.not_before") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("threat.indicator.x509.not_before")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd HH:mm:ssz"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("threat.indicator.x509.not_before", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threat.indicator.x509.not_before".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = { event.has_value("threat.indicator.x509.issuer.distinguished_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) =
                            event.get_string("threat.indicator.x509.issuer.distinguished_name")
                        {
                            for pair in cached_regex!("(?<!\\\\),").split(&kv_str).into_iter() {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "threat.indicator.x509.issuer.distinguished_name"
                                            .into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c| " ".contains(c));
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("eti._issuer_fields.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("eti._issuer_fields.CN") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.common_name",
                        json!(
                            event
                                .get("eti._issuer_fields.CN")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._issuer_fields.C") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.country",
                        json!(
                            event
                                .get("eti._issuer_fields.C")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._issuer_fields.L") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.locality",
                        json!(
                            event
                                .get("eti._issuer_fields.L")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._issuer_fields.O") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.organization",
                        json!(
                            event
                                .get("eti._issuer_fields.O")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._issuer_fields.OU") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.organizational_unit",
                        json!(
                            event
                                .get("eti._issuer_fields.OU")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._issuer_fields.S") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.state_or_province",
                        json!(
                            event
                                .get("eti._issuer_fields.S")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._issuer_fields.ST") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.state_or_province",
                        json!(
                            event
                                .get("eti._issuer_fields.ST")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._issuer_fields.P") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.issuer.state_or_province",
                        json!(
                            event
                                .get("eti._issuer_fields.P")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("threat.indicator.x509.subject.distinguished_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) =
                            event.get_string("threat.indicator.x509.subject.distinguished_name")
                        {
                            for pair in cached_regex!("(?<!\\\\),").split(&kv_str).into_iter() {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "threat.indicator.x509.subject.distinguished_name"
                                            .into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c| " ".contains(c));
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("eti._subject_fields.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("eti._subject_fields.CN") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.common_name",
                        json!(
                            event
                                .get("eti._subject_fields.CN")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._subject_fields.C") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.country",
                        json!(
                            event
                                .get("eti._subject_fields.C")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._subject_fields.L") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.locality",
                        json!(
                            event
                                .get("eti._subject_fields.L")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._subject_fields.O") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.organization",
                        json!(
                            event
                                .get("eti._subject_fields.O")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._subject_fields.OU") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.organizational_unit",
                        json!(
                            event
                                .get("eti._subject_fields.OU")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._subject_fields.S") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.state_or_province",
                        json!(
                            event
                                .get("eti._subject_fields.S")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._subject_fields.ST") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.state_or_province",
                        json!(
                            event
                                .get("eti._subject_fields.ST")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("eti._subject_fields.P") };
                if _cond {
                    event.append(
                        "threat.indicator.x509.subject.state_or_province",
                        json!(
                            event
                                .get("eti._subject_fields.P")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline-cert"
            }

            event.set("threat.indicator.confidence", json!("High"))?;

            event.remove("eti");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
