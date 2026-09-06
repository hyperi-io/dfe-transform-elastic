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
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "message", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to parse JSON: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.original", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            if let Some(v) = event.get("_ingest.timestamp").cloned() {
                event.set("event.ingested", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.TIMESTAMP_DERIVED") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.TIMESTAMP_DERIVED".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to parse TIMESTAMP_DERIVED field: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def apex = ctx.salesforce?.apex ?: [:];\ndef json = ctx.json ?: [:];\n\ndef fieldsToConvert = [\n  'CALLOUT_TIME': 'callout_time',\n  'CPU_TIME': 'cpu_time',\n  'DB_CPU_TIME': 'db_cpu_time',\n  'DB_TOTAL_TIME': 'db_total_time',\n  'EXECUTE_MS': 'execute_ms',\n  'FETCH_MS': 'fetch_ms',\n  'LIMIT_USAGE_PERCENT': 'limit_usage_pct',\n  'THROUGHPUT': 'throughput',\n  'RUN_TIME': 'run_time'\n];\nfor (def entry : fieldsToConvert.entrySet()) {\n  if (json[entry.getKey()] != null) {\n    def value = json[entry.getKey()];\n    if (value instanceof String && value.length() > 0) {\n      apex[entry.getValue()] = Float.parseFloat(value);\n    }\n  }\n}\n\ndef longFields = [\n  'DB_BLOCKS': 'db_blocks',\n  'LIMIT': 'limit',\n  'NUMBER_FIELDS': 'fields_count',\n  'NUMBER_SOQL_QUERIES': 'soql_queries_count',\n  'OFFSET': 'offset',\n  'ROWS': 'rows_total',\n  'ROWS_FETCHED': 'rows_fetched',\n  'ROWS_PROCESSED': 'rows_processed'\n];\nfor (def entry : longFields.entrySet()) {\n  if (json[entry.getKey()] != null) {\n    def value = json[entry.getKey()];\n    if (value instanceof String && value.length() > 0) {\n      apex[entry.getValue()] = Long.parseLong(value);\n    }\n  }\n}\n\ndef fieldsToRename = [\n  'ACTION': 'action',\n  'CLASS_NAME': 'class_name',\n  'CLIENT_NAME': 'client_name',\n  'ENTITY': 'entity',\n  'ENTITY_NAME': 'entity_name',\n  'ENTRY_POINT': 'entry_point',\n  'EVENT_TYPE': 'event_type',\n  'FILTER': 'filter',\n  'IS_LONG_RUNNING_REQUEST': 'is_long_running_request',\n  'LOGIN_KEY': 'login_key',\n  'MEDIA_TYPE': 'media_type',\n  'MESSAGE': 'message',\n  'METHOD_NAME': 'method_name',\n  'ORDERBY': 'orderby',\n  'ORGANIZATION_ID': 'organization_id',\n  'QUERY': 'query',\n  'QUIDDITY': 'quiddity',\n  'REQUEST_ID': 'request_id',\n  'REQUEST_STATUS': 'request_status',\n  'SELECT': 'select',\n  'SUBQUERIES': 'subqueries',\n  'TRIGGER_ID': 'trigger_id',\n  'TRIGGER_NAME': 'trigger_name',\n  'TRIGGER_TYPE': 'trigger_type',\n  'TYPE': 'type',\n  'URI': 'uri',\n  'URI_ID_DERIVED': 'uri_derived_id',\n  'USER_AGENT': 'user_agent',\n  'USER_ID_DERIVED': 'user_id_derived'\n];\nfor (def entry : fieldsToRename.entrySet()) {\n  if (json[entry.getKey()] != null) {\n    apex[entry.getValue()] = json[entry.getKey()];\n  }\n}\n\nctx.salesforce = ctx.salesforce ?: [:];\nctx.salesforce.apex = apex;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def apex = ctx.salesforce?.apex ?: [:];\ndef json = ctx.json ?: [:];\n\ndef fieldsToConvert = [\n  'CALLOUT_TIME': 'callout_time',\n  'CPU_TIME': 'cpu_time',\n  'DB_CPU_TIME': 'db_cpu_time',\n  'DB_TOTAL_TIME': 'db_total_time',\n  'EXECUTE_MS': 'execute_ms',\n  'FETCH_MS': 'fetch_ms',\n  'LIMIT_USAGE_PERCENT': 'limit_usage_pct',\n  'THROUGHPUT': 'throughput',\n  'RUN_TIME': 'run_time'\n];\nfor (def entry : fieldsToConvert.entrySet()) {\n  if (json[entry.getKey()] != null) {\n    def value = json[entry.getKey()];\n    if (value instanceof String && value.length() > 0) {\n      apex[entry.getValue()] = Float.parseFloat(value);\n    }\n  }\n}\n\ndef longFields = [\n  'DB_BLOCKS': 'db_blocks',\n  'LIMIT': 'limit',\n  'NUMBER_FIELDS': 'fields_count',\n  'NUMBER_SOQL_QUERIES': 'soql_queries_count',\n  'OFFSET': 'offset',\n  'ROWS': 'rows_total',\n  'ROWS_FETCHED': 'rows_fetched',\n  'ROWS_PROCESSED': 'rows_processed'\n];\nfor (def entry : longFields.entrySet()) {\n  if (json[entry.getKey()] != null) {\n    def value = json[entry.getKey()];\n    if (value instanceof String && value.length() > 0) {\n      apex[entry.getValue()] = Long.parseLong(value);\n    }\n  }\n}\n\ndef fieldsToRename = [\n  'ACTION': 'action',\n  'CLASS_NAME': 'class_name',\n  'CLIENT_NAME': 'client_name',\n  'ENTITY': 'entity',\n  'ENTITY_NAME': 'entity_name',\n  'ENTRY_POINT': 'entry_point',\n  'EVENT_TYPE': 'event_type',\n  'FILTER': 'filter',\n  'IS_LONG_RUNNING_REQUEST': 'is_long_running_request',\n  'LOGIN_KEY': 'login_key',\n  'MEDIA_TYPE': 'media_type',\n  'MESSAGE': 'message',\n  'METHOD_NAME': 'method_name',\n  'ORDERBY': 'orderby',\n  'ORGANIZATION_ID': 'organization_id',\n  'QUERY': 'query',\n  'QUIDDITY': 'quiddity',\n  'REQUEST_ID': 'request_id',\n  'REQUEST_STATUS': 'request_status',\n  'SELECT': 'select',\n  'SUBQUERIES': 'subqueries',\n  'TRIGGER_ID': 'trigger_id',\n  'TRIGGER_NAME': 'trigger_name',\n  'TRIGGER_TYPE': 'trigger_type',\n  'TYPE': 'type',\n  'URI': 'uri',\n  'URI_ID_DERIVED': 'uri_derived_id',\n  'USER_AGENT': 'user_agent',\n  'USER_ID_DERIVED': 'user_id_derived'\n];\nfor (def entry : fieldsToRename.entrySet()) {\n  if (json[entry.getKey()] != null) {\n    apex[entry.getValue()] = json[entry.getKey()];\n  }\n}\n\nctx.salesforce = ctx.salesforce ?: [:];\nctx.salesforce.apex = apex;\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to process Apex fields: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("salesforce.apex.user_agent") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def userAgent = ctx.salesforce.apex.user_agent.toString();\nif (userAgent.length() >= 3) {\n  def code = userAgent.substring(0, 3);\n  if (params.user_agent_map.containsKey(code)) {\n    ctx.salesforce.apex.user_agent = params.user_agent_map[code];\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def userAgent = ctx.salesforce.apex.user_agent.toString();\nif (userAgent.length() >= 3) {\n  def code = userAgent.substring(0, 3);\n  if (params.user_agent_map.containsKey(code)) {\n    ctx.salesforce.apex.user_agent = params.user_agent_map[code];\n  }\n}\n"#
                        ),
                        cached_params!(
                            "{\"user_agent_map\":{\"100\":\"Internet Explorer\",\"110\":\"Firefox\",\"130\":\"Chrome\",\"140\":\"Safari\",\"150\":\"Opera\",\"160\":\"Android\",\"170\":\"Netscape\",\"180\":\"Webkit\",\"190\":\"Gecko\",\"230\":\"Blackberry\",\"240\":\"Good Access\",\"999\":\"Unknown\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.apex.user_agent: {}",
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.get_str("salesforce.apex.event_type") != Some("ApexExecution") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("event.type", json!("connection"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("salesforce.apex.event_type") != Some("ApexTrigger")
                    && event.get_str("salesforce.apex.event_type") != Some("ApexExecution")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("event.category", json!("network"))?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("salesforce.apex.event_type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def eventType = ctx.salesforce.apex.event_type?.toLowerCase();\nif (eventType != null && params.event_action_map.containsKey(eventType)) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.action = params.event_action_map[eventType];\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def eventType = ctx.salesforce.apex.event_type?.toLowerCase();\nif (eventType != null && params.event_action_map.containsKey(eventType)) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.action = params.event_action_map[eventType];\n}\n"#
                        ),
                        cached_params!(
                            "{\"event_action_map\":{\"apexcallout\":\"apex-callout\",\"apextrigger\":\"apex-trigger\",\"apexexecution\":\"apex-execution\",\"apexrestapi\":\"apex-rest\",\"apexsoap\":\"apex-soap\",\"externalcustomapexcallout\":\"apex-external-custom-callout\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set event.action from salesforce.apex.event_type: {}",
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def apex = ctx.salesforce?.apex ?: [:];\ndef json = ctx.json ?: [:];\ndef event = ctx.event ?: [:];\n\nif (apex.event_type == \"ApexCallout\" && json.TIME != null) {\n  event.duration = Float.parseFloat(json.TIME);\n} else if (json.EXEC_TIME != null && (apex.event_type == \"ApexTrigger\" || apex.event_type == \"ApexExecution\")) {\n  event.duration = Float.parseFloat(json.EXEC_TIME);\n} else if (apex.run_time != null && (apex.event_type == \"ApexRestApi\" || apex.event_type == \"ApexSoap\")) {\n  event.duration = apex.run_time;\n} else if (json.TOTAL_MS != null && apex.event_type == \"ExternalCustomApexCallout\") {\n  event.duration = Float.parseFloat(json.TOTAL_MS);\n}\n\nif ((json.SUCCESS != null && json.SUCCESS == \"1\") || (json.STATUS != null && json.STATUS == \"1\")) {\n  event.outcome = \"success\";\n} else if ((json.SUCCESS != null && json.SUCCESS != \"1\") || (json.STATUS != null && json.STATUS != \"1\")) {\n  event.outcome = \"failure\";\n}\n\nif (json.URL != null && apex.event_type == \"ApexCallout\") {\n  event.url = json.URL;\n} else if (apex.uri != null && apex.event_type != \"ApexCallout\" && apex.event_type != \"ExternalCustomApexCallout\") {\n  event.url = apex.uri;\n}\n\nctx.event = event;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def apex = ctx.salesforce?.apex ?: [:];\ndef json = ctx.json ?: [:];\ndef event = ctx.event ?: [:];\n\nif (apex.event_type == \"ApexCallout\" && json.TIME != null) {\n  event.duration = Float.parseFloat(json.TIME);\n} else if (json.EXEC_TIME != null && (apex.event_type == \"ApexTrigger\" || apex.event_type == \"ApexExecution\")) {\n  event.duration = Float.parseFloat(json.EXEC_TIME);\n} else if (apex.run_time != null && (apex.event_type == \"ApexRestApi\" || apex.event_type == \"ApexSoap\")) {\n  event.duration = apex.run_time;\n} else if (json.TOTAL_MS != null && apex.event_type == \"ExternalCustomApexCallout\") {\n  event.duration = Float.parseFloat(json.TOTAL_MS);\n}\n\nif ((json.SUCCESS != null && json.SUCCESS == \"1\") || (json.STATUS != null && json.STATUS == \"1\")) {\n  event.outcome = \"success\";\n} else if ((json.SUCCESS != null && json.SUCCESS != \"1\") || (json.STATUS != null && json.STATUS != \"1\")) {\n  event.outcome = \"failure\";\n}\n\nif (json.URL != null && apex.event_type == \"ApexCallout\") {\n  event.url = json.URL;\n} else if (apex.uri != null && apex.event_type != \"ApexCallout\" && apex.event_type != \"ExternalCustomApexCallout\") {\n  event.url = apex.uri;\n}\n\nctx.event = event;\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to process event fields: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.USER_ID") {
                    event.rename("json.USER_ID", "user.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "user.roles",
                    json!(
                        event
                            .get("json.USER_TYPE")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.CLIENT_IP")
                    && event.get_str("json.CLIENT_IP") != Some("Salesforce.com IP")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.CLIENT_IP") {
                        event.rename("json.CLIENT_IP", "source.ip")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get_str("source.ip") != Some("Salesforce.com IP")
            };
            if _cond {
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
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.METHOD") {
                    event.rename("json.METHOD", "http.request.method")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.REQUEST_SIZE") {
                    if let Some(val) = event.get("json.REQUEST_SIZE") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.REQUEST_SIZE".into(),
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
                if event.has_value("json.RESPONSE_SIZE") {
                    if let Some(val) = event.get("json.RESPONSE_SIZE") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.RESPONSE_SIZE".into(),
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
                if event.has_value("json.STATUS_CODE") {
                    if let Some(val) = event.get("json.STATUS_CODE") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.STATUS_CODE".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.status_code", converted)?;
                    }
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return ((Map) object).isEmpty();\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return ((List) object).isEmpty();\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        nulls: true,
                        empty_strings: true,
                        empty_collections: true,
                        prune_lists: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to drop empty fields: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json");
                event.remove("message");
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
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set(
                    "error.type",
                    json!(
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
