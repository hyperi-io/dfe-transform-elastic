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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.x509")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.type", json!("info"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.version")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.serial")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.subject")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.issuer")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.not_valid_before")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.not_valid_after")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.key_alg")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.sig_alg")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.key_type")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.key_length")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.exponent")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "certificate.cn")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "zeek.x509.basic_constraints.ca")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.x509", "basic_constraints.path_len")?;
                Ok(())
            })();

            if event.has_value("zeek.x509.id") {
                event.rename("zeek.x509.id", "zeek.session_id")?;
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                if let Some(v) = event.get("zeek.session_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            if event.has_value("zeek.x509.certificate.not_valid_before") {
                event.rename(
                    "zeek.x509.certificate.not_valid_before",
                    "zeek.x509.certificate.valid.from",
                )?;
            }

            if event.has_value("zeek.x509.certificate.not_valid_after") {
                event.rename(
                    "zeek.x509.certificate.not_valid_after",
                    "zeek.x509.certificate.valid.until",
                )?;
            }

            if event.has_value("zeek.x509.basic_constraints.ca") {
                event.rename(
                    "zeek.x509.basic_constraints.ca",
                    "zeek.x509.basic_constraints.certificate_authority",
                )?;
            }

            if event.has_value("zeek.x509.basic_constraints.path_len") {
                event.rename(
                    "zeek.x509.basic_constraints.path_len",
                    "zeek.x509.basic_constraints.path_length",
                )?;
            }

            if event.has_value("zeek.x509.certificate.cn") {
                event.rename(
                    "zeek.x509.certificate.cn",
                    "zeek.x509.certificate.common_name",
                )?;
            }

            if event.has_value("zeek.x509.certificate.issuer") {
                event.rename("zeek.x509.certificate.issuer", "zeek.x509.certificate.iss")?;
            }

            if event.has_value("zeek.x509.certificate.subject") {
                event.rename("zeek.x509.certificate.subject", "zeek.x509.certificate.sub")?;
            }

            if event.has_value("zeek.x509.certificate.key_alg") {
                event.rename(
                    "zeek.x509.certificate.key_alg",
                    "zeek.x509.certificate.key.algorithm",
                )?;
            }

            if event.has_value("zeek.x509.certificate.key_length") {
                event.rename(
                    "zeek.x509.certificate.key_length",
                    "zeek.x509.certificate.key.length",
                )?;
            }

            if event.has_value("zeek.x509.certificate.key_type") {
                event.rename(
                    "zeek.x509.certificate.key_type",
                    "zeek.x509.certificate.key.type",
                )?;
            }

            if event.has_value("zeek.x509.certificate.sig_alg") {
                event.rename(
                    "zeek.x509.certificate.sig_alg",
                    "zeek.x509.certificate.signature_algorithm",
                )?;
            }

            if event.has_value("zeek.x509.logcert") {
                event.rename("zeek.x509.logcert", "zeek.x509.log_cert")?;
            }

            if let Some(date_str) = event.get_as_string("zeek.x509.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.x509.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.x509.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.x509.ts".into(),
                });
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                event.set(
                    "event.id",
                    json!(
                        event
                            .get("zeek.session_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("zeek.x509.certificate.signature_algorithm")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.x509.signature_algorithm", v)?;
            }

            let _cond = { event.has_value("file.x509.signature_algorithm") };
            if _cond {
                // Painless script
                // Source: String algo = params.get(ctx.file.x509.signature_algorithm);\nif (algo != null) {\n  ctx.file.x509.signature_algorithm = algo;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String algo = params.get(ctx.file.x509.signature_algorithm);\nif (algo != null) {\n  ctx.file.x509.signature_algorithm = algo;\n}\n"#
                    ),
                    cached_params!(
                        "{\"md2WithRSAEncryption\":\"MD2-RSA\",\"md5WithRSAEncryption\":\"MD5-RSA\",\"sha-1WithRSAEncryption\":\"SHA1-RSA\",\"sha256WithRSAEncryption\":\"SHA256-RSA\",\"sha384WithRSAEncryption\":\"SHA384-RSA\",\"sha512WithRSAEncryption\":\"SHA512-RSA\",\"dsaWithSha1\":\"DSA-SHA1\",\"dsaWithSha256\":\"DSA-SHA256\",\"ecdsa-with-SHA1\":\"ECDSA-SHA1\",\"ecdsa-with-SHA256\":\"ECDSA-SHA256\",\"ecdsa-with-SHA384\":\"ECDSA-SHA384\",\"ecdsa-with-SHA512\":\"ECDSA-SHA512\",\"id-Ed25519\":\"Ed25519\"}"
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("zeek.x509.certificate.key.algorithm")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.x509.public_key_algorithm", v)?;
            }

            if event.has_value("zeek.x509.certificate.key.length") {
                if let Some(val) = event.get("zeek.x509.certificate.key.length") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "zeek.x509.certificate.key.length".into(),
                            message,
                        }
                    })?;
                    event.set("file.x509.public_key_size", converted)?;
                }
            }

            dot_expand(event, "zeek.x509", "certificate.exponent")?;

            if event.has_value("zeek.x509.certificate.exponent") {
                if let Some(val) = event.get("zeek.x509.certificate.exponent") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "zeek.x509.certificate.exponent".into(),
                            message,
                        }
                    })?;
                    event.set("file.x509.public_key_exponent", converted)?;
                }
            }

            dot_expand(event, "zeek.x509", "certificate.serial")?;

            let v = json!(
                event
                    .get("zeek.x509.certificate.serial")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.x509.serial_number", v)?;
            }

            dot_expand(event, "zeek.x509", "certificate.version")?;

            let v = json!(
                event
                    .get("zeek.x509.certificate.version")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.x509.version_number", v)?;
            }

            dot_expand(event, "zeek.x509", "san.dns")?;

            if event.has_value("zeek.x509.san.dns") {
                foreach_array(event, "zeek.x509.san.dns", |event| {
                    event.append(
                        "file.x509.alternative_names",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            dot_expand(event, "zeek.x509", "san.uri")?;

            if event.has_value("zeek.x509.san.uri") {
                foreach_array(event, "zeek.x509.san.uri", |event| {
                    event.append(
                        "file.x509.alternative_names",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            dot_expand(event, "zeek.x509", "san.email")?;

            if event.has_value("zeek.x509.san.email") {
                foreach_array(event, "zeek.x509.san.email", |event| {
                    event.append(
                        "file.x509.alternative_names",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            dot_expand(event, "zeek.x509", "san.ip")?;

            if event.has_value("zeek.x509.san.ip") {
                foreach_array(event, "zeek.x509.san.ip", |event| {
                    event.append(
                        "file.x509.alternative_names",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            dot_expand(event, "zeek.x509", "san.other_fields")?;

            if event.has_value("zeek.x509.san.other_fields") {
                foreach_array(event, "zeek.x509.san.other_fields", |event| {
                    event.append(
                        "file.x509.alternative_names",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("zeek.x509.certificate.valid.from") };
            if _cond {
                if let Some(date_str) = event.get_as_string("zeek.x509.certificate.valid.from") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("zeek.x509.certificate.valid.from", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zeek.x509.certificate.valid.from".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let v = json!(
                event
                    .get("zeek.x509.certificate.valid.from")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.x509.not_before", v)?;
            }

            let _cond = { event.has_value("zeek.x509.certificate.valid.until") };
            if _cond {
                if let Some(date_str) = event.get_as_string("zeek.x509.certificate.valid.until") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("zeek.x509.certificate.valid.until", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zeek.x509.certificate.valid.until".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let v = json!(
                event
                    .get("zeek.x509.certificate.valid.until")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.x509.not_after", v)?;
            }

            if event.has_value("zeek.x509.certificate.iss") {
                gsub_field(
                    event,
                    "zeek.x509.certificate.iss",
                    "zeek.x509.certificate.iss",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.x509.certificate.iss") {
                if let Some(kv_str) = event.get_string("zeek.x509.certificate.iss") {
                    let mut kv_gap = false;
                    for pair in kv_str.split(",") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "zeek.x509.certificate.iss".into(),
                                split: "=".into(),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(
                                    event,
                                    &format!("zeek.x509.certificate.issuer.{}", key),
                                    value,
                                )?;
                            }
                        }
                    }
                }
            }

            event.remove("zeek.x509.certificate.iss");

            let _cond = {
                event
                    .get("zeek.x509.certificate.issuer.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.issuer.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.issuer.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.issuer.C") {
                event.rename(
                    "zeek.x509.certificate.issuer.C",
                    "zeek.x509.certificate.issuer.country",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.issuer.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.issuer.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.issuer.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.issuer.CN") {
                event.rename(
                    "zeek.x509.certificate.issuer.CN",
                    "zeek.x509.certificate.issuer.common_name",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.issuer.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.issuer.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.issuer.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.issuer.L") {
                event.rename(
                    "zeek.x509.certificate.issuer.L",
                    "zeek.x509.certificate.issuer.locality",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.issuer.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.issuer.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.issuer.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.issuer.O") {
                event.rename(
                    "zeek.x509.certificate.issuer.O",
                    "zeek.x509.certificate.issuer.organization",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.issuer.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.issuer.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.issuer.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.issuer.OU") {
                event.rename(
                    "zeek.x509.certificate.issuer.OU",
                    "zeek.x509.certificate.issuer.organizational_unit",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.issuer.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.issuer.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.issuer.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.issuer.ST") {
                event.rename(
                    "zeek.x509.certificate.issuer.ST",
                    "zeek.x509.certificate.issuer.state",
                )?;
            }

            if event.has_value("zeek.x509.certificate.sub") {
                gsub_field(
                    event,
                    "zeek.x509.certificate.sub",
                    "zeek.x509.certificate.sub",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.x509.certificate.sub") {
                if let Some(kv_str) = event.get_string("zeek.x509.certificate.sub") {
                    let mut kv_gap = false;
                    for pair in kv_str.split(",") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "zeek.x509.certificate.sub".into(),
                                split: "=".into(),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(
                                    event,
                                    &format!("zeek.x509.certificate.subject.{}", key),
                                    value,
                                )?;
                            }
                        }
                    }
                }
            }

            event.remove("zeek.x509.certificate.sub");

            let _cond = {
                event
                    .get("zeek.x509.certificate.subject.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.subject.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.subject.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.subject.C") {
                event.rename(
                    "zeek.x509.certificate.subject.C",
                    "zeek.x509.certificate.subject.country",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.subject.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.subject.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.subject.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.subject.CN") {
                event.rename(
                    "zeek.x509.certificate.subject.CN",
                    "zeek.x509.certificate.subject.common_name",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.subject.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.subject.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.subject.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.subject.L") {
                event.rename(
                    "zeek.x509.certificate.subject.L",
                    "zeek.x509.certificate.subject.locality",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.subject.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.subject.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.subject.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.subject.O") {
                event.rename(
                    "zeek.x509.certificate.subject.O",
                    "zeek.x509.certificate.subject.organization",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.subject.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.subject.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.subject.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.subject.OU") {
                event.rename(
                    "zeek.x509.certificate.subject.OU",
                    "zeek.x509.certificate.subject.organizational_unit",
                )?;
            }

            let _cond = {
                event
                    .get("zeek.x509.certificate.subject.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "file.x509.subject.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.x509.certificate.subject.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("zeek.x509.certificate.subject.ST") {
                event.rename(
                    "zeek.x509.certificate.subject.ST",
                    "zeek.x509.certificate.subject.state",
                )?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
