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
            let _cond = { event.has_value("error.message") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            event.rename("created", "event.created")?;

            event.rename("id", "event.id")?;

            event.set(
                "threat.feed.dashboard_id",
                json!("ti_opencti-83b2bef0-591c-11ee-ba5f-49a63bb985cd"),
            )?;

            event.set(
                "threat.feed.description",
                json!("Indicator data from OpenCTI"),
            )?;

            event.set("threat.feed.name", json!("OpenCTI"))?;

            event.set(
                "threat.feed.reference",
                json!("https://docs.opencti.io/latest/usage/overview/"),
            )?;

            event.set(
                "threat.indicator.reference",
                json!(format!(
                    "{}/dashboard/observations/indicators/{}",
                    event
                        .get("_conf.url")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("event.id")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;

            event.remove("_conf");

            event.rename("standard_id", "opencti.indicator.standard_id")?;

            event.rename("is_inferred", "opencti.indicator.is_inferred")?;

            event.rename("revoked", "opencti.indicator.revoked")?;

            event.set("threat.indicator.confidence", json!("Not Specified"))?;

            // Painless script
            // Source: if (ctx.confidence == null) {\n  ctx.threat.indicator.confidence = 'Not Specified';\n} else if (ctx.confidence == 0) {\n  ctx.threat.indicator.confidence = 'None';\n} else if (1 <= ctx.confidence && ctx.confidence <= 29) {\n  ctx.threat.indicator.confidence = 'Low';\n} else if (30 <= ctx.confidence && ctx.confidence <= 69) {\n  ctx.threat.indicator.confidence = 'Medium';\n} else if (70 <= ctx.confidence && ctx.confidence <= 100) {\n  ctx.threat.indicator.confidence = 'High';\n} else {\n  ctx.threat.indicator.confidence = 'Not Specified';\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.confidence == null) {\n  ctx.threat.indicator.confidence = 'Not Specified';\n} else if (ctx.confidence == 0) {\n  ctx.threat.indicator.confidence = 'None';\n} else if (1 <= ctx.confidence && ctx.confidence <= 29) {\n  ctx.threat.indicator.confidence = 'Low';\n} else if (30 <= ctx.confidence && ctx.confidence <= 69) {\n  ctx.threat.indicator.confidence = 'Medium';\n} else if (70 <= ctx.confidence && ctx.confidence <= 100) {\n  ctx.threat.indicator.confidence = 'High';\n} else {\n  ctx.threat.indicator.confidence = 'Not Specified';\n}\n"#
                ),
            )?;

            if event.remove("confidence").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "confidence".into(),
                });
            }

            event.rename("lang", "opencti.indicator.lang")?;

            event.rename("modified", "threat.indicator.modified_at")?;

            if event.has_value("updated_at") {
                event.rename("updated_at", "opencti.indicator.updated_at")?;
            }

            event.rename("pattern_type", "opencti.indicator.pattern_type")?;

            let _cond = { event.has_value("pattern_version") };
            if _cond {
                if event.has_value("pattern_version") {
                    event.rename("pattern_version", "opencti.indicator.pattern_version")?;
                }
            }

            event.remove("pattern_version");

            event.rename("pattern", "opencti.indicator.pattern")?;

            event.rename("name", "threat.indicator.name")?;

            event.rename("description", "threat.indicator.description")?;

            let _cond = {
                !event.has_value("threat.indicator.description")
                    || event
                        .get("threat.indicator.description")
                        .is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        })
            };
            if _cond {
                if event.remove("threat.indicator.description").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator.description".into(),
                    });
                }
            }

            event.rename("valid_from", "opencti.indicator.valid_from")?;

            event.rename("valid_until", "opencti.indicator.valid_until")?;

            // Painless script
            // Source: if (ctx.opencti.indicator.revoked == true &&\n    ctx.threat.indicator.modified_at.compareTo(ctx.opencti.indicator.valid_until) < 0) {\n    ctx.opencti.indicator.invalid_or_revoked_from = ctx.threat.indicator.modified_at;\n} else {\n    // valid_until always has a value, will be epoch + 10^14 ms if no other value\n    ctx.opencti.indicator.invalid_or_revoked_from = ctx.opencti.indicator.valid_until;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.opencti.indicator.revoked == true &&\n    ctx.threat.indicator.modified_at.compareTo(ctx.opencti.indicator.valid_until) < 0) {\n    ctx.opencti.indicator.invalid_or_revoked_from = ctx.threat.indicator.modified_at;\n} else {\n    // valid_until always has a value, will be epoch + 10^14 ms if no other value\n    ctx.opencti.indicator.invalid_or_revoked_from = ctx.opencti.indicator.valid_until;\n}\n"#
                ),
            )?;

            event.rename("x_opencti_score", "opencti.indicator.score")?;

            event.rename("x_opencti_detection", "opencti.indicator.detection")?;

            event.rename("x_opencti_main_observable_type", "threat.indicator.type")?;

            // Painless script, resolved to its runners at generation time
            // Source: String type = ctx.threat.indicator.type;\ntype = type.toLowerCase();\ntype = type.replace('stixfile', 'file');\nctx.threat.indicator.type = type;\n
            string_ops(
                event,
                &StringOps::new(
                    "threat.indicator.type",
                    "threat.indicator.type",
                    vec![
                        StringOp::Lower,
                        StringOp::Replace {
                            from: "stixfile".into(),
                            to: "file".into(),
                        },
                    ],
                ),
            );

            if event.has_value("createdBy.name") {
                event.rename("createdBy.name", "threat.indicator.provider")?;
            }

            if event.has_value("createdBy.identity_class") {
                event.rename(
                    "createdBy.identity_class",
                    "opencti.indicator.creator_identity_class",
                )?;
            }

            event.remove("createdBy");

            let _cond = {
                event.has_value("objectMarking")
                    && event.get("objectMarking").is_some_and(|v| !match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
                    && event.get_str("objectMarking.0.definition_type") == Some("TLP")
            };
            if _cond {
                gsub_field(
                    event,
                    "objectMarking.0.definition",
                    "threat.indicator.marking.tlp",
                    cached_regex!("^TLP:"),
                    "",
                )?;
            }

            if event.remove("objectMarking").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "objectMarking".into(),
                });
            }

            if event.has_value("objectLabel") {
                foreach_array(event, "objectLabel", |event| {
                    event.append_unique(
                        "tags",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.remove("objectLabel").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "objectLabel".into(),
                });
            }

            if event.has_value("killChainPhases") {
                foreach_array(event, "killChainPhases", |event| {
                    event.append(
                        "opencti.indicator.kill_chain_phase",
                        json!(format!(
                            "[{}] {}",
                            event
                                .get("_ingest._value.kill_chain_name")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest._value.phase_name")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    Ok(())
                })?;
            }

            if event.remove("killChainPhases").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "killChainPhases".into(),
                });
            }

            // Painless script
            // Source: ArrayList edges = ctx.externalReferences.edges;\nif (edges == null) {\n  return;\n}\nfor (int i = 0; i < edges.length; i++) {\n  if (!ctx.opencti?.indicator?.containsKey('external_reference') == true) {\n    if (!ctx.containsKey('opencti')) {\n      ctx.opencti = [:];\n    }\n    if (!ctx.opencti.containsKey('indicator')) {\n      ctx.opencti.indicator = [:];\n    }\n    if (!ctx.opencti.indicator.containsKey('external_reference')) {\n      ctx.opencti.indicator.external_reference = [];\n    }\n  }\n  def newNode = [:];\n  for (def key : edges[i]['node'].keySet()) {\n    if (edges[i]['node'][key] != null) {\n      newNode[key] = edges[i]['node'][key];\n    }\n  }\n  ctx.opencti.indicator.external_reference.add(newNode);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ArrayList edges = ctx.externalReferences.edges;\nif (edges == null) {\n  return;\n}\nfor (int i = 0; i < edges.length; i++) {\n  if (!ctx.opencti?.indicator?.containsKey('external_reference') == true) {\n    if (!ctx.containsKey('opencti')) {\n      ctx.opencti = [:];\n    }\n    if (!ctx.opencti.containsKey('indicator')) {\n      ctx.opencti.indicator = [:];\n    }\n    if (!ctx.opencti.indicator.containsKey('external_reference')) {\n      ctx.opencti.indicator.external_reference = [];\n    }\n  }\n  def newNode = [:];\n  for (def key : edges[i]['node'].keySet()) {\n    if (edges[i]['node'][key] != null) {\n      newNode[key] = edges[i]['node'][key];\n    }\n  }\n  ctx.opencti.indicator.external_reference.add(newNode);\n}\n"#
                ),
            )?;

            if event.remove("externalReferences").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "externalReferences".into(),
                });
            }

            if event.has_value("observables.pageInfo.globalCount") {
                event.rename(
                    "observables.pageInfo.globalCount",
                    "opencti.indicator.observables_count",
                )?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    event.remove("_ingest._value.node.value");
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    event.rename(
                        "_ingest._value.node.observable_value",
                        "_ingest._value.node.value",
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    if event.has_value("_ingest._value.node.x_opencti_description") {
                        event.rename(
                            "_ingest._value.node.x_opencti_description",
                            "_ingest._value.node.description",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    if event.has_value("_ingest._value.node.x_opencti_additional_names") {
                        event.rename(
                            "_ingest._value.node.x_opencti_additional_names",
                            "_ingest._value.node.additional_names",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    if event.has_value("_ingest._value.node.obsContent") {
                        event.rename(
                            "_ingest._value.node.obsContent",
                            "_ingest._value.node.content",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    if event.has_value("_ingest._value.node.serviceDlls.edges") {
                        foreach_array(event, "_ingest._value.node.serviceDlls.edges", |event| {
                            event.rename(
                                "_ingest._value.node.x_opencti_additional_names",
                                "_ingest.node._value.additional_names",
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    if event.has_value("_ingest._value.node.serviceDlls.edges") {
                        foreach_array(event, "_ingest._value.node.serviceDlls.edges", |event| {
                            event.rename(
                                "_ingest._value.node.obsContent",
                                "_ingest._value.node.content",
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    if event.has_value("_ingest._value.node.serviceDlls.edges") {
                        foreach_array(event, "_ingest._value.node.serviceDlls.edges", |event| {
                            event.rename("_ingest._value.node", "_ingest._value")?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("observables.edges") {
                foreach_array(event, "observables.edges", |event| {
                    if event.has_value("_ingest._value.node.serviceDlls.edges") {
                        event.rename(
                            "_ingest._value.node.serviceDlls.edges",
                            "_ingest._value.node.service_dlls",
                        )?;
                    }
                    Ok(())
                })?;
            }

            // Painless script
            // Source: if (ctx.observables?.edges instanceof List) {\n  for (def edge : ctx.observables.edges) {\n    if (edge.node?.startup_info instanceof List) {\n      def result = [:];\n      for (def kv : edge.node.startup_info) {\n        result[kv.key] = kv.value;\n      }\n      edge.node.startup_info = result;\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.observables?.edges instanceof List) {\n  for (def edge : ctx.observables.edges) {\n    if (edge.node?.startup_info instanceof List) {\n      def result = [:];\n      for (def kv : edge.node.startup_info) {\n        result[kv.key] = kv.value;\n      }\n      edge.node.startup_info = result;\n    }\n  }\n}\n"#
                ),
            )?;

            // Painless script
            // Source: Map hashesToECS(ArrayList hashes) {\n  Map hash = [:];\n  for (int i = 0; i < hashes.length; i++) {\n    String algorithm = hashes[i].algorithm;\n    algorithm = algorithm.toLowerCase();\n    algorithm = algorithm.replace('sha-', 'sha');\n    algorithm = algorithm.replace('sha3-', 'sha3_');\n    hash[algorithm] = hashes[i].hash;\n  }\n  return hash;\n}\nArrayList observables = ctx.observables.edges;\nfor (int i = 0; i < observables.length; i++) {\n  Map observable = observables[i].node;\n  if (observable.containsKey('hashes')) {\n    def ecsHash = hashesToECS(observable.hashes);\n    if (ecsHash.size() > 0) {\n      observable.hash = ecsHash;\n    }\n    observable.remove('hashes');\n  }\n  if (observable.containsKey('service_dlls')) {\n    for (int ii = 0; ii < observable.service_dlls.length; ii++) {\n      Map serviceDll = observable.service_dlls[ii];\n      if (serviceDll.containsKey('hashes')) {\n        def ecsHash = hashesToECS(serviceDll.hashes);\n        if (ecsHash.size() > 0) {\n          serviceDll.hash = ecsHash;\n        }\n        serviceDll.remove('hashes');\n      }\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map hashesToECS(ArrayList hashes) {\n  Map hash = [:];\n  for (int i = 0; i < hashes.length; i++) {\n    String algorithm = hashes[i].algorithm;\n    algorithm = algorithm.toLowerCase();\n    algorithm = algorithm.replace('sha-', 'sha');\n    algorithm = algorithm.replace('sha3-', 'sha3_');\n    hash[algorithm] = hashes[i].hash;\n  }\n  return hash;\n}\nArrayList observables = ctx.observables.edges;\nfor (int i = 0; i < observables.length; i++) {\n  Map observable = observables[i].node;\n  if (observable.containsKey('hashes')) {\n    def ecsHash = hashesToECS(observable.hashes);\n    if (ecsHash.size() > 0) {\n      observable.hash = ecsHash;\n    }\n    observable.remove('hashes');\n  }\n  if (observable.containsKey('service_dlls')) {\n    for (int ii = 0; ii < observable.service_dlls.length; ii++) {\n      Map serviceDll = observable.service_dlls[ii];\n      if (serviceDll.containsKey('hashes')) {\n        def ecsHash = hashesToECS(serviceDll.hashes);\n        if (ecsHash.size() > 0) {\n          serviceDll.hash = ecsHash;\n        }\n        serviceDll.remove('hashes');\n      }\n    }\n  }\n}\n"#
                ),
            )?;

            // Painless script
            // Source: if (!ctx.containsKey('opencti')) {\n  ctx.opencti = [:];\n}\nif (!ctx.opencti.containsKey('observable')) {\n  ctx.opencti.observable = [:];\n}\nArrayList observables = ctx.observables.edges;\nfor (int i = 0; i < observables.length; i++) {\n  String entity = observables[i]['node']['entity_type'];\n  entity = entity.replace('StixFile', 'file');\n  entity = entity.toLowerCase();\n  entity = entity.replace('-', '_');\n  if (!ctx.opencti.observable.containsKey(entity)) {\n    ctx.opencti.observable[entity] = [];\n  }\n  ctx.opencti.observable[entity].add(observables[i]['node']);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (!ctx.containsKey('opencti')) {\n  ctx.opencti = [:];\n}\nif (!ctx.opencti.containsKey('observable')) {\n  ctx.opencti.observable = [:];\n}\nArrayList observables = ctx.observables.edges;\nfor (int i = 0; i < observables.length; i++) {\n  String entity = observables[i]['node']['entity_type'];\n  entity = entity.replace('StixFile', 'file');\n  entity = entity.toLowerCase();\n  entity = entity.replace('-', '_');\n  if (!ctx.opencti.observable.containsKey(entity)) {\n    ctx.opencti.observable[entity] = [];\n  }\n  ctx.opencti.observable[entity].add(observables[i]['node']);\n}\n"#
                ),
            )?;

            event.remove("observables");

            // Painless script
            // Source: def dropNulls(Object obj) {\n  if (obj instanceof Map) {\n    def map = (Map) obj;\n    Map result = [:];\n    for (def key : map.keySet()) {\n      if (map[key] instanceof Map || map[key] instanceof List) {\n        result[key] = dropNulls(map[key]);\n      } else if (map[key] != null) {\n        result[key] = map[key];\n      }\n    }\n    return result;\n  } else if (obj instanceof List) {\n    def list = (List) obj;\n    List result = [];\n    for (def item : list) {\n      if (item instanceof Map || item instanceof List) {\n        result.add(dropNulls(item));\n      } else if (item != null) {\n        result.add(item);\n      }\n    }\n    return result;\n  } else {\n    return obj;\n  }\n}\nif (ctx.opencti?.containsKey('observable') == true) {\n  ctx.opencti.observable = dropNulls(ctx.opencti.observable);\n  for (def key : ctx.opencti.observable.keySet()) {\n    if (ctx.opencti.observable[key].size() == 0) {\n      ctx.opencti.observable.remove(key);\n    }\n  }\n  if (ctx.opencti.observable.size() == 0) {\n    ctx.opencti.remove('observable');\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def dropNulls(Object obj) {\n  if (obj instanceof Map) {\n    def map = (Map) obj;\n    Map result = [:];\n    for (def key : map.keySet()) {\n      if (map[key] instanceof Map || map[key] instanceof List) {\n        result[key] = dropNulls(map[key]);\n      } else if (map[key] != null) {\n        result[key] = map[key];\n      }\n    }\n    return result;\n  } else if (obj instanceof List) {\n    def list = (List) obj;\n    List result = [];\n    for (def item : list) {\n      if (item instanceof Map || item instanceof List) {\n        result.add(dropNulls(item));\n      } else if (item != null) {\n        result.add(item);\n      }\n    }\n    return result;\n  } else {\n    return obj;\n  }\n}\nif (ctx.opencti?.containsKey('observable') == true) {\n  ctx.opencti.observable = dropNulls(ctx.opencti.observable);\n  for (def key : ctx.opencti.observable.keySet()) {\n    if (ctx.opencti.observable[key].size() == 0) {\n      ctx.opencti.observable.remove(key);\n    }\n  }\n  if (ctx.opencti.observable.size() == 0) {\n    ctx.opencti.remove('observable');\n  }\n}\n"#
                ),
            )?;

            if event.has_value("opencti.observable.email_addr") {
                foreach_array(event, "opencti.observable.email_addr", |event| {
                    event.append_unique(
                        "threat.indicator.email.address",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.ipv4_addr") {
                foreach_array(event, "opencti.observable.ipv4_addr", |event| {
                    event.append_unique(
                        "threat.indicator.ip",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.ipv6_addr") {
                foreach_array(event, "opencti.observable.ipv6_addr", |event| {
                    event.append_unique(
                        "threat.indicator.ip",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("threat.indicator.ip") {
                foreach_array(event, "threat.indicator.ip", |event| {
                    gsub_field(
                        event,
                        "_ingest._value",
                        "_ingest._value",
                        cached_regex!("/\\d+$"),
                        "",
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.network_traffic") {
                foreach_array(event, "opencti.observable.network_traffic", |event| {
                    event.append_unique(
                        "threat.indicator.port",
                        json!(
                            event
                                .get("_ingest._value.src_port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.network_traffic") {
                foreach_array(event, "opencti.observable.network_traffic", |event| {
                    event.append_unique(
                        "threat.indicator.port",
                        json!(
                            event
                                .get("_ingest._value.dst_port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.autonomous_system") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("opencti.observable.autonomous_system").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_autonomous_system"
                            event.set(
                                "_tmp_as.number",
                                json!(
                                    event
                                        .get("_ingest._value.number")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_as.organization.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.as = ctx.threat.indicator.as ?: [];\nctx.threat.indicator.as.add(ctx._tmp_as);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_as", "threat.indicator.as"),
                            );
                            if event.remove("_tmp_as").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_as".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_autonomous_system"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.autonomous_system",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("opencti.observable.windows_registry_key") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("opencti.observable.windows_registry_key")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_windows_registry_key"
                            if let Some(input) = event.get_string("_ingest._value.attribute_key") {
                                // Grok pattern: ^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:_tmp_registry.key}$
                                if !cached_grok_mapped!("^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:_tmp_registry.key}$", [("_tmp_registry_hive", "_tmp_registry.hive")]).extract_into(&input, event)? {
                            return Err(TransformError::GrokNoMatch { value: input });
                            }
                            }
                            let _cond = { event.has_value("_tmp_registry.hive") };
                            if _cond {
                                // Painless script
                                // Source: def name = ctx._tmp_registry.hive.toUpperCase();\nctx._tmp_registry.hive = params.getOrDefault(name, name);\n
                                // TODO: Transpile Painless to Rust (2.2.3)
                                painless_exec_plan_params(
                                    event,
                                    cached_painless!(
                                        r#"def name = ctx._tmp_registry.hive.toUpperCase();\nctx._tmp_registry.hive = params.getOrDefault(name, name);\n"#
                                    ),
                                    cached_params!(
                                        "{\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_USERS\":\"HKU\",\"HKEY_CURRENT_CONFIG\":\"HKCC\"}"
                                    ),
                                )?;
                            }
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.registry = ctx.threat.indicator.registry ?: [];\nctx.threat.indicator.registry.add(ctx._tmp_registry);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_registry", "threat.indicator.registry"),
                            );
                            if event.remove("_tmp_registry").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_registry".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_windows_registry_key"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.windows_registry_key",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("opencti.observable.windows_registry_value_type") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("opencti.observable.windows_registry_value_type")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_windows_registry_value_type"
                            event.set(
                                "_tmp_registry.value",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_registry.data.type",
                                json!(
                                    event
                                        .get("_ingest._value.data_type")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.append(
                                "_tmp_registry.data.strings",
                                json!(
                                    event
                                        .get("_ingest._value.data")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.registry = ctx.threat.indicator.registry ?: [];\nctx.threat.indicator.registry.add(ctx._tmp_registry);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_registry", "threat.indicator.registry"),
                            );
                            if event.remove("_tmp_registry").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_registry".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_windows_registry_value_type"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.windows_registry_value_type",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("opencti.observable.x509_certificate") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("opencti.observable.x509_certificate").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_x509_certificate"
                            event.set(
                                "_tmp_x509.alternative_names",
                                json!(
                                    event
                                        .get("_ingest._value.subject_alternative_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.issuer.common_name",
                                json!(
                                    event
                                        .get("_ingest._value.issuer")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.not_after",
                                json!(
                                    event
                                        .get("_ingest._value.validity_not_after")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.not_before",
                                json!(
                                    event
                                        .get("_ingest._value.validity_not_before")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.public_key_algorithm",
                                json!(
                                    event
                                        .get("_ingest._value.subject_public_key_algorithm")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.public_key_exponent",
                                json!(
                                    event
                                        .get("_ingest._value.subject_public_key_exponent")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.serial_number",
                                json!(
                                    event
                                        .get("_ingest._value.serial_number")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.signature_algorithm",
                                json!(
                                    event
                                        .get("_ingest._value.signature_algorithm")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.subject.common_name",
                                json!(
                                    event
                                        .get("_ingest._value.subject")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.set(
                                "_tmp_x509.version_number",
                                json!(
                                    event
                                        .get("_ingest._value.version")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.x509 = ctx.threat.indicator.x509 ?: [];\nctx.threat.indicator.x509.add(ctx._tmp_x509);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_x509", "threat.indicator.x509"),
                            );
                            if event.remove("_tmp_x509").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_x509".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_x509_certificate"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.x509_certificate",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("opencti.observable.url") {
                foreach_array(event, "opencti.observable.url", |event| {
                    event.append_unique(
                        "_tmp_found_urls",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.artifact") {
                foreach_array(event, "opencti.observable.artifact", |event| {
                    event.append_unique(
                        "_tmp_found_urls",
                        json!(
                            event
                                .get("_ingest._value.url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.file") {
                foreach_array(event, "opencti.observable.file", |event| {
                    event.append_unique(
                        "_tmp_found_urls",
                        json!(
                            event
                                .get("_ingest._value.content?.url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.media_content") {
                foreach_array(event, "opencti.observable.media_content", |event| {
                    event.append_unique(
                        "_tmp_found_urls",
                        json!(
                            event
                                .get("_ingest._value.url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.process") {
                foreach_array(event, "opencti.observable.process", |event| {
                    if event.has_value("_ingest._value.service_dll") {
                        foreach_array(event, "_ingest._value.service_dll", |event| {
                            event.append_unique(
                                "_tmp_found_urls",
                                json!(
                                    event
                                        .get("_ingest._value.content?.url")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            // Painless script
            // Source: if (ctx._tmp_found_urls != null) {\n  def result = [];\n  for (item in ctx._tmp_found_urls) {\n      if (item != null && item != '') {\n          result.add(item);\n      }\n  }\n  ctx._tmp_found_urls = result;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx._tmp_found_urls != null) {\n  def result = [];\n  for (item in ctx._tmp_found_urls) {\n      if (item != null && item != '') {\n          result.add(item);\n      }\n  }\n  ctx._tmp_found_urls = result;\n}\n"#
                ),
            )?;

            if event.has_value("_tmp_found_urls") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("_tmp_found_urls").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_url_field"
                            event.rename("_ingest._value", "_tmp_url.original")?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                uri_parts(event, "_tmp_url.original", "_tmp_url", true, false)?;
                                Ok(())
                            })();
                            event.remove("_tmp_url.user_info");
                            if event.has_value("_tmp_url.domain") {
                                if let Some(domain_str) = event.get_string("_tmp_url.domain") {
                                    let domain = domain_str.to_string();
                                    event.set("_tmp_url.domain", json!(domain.clone()))?;
                                    // Public suffix list lookup for registered domain extraction
                                    if let Some(rd) = registered_domain_lookup(&domain) {
                                        if let Some(registered) = rd.registered_domain {
                                            event.set(
                                                "_tmp_url.registered_domain",
                                                json!(registered),
                                            )?;
                                        }
                                        event.set(
                                            "_tmp_url.top_level_domain",
                                            json!(rd.top_level_domain),
                                        )?;
                                        if let Some(sub) = rd.subdomain {
                                            event.set("_tmp_url.subdomain", json!(sub))?;
                                        }
                                    }
                                }
                            }
                            if let Some(v) = event.get("_tmp_url.original").cloned() {
                                event.set("_tmp_url.full", v)?;
                            }
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.url = ctx.threat.indicator.url ?: [];\nctx.threat.indicator.url.add(ctx._tmp_url);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_url", "threat.indicator.url"),
                            );
                            if event.remove("_tmp_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_url".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_url_field"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "_tmp_found_urls",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            event.remove("_tmp_found_urls");

            if event.has_value("opencti.observable.domain_name") {
                foreach_array(event, "opencti.observable.domain_name", |event| {
                    event.append_unique(
                        "_tmp_found_domain_names_and_hostnames",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.hostname") {
                foreach_array(event, "opencti.observable.hostname", |event| {
                    event.append_unique(
                        "_tmp_found_domain_names_and_hostnames",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("_tmp_found_domain_names_and_hostnames") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("_tmp_found_domain_names_and_hostnames").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_domain_name_or_hostname"
                            event.rename("_ingest._value", "_tmp_url.domain")?;
                            if event.has_value("_tmp_url.domain") {
                                if let Some(domain_str) = event.get_string("_tmp_url.domain") {
                                    let domain = domain_str.to_string();
                                    event.set("_tmp_url.domain", json!(domain.clone()))?;
                                    // Public suffix list lookup for registered domain extraction
                                    if let Some(rd) = registered_domain_lookup(&domain) {
                                        if let Some(registered) = rd.registered_domain {
                                            event.set(
                                                "_tmp_url.registered_domain",
                                                json!(registered),
                                            )?;
                                        }
                                        event.set(
                                            "_tmp_url.top_level_domain",
                                            json!(rd.top_level_domain),
                                        )?;
                                        if let Some(sub) = rd.subdomain {
                                            event.set("_tmp_url.subdomain", json!(sub))?;
                                        }
                                    }
                                }
                            }
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.url = ctx.threat.indicator.url ?: [];\nctx.threat.indicator.url.add(ctx._tmp_url);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_url", "threat.indicator.url"),
                            );
                            if event.remove("_tmp_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_url".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_domain_name_or_hostname"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "_tmp_found_domain_names_and_hostnames",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            event.remove("_tmp_found_domain_names_and_hostnames");

            if event.has_value("opencti.observable.file") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("opencti.observable.file").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_file"
                            let v = json!(
                                event
                                    .get("_ingest._value.atime")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.accessed", v)?;
                            }
                            let v = json!(
                                event
                                    .get("_ingest._value.ctime")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.created", v)?;
                            }
                            let v = json!(
                                event
                                    .get("_ingest._value.mtime")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.mtime", v)?;
                            }
                            event.append_unique(
                                "_tmp_file.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            if event.has_value("_ingest._value.additional_names") {
                                foreach_array(event, "_ingest._value.additional_names", |event| {
                                    event.append_unique(
                                        "_tmp_file.name",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    Ok(())
                                })?;
                            }
                            // Painless script
                            // Source: if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n
                            // TODO: Transpile Painless to Rust (2.2.3)
                            painless_exec_plan(
                                event,
                                cached_painless!(
                                    r#"if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n"#
                                ),
                            )?;
                            // Painless script
                            // Source: if (ctx._tmp_file.name != null) {\n  def result = [];\n  for (fullname in ctx._tmp_file.name) {\n    // name shouldn't have a directory, but do strip it if it's there\n    def parts = /[\\/\\\\]/.split(fullname);\n    def name = parts[parts.length - 1];\n    if (name.contains(\".\")) {\n      def nameParts = /\\./.split(name);\n      def extension = nameParts[nameParts.length - 1];\n      if (extension.length() > 0) {\n        result.add(extension);\n      }\n    }\n  }\n  if (result.length > 0) {\n    ctx._tmp_file.extension = result;\n  }\n}\n
                            // TODO: Transpile Painless to Rust (2.2.3)
                            painless_exec_plan(
                                event,
                                cached_painless!(
                                    r#"if (ctx._tmp_file.name != null) {\n  def result = [];\n  for (fullname in ctx._tmp_file.name) {\n    // name shouldn't have a directory, but do strip it if it's there\n    def parts = /[\\/\\\\]/.split(fullname);\n    def name = parts[parts.length - 1];\n    if (name.contains(\".\")) {\n      def nameParts = /\\./.split(name);\n      def extension = nameParts[nameParts.length - 1];\n      if (extension.length() > 0) {\n        result.add(extension);\n      }\n    }\n  }\n  if (result.length > 0) {\n    ctx._tmp_file.extension = result;\n  }\n}\n"#
                                ),
                            )?;
                            let v = json!(
                                event
                                    .get("_ingest._value.mime_type")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.mime_type", v)?;
                            }
                            let v = json!(
                                event
                                    .get("_ingest._value.size")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.size", v)?;
                            }
                            let v = json!("file");
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.type", v)?;
                            }
                            if let Some(v) = event
                                .get("_ingest._value.hash")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("_tmp_file.hash", v)?;
                            }
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.file = ctx.threat.indicator.file ?: [];\nctx.threat.indicator.file.add(ctx._tmp_file);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_file", "threat.indicator.file"),
                            );
                            if event.remove("_tmp_file").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_file".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_file"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.file",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("opencti.observable.process") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("opencti.observable.process").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            if event.has_value("_ingest._value.service_dll") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.service_dll").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
                                        Some(Value::Object(fields)) => {
                                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                        }
                                        _ => Vec::new(),
                                    };
                                    if !entries.is_empty() {
                                        // A NESTED loop borrows the same slots, so the enclosing
                                        // entry is saved and put back afterwards.
                                        let enclosing = event.get("_ingest._value").cloned();
                                        let enclosing_key = event.get("_ingest._key").cloned();
                                        let mut list = Vec::with_capacity(entries.len());
                                        let mut fields = Map::new();
                                        for (key, item) in entries {
                                            if let Some(key) = key.as_deref() {
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // Begin nested pipeline: "ecs_from_file"
                                            let v = json!(
                                                event
                                                    .get("_ingest._value.atime")
                                                    .map_or_else(String::new, template_to_string)
                                            );
                                            if !painless_is_empty_value(&v) {
                                                event.set("_tmp_file.accessed", v)?;
                                            }
                                            let v = json!(
                                                event
                                                    .get("_ingest._value.ctime")
                                                    .map_or_else(String::new, template_to_string)
                                            );
                                            if !painless_is_empty_value(&v) {
                                                event.set("_tmp_file.created", v)?;
                                            }
                                            let v = json!(
                                                event
                                                    .get("_ingest._value.mtime")
                                                    .map_or_else(String::new, template_to_string)
                                            );
                                            if !painless_is_empty_value(&v) {
                                                event.set("_tmp_file.mtime", v)?;
                                            }
                                            event.append_unique(
                                                "_tmp_file.name",
                                                json!(
                                                    event.get("_ingest._value.name").map_or_else(
                                                        String::new,
                                                        template_to_string
                                                    )
                                                ),
                                            )?;
                                            if event.has_value("_ingest._value.additional_names") {
                                                foreach_array(
                                                    event,
                                                    "_ingest._value.additional_names",
                                                    |event| {
                                                        event.append_unique(
                                                            "_tmp_file.name",
                                                            json!(
                                                                event
                                                                    .get("_ingest._value")
                                                                    .map_or_else(
                                                                        String::new,
                                                                        template_to_string
                                                                    )
                                                            ),
                                                        )?;
                                                        Ok(())
                                                    },
                                                )?;
                                            }
                                            // Painless script
                                            // Source: if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n
                                            // TODO: Transpile Painless to Rust (2.2.3)
                                            painless_exec_plan(
                                                event,
                                                cached_painless!(
                                                    r#"if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n"#
                                                ),
                                            )?;
                                            // Painless script
                                            // Source: if (ctx._tmp_file.name != null) {\n  def result = [];\n  for (fullname in ctx._tmp_file.name) {\n    // name shouldn't have a directory, but do strip it if it's there\n    def parts = /[\\/\\\\]/.split(fullname);\n    def name = parts[parts.length - 1];\n    if (name.contains(\".\")) {\n      def nameParts = /\\./.split(name);\n      def extension = nameParts[nameParts.length - 1];\n      if (extension.length() > 0) {\n        result.add(extension);\n      }\n    }\n  }\n  if (result.length > 0) {\n    ctx._tmp_file.extension = result;\n  }\n}\n
                                            // TODO: Transpile Painless to Rust (2.2.3)
                                            painless_exec_plan(
                                                event,
                                                cached_painless!(
                                                    r#"if (ctx._tmp_file.name != null) {\n  def result = [];\n  for (fullname in ctx._tmp_file.name) {\n    // name shouldn't have a directory, but do strip it if it's there\n    def parts = /[\\/\\\\]/.split(fullname);\n    def name = parts[parts.length - 1];\n    if (name.contains(\".\")) {\n      def nameParts = /\\./.split(name);\n      def extension = nameParts[nameParts.length - 1];\n      if (extension.length() > 0) {\n        result.add(extension);\n      }\n    }\n  }\n  if (result.length > 0) {\n    ctx._tmp_file.extension = result;\n  }\n}\n"#
                                                ),
                                            )?;
                                            let v = json!(
                                                event
                                                    .get("_ingest._value.mime_type")
                                                    .map_or_else(String::new, template_to_string)
                                            );
                                            if !painless_is_empty_value(&v) {
                                                event.set("_tmp_file.mime_type", v)?;
                                            }
                                            let v = json!(
                                                event
                                                    .get("_ingest._value.size")
                                                    .map_or_else(String::new, template_to_string)
                                            );
                                            if !painless_is_empty_value(&v) {
                                                event.set("_tmp_file.size", v)?;
                                            }
                                            let v = json!("file");
                                            if !painless_is_empty_value(&v) {
                                                event.set("_tmp_file.type", v)?;
                                            }
                                            if let Some(v) = event
                                                .get("_ingest._value.hash")
                                                .filter(|v| !painless_is_empty_value(v))
                                                .cloned()
                                            {
                                                event.set("_tmp_file.hash", v)?;
                                            }
                                            // Painless script, resolved to its runners at generation time
                                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.file = ctx.threat.indicator.file ?: [];\nctx.threat.indicator.file.add(ctx._tmp_file);\n
                                            ensure_append(
                                                event,
                                                &EnsureAppend::new(
                                                    "_tmp_file",
                                                    "threat.indicator.file",
                                                ),
                                            );
                                            if event.remove("_tmp_file").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_tmp_file".into(),
                                                });
                                            }
                                            // End nested pipeline: "ecs_from_file"
                                            let left = event.remove("_ingest._value");
                                            match key {
                                                // An entry the body renamed AWAY is gone from the
                                                // object, which is how a foreach lifts fields up.
                                                Some(key) => {
                                                    if let Some(value) = left {
                                                        fields.insert(key, value);
                                                    }
                                                }
                                                None => list.push(left.unwrap_or(Value::Null)),
                                            }
                                        }
                                        match enclosing {
                                            Some(previous) => {
                                                event.set("_ingest._value", previous)?;
                                            }
                                            None => {
                                                event.remove("_ingest");
                                            }
                                        }
                                        if let Some(previous) = enclosing_key {
                                            event.set("_ingest._key", previous)?;
                                        }
                                        event.set(
                                            "_ingest._value.service_dll",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.process",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("opencti.observable.directory") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("opencti.observable.directory").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_directory"
                            let v = json!(
                                event
                                    .get("_ingest._value.atime")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.accessed", v)?;
                            }
                            let v = json!(
                                event
                                    .get("_ingest._value.ctime")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.created", v)?;
                            }
                            let v = json!(
                                event
                                    .get("_ingest._value.mtime")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.mtime", v)?;
                            }
                            event.append_unique(
                                "_tmp_file.path",
                                json!(
                                    event
                                        .get("_ingest._value.path")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            let v = json!("dir");
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.type", v)?;
                            }
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.file = ctx.threat.indicator.file ?: [];\nctx.threat.indicator.file.add(ctx._tmp_file);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_file", "threat.indicator.file"),
                            );
                            if event.remove("_tmp_file").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_file".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_directory"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.directory",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("opencti.observable.artifact") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("opencti.observable.artifact").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_artifact"
                            if event.has_value("_ingest._value.additional_names") {
                                foreach_array(event, "_ingest._value.additional_names", |event| {
                                    event.append_unique(
                                        "_tmp_file.name",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    Ok(())
                                })?;
                            }
                            // Painless script
                            // Source: if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else if (result.length == 1) {\n    ctx._tmp_file.name = result[0];\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n
                            // TODO: Transpile Painless to Rust (2.2.3)
                            painless_exec_plan(
                                event,
                                cached_painless!(
                                    r#"if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else if (result.length == 1) {\n    ctx._tmp_file.name = result[0];\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n"#
                                ),
                            )?;
                            let v = json!(
                                event
                                    .get("_ingest._value.mime_type")
                                    .map_or_else(String::new, template_to_string)
                            );
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.mime_type", v)?;
                            }
                            let v = json!("file");
                            if !painless_is_empty_value(&v) {
                                event.set("_tmp_file.type", v)?;
                            }
                            if let Some(v) = event
                                .get("_ingest._value.hash")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("_tmp_file.hash", v)?;
                            }
                            // Painless script, resolved to its runners at generation time
                            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.file = ctx.threat.indicator.file ?: [];\nctx.threat.indicator.file.add(ctx._tmp_file);\n
                            ensure_append(
                                event,
                                &EnsureAppend::new("_tmp_file", "threat.indicator.file"),
                            );
                            if event.remove("_tmp_file").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_tmp_file".into(),
                                });
                            }
                            // End nested pipeline: "ecs_from_artifact"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.observable.artifact",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event.get_i64("opencti.indicator.observables_count") == Some(0)
                    && event.has_value("opencti.indicator.pattern")
            };
            if _cond {
                if event.has_value("opencti.indicator.pattern") {
                    if let Some(s) = event.get_string("opencti.indicator.pattern") {
                        let mut parts: Vec<Value> = cached_regex!("\\s+AND\\s+|\\s+OR\\s+")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("opencti.indicator._patterns", Value::Array(parts))?;
                    }
                }
            }

            if event.has_value("opencti.indicator._patterns") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("opencti.indicator._patterns").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // Begin nested pipeline: "ecs_from_pattern"
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
                                            cached_grok!(
                                                "file:hashes.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.md5}'"
                                            ),
                                            cached_grok!(
                                                "file:hashes.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha1}'"
                                            ),
                                            cached_grok!(
                                                "file:hashes.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.hash.sha256}'"
                                            ),
                                            cached_grok!(
                                                "file:name%{SPACE}=%{SPACE}'%{DATA:threat.indicator.file.name}'"
                                            ),
                                            cached_grok!(
                                                "domain-name:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.domain}'"
                                            ),
                                            cached_grok!(
                                                "hostname:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.domain}'"
                                            ),
                                            cached_grok!(
                                                "url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'"
                                            ),
                                            cached_grok!(
                                                "email-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.email.address}'"
                                            ),
                                            cached_grok!(
                                                "ipv4-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.ip}'"
                                            ),
                                            cached_grok!(
                                                "ipv6-addr:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.ip}'"
                                            ),
                                            cached_grok!(
                                                "windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp_registry}'"
                                            ),
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
                                painless_exec_plan(
                                    event,
                                    cached_painless!(
                                        r#"def tmp_file_name = ctx.threat.indicator.file.name;\nif (tmp_file_name != null) {\n  def parts = /[\\/\\\\]/.split(tmp_file_name);\n  def name = parts[parts.length - 1];\n  if (name.contains(\".\")) {\n    def nameParts = /\\./.split(name);\n    def extension = nameParts[nameParts.length - 1];\n    if (extension.length() > 0) {\n      ctx.threat.indicator.file.extension = extension;\n    }\n  }\n}\n"#
                                    ),
                                )?;
                            }
                            if event.has_value("threat.indicator.ip") {
                                gsub_field(
                                    event,
                                    "threat.indicator.ip",
                                    "threat.indicator.ip",
                                    cached_regex!("/\\d+$"),
                                    "",
                                )?;
                            }
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                uri_parts(
                                    event,
                                    "threat.indicator.url.original",
                                    "threat.indicator.url",
                                    true,
                                    false,
                                )?;
                                Ok(())
                            })();
                            if event.has_value("threat.indicator.url.domain") {
                                if let Some(domain_str) =
                                    event.get_string("threat.indicator.url.domain")
                                {
                                    let domain = domain_str.to_string();
                                    event.set(
                                        "threat.indicator.url.domain",
                                        json!(domain.clone()),
                                    )?;
                                    // Public suffix list lookup for registered domain extraction
                                    if let Some(rd) = registered_domain_lookup(&domain) {
                                        if let Some(registered) = rd.registered_domain {
                                            event.set(
                                                "threat.indicator.url.registered_domain",
                                                json!(registered),
                                            )?;
                                        }
                                        event.set(
                                            "threat.indicator.url.top_level_domain",
                                            json!(rd.top_level_domain),
                                        )?;
                                        if let Some(sub) = rd.subdomain {
                                            event.set(
                                                "threat.indicator.url.subdomain",
                                                json!(sub),
                                            )?;
                                        }
                                    }
                                }
                            }
                            if let Some(v) = event
                                .get("threat.indicator.url.original")
                                .filter(|v| !painless_is_empty_value(v))
                                .cloned()
                            {
                                event.set("threat.indicator.url.full", v)?;
                            }
                            if event.has_value("_tmp_registry") {
                                gsub_field(
                                    event,
                                    "_tmp_registry",
                                    "_tmp_registry",
                                    cached_regex!("\\\\\\\\"),
                                    "\\\\",
                                )?;
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
                                painless_exec_plan_params(
                                    event,
                                    cached_painless!(
                                        r#"def name = ctx.tmp_registry.hive.toUpperCase();\nctx.tmp_registry.hive = params.getOrDefault(name, name);\n"#
                                    ),
                                    cached_params!(
                                        "{\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_USERS\":\"HKU\",\"HKEY_CURRENT_CONFIG\":\"HKCC\"}"
                                    ),
                                )?;
                            }
                            if event.has_value("tmp_registry") {
                                event.rename("tmp_registry", "threat.indicator.registry")?;
                            }
                            event.remove("_tmp_registry");
                            // Painless script
                            // Source: if (ctx.threat?.indicator?.file?.name != null && !(ctx.threat.indicator.file.name instanceof List)) {\n  ctx.threat.indicator.file.name = [ctx.threat.indicator.file.name];\n}\nif (ctx.threat?.indicator?.file?.extension != null && !(ctx.threat.indicator.file.extension instanceof List)) {\n  ctx.threat.indicator.file.extension = [ctx.threat.indicator.file.extension];\n}\nif (ctx.threat?.indicator?.email?.address != null && !(ctx.threat.indicator.email.address instanceof List)) {\n  ctx.threat.indicator.email.address = [ctx.threat.indicator.email.address];\n}\nif (ctx.threat?.indicator?.ip != null && !(ctx.threat.indicator.ip instanceof List)) {\n  ctx.threat.indicator.ip = [ctx.threat.indicator.ip];\n}\nif (ctx.threat?.indicator?.url != null && !(ctx.threat.indicator.url instanceof List)) {\n  ctx.threat.indicator.url = [ctx.threat.indicator.url];\n}\n
                            // TODO: Transpile Painless to Rust (2.2.3)
                            painless_exec_plan(
                                event,
                                cached_painless!(
                                    r#"if (ctx.threat?.indicator?.file?.name != null && !(ctx.threat.indicator.file.name instanceof List)) {\n  ctx.threat.indicator.file.name = [ctx.threat.indicator.file.name];\n}\nif (ctx.threat?.indicator?.file?.extension != null && !(ctx.threat.indicator.file.extension instanceof List)) {\n  ctx.threat.indicator.file.extension = [ctx.threat.indicator.file.extension];\n}\nif (ctx.threat?.indicator?.email?.address != null && !(ctx.threat.indicator.email.address instanceof List)) {\n  ctx.threat.indicator.email.address = [ctx.threat.indicator.email.address];\n}\nif (ctx.threat?.indicator?.ip != null && !(ctx.threat.indicator.ip instanceof List)) {\n  ctx.threat.indicator.ip = [ctx.threat.indicator.ip];\n}\nif (ctx.threat?.indicator?.url != null && !(ctx.threat.indicator.url instanceof List)) {\n  ctx.threat.indicator.url = [ctx.threat.indicator.url];\n}\n"#
                                ),
                            )?;
                            // End nested pipeline: "ecs_from_pattern"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "opencti.indicator._patterns",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            event.remove("opencti.indicator._patterns");

            if event.has_value("opencti.observable.x509_certificate") {
                foreach_array(event, "opencti.observable.x509_certificate", |event| {
                    if event.has_value("_ingest._value.hash") {
                        foreach_array(event, "_ingest._value.hash", |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            // Painless script
            // Source: if (ctx.threat?.indicator?.file != null) {\n  def files = ctx.threat.indicator.file instanceof List ? ctx.threat.indicator.file : [ctx.threat.indicator.file];\n  ctx.related = ctx.related ?: [:];\n  ctx.related.hash = ctx.related.hash ?: [];\n  def hashSet = new HashSet(ctx.related.hash);\n  for (def file : files) {\n    if (file?.hash != null) {\n      if (file.hash.md5 != null) {\n        hashSet.add(file.hash.md5);\n      }\n      if (file.hash.sha1 != null) {\n        hashSet.add(file.hash.sha1);\n      }\n      if (file.hash.sha256 != null) {\n        hashSet.add(file.hash.sha256);\n      }\n    }\n  }\n  if (hashSet.size() > 0) {\n    ctx.related.hash = new ArrayList(hashSet);\n  } else {\n    ctx.related.remove('hash');\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.threat?.indicator?.file != null) {\n  def files = ctx.threat.indicator.file instanceof List ? ctx.threat.indicator.file : [ctx.threat.indicator.file];\n  ctx.related = ctx.related ?: [:];\n  ctx.related.hash = ctx.related.hash ?: [];\n  def hashSet = new HashSet(ctx.related.hash);\n  for (def file : files) {\n    if (file?.hash != null) {\n      if (file.hash.md5 != null) {\n        hashSet.add(file.hash.md5);\n      }\n      if (file.hash.sha1 != null) {\n        hashSet.add(file.hash.sha1);\n      }\n      if (file.hash.sha256 != null) {\n        hashSet.add(file.hash.sha256);\n      }\n    }\n  }\n  if (hashSet.size() > 0) {\n    ctx.related.hash = new ArrayList(hashSet);\n  } else {\n    ctx.related.remove('hash');\n  }\n}\n"#
                ),
            )?;

            if event.has_value("opencti.observable.artifact") {
                foreach_array(event, "opencti.observable.artifact", |event| {
                    if event.has_value("_ingest._value.hash") {
                        foreach_array(event, "_ingest._value.hash", |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.process") {
                foreach_array(event, "opencti.observable.process", |event| {
                    if event.has_value("_ingest._value.service_dll") {
                        foreach_array(event, "_ingest._value.service_dll", |event| {
                            if event.has_value("_ingest._value.hash") {
                                foreach_array(event, "_ingest._value.hash", |event| {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.hostname") {
                foreach_array(event, "opencti.observable.hostname", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("threat.indicator.url") {
                foreach_array(event, "threat.indicator.url", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("threat.indicator.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(v) = event.get("threat.indicator.ip").cloned() {
                    event.set("related.ip", v)?;
                }
            }

            if event.has_value("threat.indicator.email.address") {
                foreach_array(event, "threat.indicator.email.address", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("threat.indicator.url") {
                foreach_array(event, "threat.indicator.url", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.user_account") {
                foreach_array(event, "opencti.observable.user_account", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("opencti.observable.user_account") {
                foreach_array(event, "opencti.observable.user_account", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.account_login")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // Painless script
            // Source: def cleanUp(Map obj, String key) {\n  if (obj.containsKey(key) && obj[key] instanceof List) {\n    def result = [];\n    for (item in obj[key]) {\n        if (item != null && item != '') {\n            result.add(item);\n        }\n    }\n    if (result.isEmpty()) {\n      obj.remove(key);\n    } else {\n      obj[key] = result;\n    }\n  }\n  return obj;\n}\nif (ctx.containsKey('related') && ctx.related != null) {\n  cleanUp(ctx.related, 'hash');\n  cleanUp(ctx.related, 'hosts');\n  cleanUp(ctx.related, 'ip');\n  cleanUp(ctx.related, 'user');\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def cleanUp(Map obj, String key) {\n  if (obj.containsKey(key) && obj[key] instanceof List) {\n    def result = [];\n    for (item in obj[key]) {\n        if (item != null && item != '') {\n            result.add(item);\n        }\n    }\n    if (result.isEmpty()) {\n      obj.remove(key);\n    } else {\n      obj[key] = result;\n    }\n  }\n  return obj;\n}\nif (ctx.containsKey('related') && ctx.related != null) {\n  cleanUp(ctx.related, 'hash');\n  cleanUp(ctx.related, 'hosts');\n  cleanUp(ctx.related, 'ip');\n  cleanUp(ctx.related, 'user');\n}\n"#
                ),
            )?;

            // Painless script
            // Source: def mergeMaps(Map map1, Map map2) {\n  for (def key : map2.keySet()) {\n    if (map1.containsKey(key) && map1[key] != map2[key]) {\n      if (map1[key] instanceof Map && map2[key] instanceof Map) {\n        map1[key] = mergeMaps(map1[key], map2[key]);\n      } else {\n        if (!(map1[key] instanceof List)) {\n          map1[key] = [map1[key]];\n        }\n        def combined = new HashSet(map1[key]);\n        if (map2[key] instanceof List) {\n          combined.addAll(map2[key]);\n        } else {\n          combined.add(map2[key]);\n        }\n        map1[key] = new ArrayList(combined);\n      }\n    } else {\n      map1[key] = map2[key];\n    }\n  }\n  return map1;\n}\ndef mergeListOfMaps(List list) {\n  def merged = new HashMap();\n  for (def map : list) {\n    merged = mergeMaps(merged, map);\n  }\n  return merged;\n}\nif (ctx.opencti?.containsKey('observable') == true) {\n  for (def key : ctx.opencti.observable.keySet()) {\n    if (ctx.opencti.observable[key] instanceof List) {\n      ctx.opencti.observable[key] = mergeListOfMaps(ctx.opencti.observable[key]);\n    }\n  }\n}\nif (ctx.opencti?.indicator?.containsKey('external_reference') == true && ctx.opencti.indicator.external_reference instanceof List) {\n  ctx.opencti.indicator.external_reference = mergeListOfMaps(ctx.opencti.indicator.external_reference);\n}\nif (ctx.threat.indicator.containsKey('file') && ctx.threat.indicator.file instanceof List) {\n  ctx.threat.indicator.file = mergeListOfMaps(ctx.threat.indicator.file);\n}\nif (ctx.threat.indicator.containsKey('as') && ctx.threat.indicator.as instanceof List) {\n  ctx.threat.indicator.as = mergeListOfMaps(ctx.threat.indicator.as);\n}\nif (ctx.threat.indicator.containsKey('url') && ctx.threat.indicator.url instanceof List) {\n  ctx.threat.indicator.url = mergeListOfMaps(ctx.threat.indicator.url);\n}\nif (ctx.threat.indicator.containsKey('registry') && ctx.threat.indicator.registry instanceof List) {\n  ctx.threat.indicator.registry = mergeListOfMaps(ctx.threat.indicator.registry);\n}\nif (ctx.threat.indicator.containsKey('x509') && ctx.threat.indicator.x509 instanceof List) {\n  ctx.threat.indicator.x509 = mergeListOfMaps(ctx.threat.indicator.x509);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def mergeMaps(Map map1, Map map2) {\n  for (def key : map2.keySet()) {\n    if (map1.containsKey(key) && map1[key] != map2[key]) {\n      if (map1[key] instanceof Map && map2[key] instanceof Map) {\n        map1[key] = mergeMaps(map1[key], map2[key]);\n      } else {\n        if (!(map1[key] instanceof List)) {\n          map1[key] = [map1[key]];\n        }\n        def combined = new HashSet(map1[key]);\n        if (map2[key] instanceof List) {\n          combined.addAll(map2[key]);\n        } else {\n          combined.add(map2[key]);\n        }\n        map1[key] = new ArrayList(combined);\n      }\n    } else {\n      map1[key] = map2[key];\n    }\n  }\n  return map1;\n}\ndef mergeListOfMaps(List list) {\n  def merged = new HashMap();\n  for (def map : list) {\n    merged = mergeMaps(merged, map);\n  }\n  return merged;\n}\nif (ctx.opencti?.containsKey('observable') == true) {\n  for (def key : ctx.opencti.observable.keySet()) {\n    if (ctx.opencti.observable[key] instanceof List) {\n      ctx.opencti.observable[key] = mergeListOfMaps(ctx.opencti.observable[key]);\n    }\n  }\n}\nif (ctx.opencti?.indicator?.containsKey('external_reference') == true && ctx.opencti.indicator.external_reference instanceof List) {\n  ctx.opencti.indicator.external_reference = mergeListOfMaps(ctx.opencti.indicator.external_reference);\n}\nif (ctx.threat.indicator.containsKey('file') && ctx.threat.indicator.file instanceof List) {\n  ctx.threat.indicator.file = mergeListOfMaps(ctx.threat.indicator.file);\n}\nif (ctx.threat.indicator.containsKey('as') && ctx.threat.indicator.as instanceof List) {\n  ctx.threat.indicator.as = mergeListOfMaps(ctx.threat.indicator.as);\n}\nif (ctx.threat.indicator.containsKey('url') && ctx.threat.indicator.url instanceof List) {\n  ctx.threat.indicator.url = mergeListOfMaps(ctx.threat.indicator.url);\n}\nif (ctx.threat.indicator.containsKey('registry') && ctx.threat.indicator.registry instanceof List) {\n  ctx.threat.indicator.registry = mergeListOfMaps(ctx.threat.indicator.registry);\n}\nif (ctx.threat.indicator.containsKey('x509') && ctx.threat.indicator.x509 instanceof List) {\n  ctx.threat.indicator.x509 = mergeListOfMaps(ctx.threat.indicator.x509);\n}\n"#
                ),
            )?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("opencti.indicator.standard_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("opencti.indicator.updated_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("threat.indicator.modified_at") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("opencti.indicator.pattern_type")
                    && (event.get_str("opencti.indicator.pattern_type") == Some("kql")
                        || event.get_str("opencti.indicator.pattern_type") == Some("lucene")
                        || event.get_str("opencti.indicator.pattern_type") == Some("eql")
                        || event.get_str("opencti.indicator.pattern_type") == Some("esql"))
                    && event.get_bool("opencti.indicator.revoked") != Some(true)
            };
            if _cond {
                event.set("opencti.indicator.rule_compatible", json!(true))?;
            }

            let _cond = { event.get_bool("opencti.indicator.rule_compatible") == Some(true) };
            if _cond {
                event.append_unique("tags", json!("detection-rule-candidate"))?;
            }

            let _cond = {
                event.get_str("opencti.indicator.pattern_type") == Some("kql")
                    || event.get_str("opencti.indicator.pattern_type") == Some("lucene")
            };
            if _cond {
                event.set("opencti.indicator.detection_rule.type", json!("query"))?;
            }

            let _cond = { event.get_str("opencti.indicator.pattern_type") == Some("eql") };
            if _cond {
                event.set("opencti.indicator.detection_rule.type", json!("eql"))?;
            }

            let _cond = { event.get_str("opencti.indicator.pattern_type") == Some("esql") };
            if _cond {
                event.set("opencti.indicator.detection_rule.type", json!("esql"))?;
            }

            let _cond = { event.get_bool("opencti.indicator.rule_compatible") == Some(true) };
            if _cond {
                if let Some(v) = event.get("opencti.indicator.pattern").cloned() {
                    event.set("opencti.indicator.detection_rule.query", v)?;
                }
            }

            let _cond = {
                event.has("threat.indicator.as")
                    || event.has("threat.indicator.email")
                    || event.has("threat.indicator.file")
                    || event.has("threat.indicator.ip")
                    || event.has("threat.indicator.port")
                    || event.has("threat.indicator.registry")
                    || event.has("threat.indicator.url")
                    || event.has("threat.indicator.x509")
                    || false
            };
            if _cond {
                event.append_unique("tags", json!("ecs-indicator-detail"))?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '-' || v == 'none' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '-' || v == 'none' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["-".into(), "none".into()],
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
