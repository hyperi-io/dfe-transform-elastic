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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("observer.vendor", json!("QNAP"))?;

            event.set("observer.product", json!("NAS"))?;

            event.set("observer.type", json!("nas"))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>))?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:(?:%{IP:host.ip}|%{HOSTNAME:host.name})) (?:%{PROG:process.name}(?:\\[%{POSINT:process.pid:int}\\])?): (?P<event_provider>(?:(event log|conn log))): %{GREEDYDATA:_tmp.message}
                if !cached_grok_mapped!("^((?:<%{NONNEGINT:log.syslog.priority:long}>))?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:(?:%{IP:host.ip}|%{HOSTNAME:host.name})) (?:%{PROG:process.name}(?:\\[%{POSINT:process.pid:int}\\])?): (?P<event_provider>(?:(event log|conn log))): %{GREEDYDATA:_tmp.message}", [("event_provider", "event.provider")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            let _cond = {
                event.has_value("_tmp.tz_offset")
                    && event.get_str("_tmp.tz_offset") != Some("local")
            };
            if _cond {
                event.set(
                    "event.timezone",
                    json!(
                        event
                            .get("_tmp.tz_offset")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss"],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            if let Some(input) = event.get_string("_tmp.message") {
                // Grok pattern: ^(?:Users: (---|(%{DATA:user.domain}\\\\)?%{DATA:user.name}), Source IP: (---|127.0.0.1|%{IP:source.address}), Computer name: (---|%{HOSTNAME:source.domain})), Application: %{DATA:qnap.nas.application}, Category: %{DATA:qnap.nas.category}, Content: %{DATA:message}$
                // Grok pattern: ^(?:Users: (---|(%{DATA:user.domain}\\\\)?%{DATA:user.name}), Source IP: (---|127.0.0.1|%{IP:source.address}), Computer name: (---|%{HOSTNAME:source.domain})), Content: %{DATA:message}$
                // Grok pattern: ^(?:Users: (---|(%{DATA:user.domain}\\\\)?%{DATA:user.name}), Source IP: (---|127.0.0.1|%{IP:source.address}), Computer name: (---|%{HOSTNAME:source.domain})), Connection type: %{DATA:qnap.nas.connection_type}, Accessed resources: (?:(\\[%{DATA:qnap.nas.application}\\] )?(---|(?P<qnap_nas_file_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)+))|%{DATA:qnap.nas.application})), Action: %{DATA:event.action}$
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "^(?:Users: (---|(%{DATA:user.domain}\\\\)?%{DATA:user.name}), Source IP: (---|127.0.0.1|%{IP:source.address}), Computer name: (---|%{HOSTNAME:source.domain})), Application: %{DATA:qnap.nas.application}, Category: %{DATA:qnap.nas.category}, Content: %{DATA:message}$"
                        ),
                        cached_grok!(
                            "^(?:Users: (---|(%{DATA:user.domain}\\\\)?%{DATA:user.name}), Source IP: (---|127.0.0.1|%{IP:source.address}), Computer name: (---|%{HOSTNAME:source.domain})), Content: %{DATA:message}$"
                        ),
                        cached_grok_mapped!(
                            "^(?:Users: (---|(%{DATA:user.domain}\\\\)?%{DATA:user.name}), Source IP: (---|127.0.0.1|%{IP:source.address}), Computer name: (---|%{HOSTNAME:source.domain})), Connection type: %{DATA:qnap.nas.connection_type}, Accessed resources: (?:(\\[%{DATA:qnap.nas.application}\\] )?(---|(?P<qnap_nas_file_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)+))|%{DATA:qnap.nas.application})), Action: %{DATA:event.action}$",
                            [("qnap_nas_file_path", "qnap.nas.file.path")]
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^\\[Shared Folders\\] (?P<event_action>(?:(Created|Deleted) %{DATA})) \"%{DATA:qnap.nas.file.path}\"\\.$
                        // Grok pattern: ^\\[User Groups\\] (?P<event_action>(?:(Created|Deleted) %{DATA})) \"%{DATA:group.name}\"\\.$
                        // Grok pattern: ^\\[Users\\] (?:((?P<event_action>(?:(Created|Deleted) %{DATA}))|%{DATA:event.action} of user)) \"%{DATA:user.target.name}\"\\.$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^\\[Shared Folders\\] (?P<event_action>(?:(Created|Deleted) %{DATA})) \"%{DATA:qnap.nas.file.path}\"\\.$",
                                    [("event_action", "event.action")]
                                ),
                                cached_grok_mapped!(
                                    "^\\[User Groups\\] (?P<event_action>(?:(Created|Deleted) %{DATA})) \"%{DATA:group.name}\"\\.$",
                                    [("event_action", "event.action")]
                                ),
                                cached_grok_mapped!(
                                    "^\\[Users\\] (?:((?P<event_action>(?:(Created|Deleted) %{DATA}))|%{DATA:event.action} of user)) \"%{DATA:user.target.name}\"\\.$",
                                    [("event_action", "event.action")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("qnap.nas.file.path") {
                    if let Some(input) = event.get_string("qnap.nas.file.path") {
                        // Grok pattern: (?P<file_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)*)) -> (?P<qnap_nas_file_new_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)*))
                        // Grok pattern: (?P<file_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)*))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "(?P<file_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)*)) -> (?P<qnap_nas_file_new_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)*))",
                                    [
                                        ("file_path", "file.path"),
                                        ("qnap_nas_file_new_path", "qnap.nas.file.new_path")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "(?P<file_path>(?:[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*(\\/[_%\\(\\)!$@:.,+~\\-\\s[:alnum:]]*)*))",
                                    [("file_path", "file.path")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("file.path") {
                    if let Some(input) = event.get_string("file.path") {
                        // Grok pattern: \\.%{DATA:file.extension}$
                        if !cached_grok!("\\.%{DATA:file.extension}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if event.has_value("event.action") {
                gsub_field(
                    event,
                    "event.action",
                    "event.action",
                    cached_regex!("the "),
                    "",
                )?;
            }

            if event.has_value("event.action") {
                gsub_field(
                    event,
                    "event.action",
                    "event.action",
                    cached_regex!("\\s"),
                    "-",
                )?;
            }

            if event.has_value("event.provider") {
                gsub_field(
                    event,
                    "event.provider",
                    "event.provider",
                    cached_regex!("\\s"),
                    "-",
                )?;
            }

            // Painless script
            // Source: ctx.event.kind = 'event'; ctx.event.type = 'info'; if(ctx?.event?.action == null && ctx.event?.provider == 'event-log') {\n  if(ctx.event?.category == null) {\n    List list = new ArrayList();\n    ctx.event.put(\"category\", list);\n  }\n  ctx.event.category.add('configuration');\n  ctx.event.type = 'change';\n} else if (ctx?.event?.action == null) {\n  return;\n} if (params.get(ctx.event.action) == null) {\n  return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; ctx.event.type = 'info'; if(ctx?.event?.action == null && ctx.event?.provider == 'event-log') {\n  if(ctx.event?.category == null) {\n    List list = new ArrayList();\n    ctx.event.put(\"category\", list);\n  }\n  ctx.event.category.add('configuration');\n  ctx.event.type = 'change';\n} else if (ctx?.event?.action == null) {\n  return;\n} if (params.get(ctx.event.action) == null) {\n  return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"create-directory\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"read\":{\"category\":[\"file\"],\"type\":[\"access\"]},\"rename\":{\"category\":[\"file\"],\"type\":[\"change\"]},\"delete\":{\"category\":[\"file\"],\"type\":[\"deletion\"]},\"add\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"created-shared-folder\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"deleted-shared-folder\":{\"category\":[\"file\"],\"type\":[\"deletion\"]},\"created-user-group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"deleted-user-group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"changed-password\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"edited-account-profile\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"created-user\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"deleted-user\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"login-fail\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"login-success\":{\"category\":[\"authentication\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"logout\":{\"category\":[\"authentication\"],\"type\":[\"end\"]}}"
                ),
            )?;

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("_tmp");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == \"---\");\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    sentinels: vec!["---".into()],
                    ..DropPolicy::none()
                },
                None,
            );

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
