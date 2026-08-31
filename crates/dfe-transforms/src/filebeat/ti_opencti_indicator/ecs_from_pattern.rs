// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_pattern` pipeline.
pub struct EcsFromPattern;

impl Transform for EcsFromPattern {
    fn name(&self) -> &str {
        "ecs_from_pattern"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("_ingest._value") {
                // Grok pattern: file:hashes.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'
                // Grok pattern: file:hashes.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'
                // Grok pattern: file:hashes.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'
                // Grok pattern: file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'
                // Grok pattern: domain-name:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.domain}'
                // Grok pattern: hostname:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.domain}'
                // Grok pattern: url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'
                // Grok pattern: email-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'
                // Grok pattern: ipv4-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.ip}'
                // Grok pattern: ipv6-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.ip}'
                // Grok pattern: windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp_registry}'
                if !extract_first_match(
                    &[
                        cached_grok!("file:hashes.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'"),
                        cached_grok!("file:hashes.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'"),
                        cached_grok!("file:hashes.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'"),
                        cached_grok!("file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'"),
                        cached_grok!("domain-name:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.domain}'"),
                        cached_grok!("hostname:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.domain}'"),
                        cached_grok!("url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'"),
                        cached_grok!("email-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'"),
                        cached_grok!("ipv4-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.ip}'"),
                        cached_grok!("ipv6-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.ip}'"),
                        cached_grok!("windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp_registry}'"),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
            Ok(())
        })();

        let _cond = { event.has_value("threat.indicator.file.name") };
        if _cond {
        let v = json!("file");
        if !painless_is_empty_value(&v) {
                event.set("threat.indicator.file.type", v)?;
        }
        }

        let _cond = { event.has_value("threat.indicator.file.name") };
        if _cond {
            // Painless script
            // Source: def tmp_file_name = ctx.threat.indicator.file.name;\nif (tmp_file_name != null) {\n  def parts = /[\\/\\\\]/.split(tmp_file_name);\n  def name = parts[parts.length - 1];\n  if (name.contains(\".\")) {\n    def nameParts = /\\./.split(name);\n    def extension = nameParts[nameParts.length - 1];\n    if (extension.length() > 0) {\n      ctx.threat.indicator.file.extension = extension;\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def tmp_file_name = ctx.threat.indicator.file.name;\nif (tmp_file_name != null) {\n  def parts = /[\\/\\\\]/.split(tmp_file_name);\n  def name = parts[parts.length - 1];\n  if (name.contains(\".\")) {\n    def nameParts = /\\./.split(name);\n    def extension = nameParts[nameParts.length - 1];\n    if (extension.length() > 0) {\n      ctx.threat.indicator.file.extension = extension;\n    }\n  }\n}\n"#))?;
        }

        if event.has_value("threat.indicator.ip") {
            gsub_field(event, "threat.indicator.ip", "threat.indicator.ip", cached_regex!("/\\d+$"), "")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            uri_parts(event, "threat.indicator.url.original", "threat.indicator.url", true, false)?;
            Ok(())
        })();

        if event.has_value("threat.indicator.url.domain") {
            if let Some(domain_str) = event.get_string("threat.indicator.url.domain") {
                let domain = domain_str.to_string();
                event.set("threat.indicator.url.domain", json!(domain.clone()))?;
                // Public suffix list lookup for registered domain extraction
                if let Some(rd) = registered_domain_lookup(&domain) {
                    if let Some(registered) = rd.registered_domain {
                        event.set("threat.indicator.url.registered_domain", json!(registered))?;
                    }
                    event.set("threat.indicator.url.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("threat.indicator.url.subdomain", json!(sub))?;
                    }
                }
            }
        }

        if let Some(v) = event.get("threat.indicator.url.original").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("threat.indicator.url.full", v)?;
        }

        if event.has_value("_tmp_registry") {
            gsub_field(event, "_tmp_registry", "_tmp_registry", cached_regex!("\\\\\\\\"), "\\\\")?;
        }

        if event.has_value("_tmp_registry") {
            if let Some(input) = event.get_string("_tmp_registry") {
                // Grok pattern: ^((?P<tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:tmp_registry.key}$
                if !cached_grok_mapped!("^((?P<tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:tmp_registry.key}$", [("tmp_registry_hive", "tmp_registry.hive")]).extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
        }

        let _cond = { event.has_value("tmp_registry.hive") };
        if _cond {
            // Painless script
            // Source: def name = ctx.tmp_registry.hive.toUpperCase();\nctx.tmp_registry.hive = params.getOrDefault(name, name);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"def name = ctx.tmp_registry.hive.toUpperCase();\nctx.tmp_registry.hive = params.getOrDefault(name, name);\n"#), cached_params!("{\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_USERS\":\"HKU\",\"HKEY_CURRENT_CONFIG\":\"HKCC\"}"))?;
        }

            if event.has_value("tmp_registry") {
                event.rename("tmp_registry", "threat.indicator.registry")?;
            }

            event.remove("_tmp_registry");

            // Painless script
            // Source: if (ctx.threat?.indicator?.file?.name != null && !(ctx.threat.indicator.file.name instanceof List)) {\n  ctx.threat.indicator.file.name = [ctx.threat.indicator.file.name];\n}\nif (ctx.threat?.indicator?.file?.extension != null && !(ctx.threat.indicator.file.extension instanceof List)) {\n  ctx.threat.indicator.file.extension = [ctx.threat.indicator.file.extension];\n}\nif (ctx.threat?.indicator?.email?.address != null && !(ctx.threat.indicator.email.address instanceof List)) {\n  ctx.threat.indicator.email.address = [ctx.threat.indicator.email.address];\n}\nif (ctx.threat?.indicator?.ip != null && !(ctx.threat.indicator.ip instanceof List)) {\n  ctx.threat.indicator.ip = [ctx.threat.indicator.ip];\n}\nif (ctx.threat?.indicator?.url != null && !(ctx.threat.indicator.url instanceof List)) {\n  ctx.threat.indicator.url = [ctx.threat.indicator.url];\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.threat?.indicator?.file?.name != null && !(ctx.threat.indicator.file.name instanceof List)) {\n  ctx.threat.indicator.file.name = [ctx.threat.indicator.file.name];\n}\nif (ctx.threat?.indicator?.file?.extension != null && !(ctx.threat.indicator.file.extension instanceof List)) {\n  ctx.threat.indicator.file.extension = [ctx.threat.indicator.file.extension];\n}\nif (ctx.threat?.indicator?.email?.address != null && !(ctx.threat.indicator.email.address instanceof List)) {\n  ctx.threat.indicator.email.address = [ctx.threat.indicator.email.address];\n}\nif (ctx.threat?.indicator?.ip != null && !(ctx.threat.indicator.ip instanceof List)) {\n  ctx.threat.indicator.ip = [ctx.threat.indicator.ip];\n}\nif (ctx.threat?.indicator?.url != null && !(ctx.threat.indicator.url instanceof List)) {\n  ctx.threat.indicator.url = [ctx.threat.indicator.url];\n}\n"#))?;

        Ok(TransformResult::Continue)
    }
}
