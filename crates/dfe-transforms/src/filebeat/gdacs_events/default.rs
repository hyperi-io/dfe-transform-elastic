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
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field_to_root(event, "event.original", false)?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("event.original");
            }

            if event.has_value("properties.eventid") {
                event.rename("properties.eventid", "gdacs.event_id")?;
            }

            if event.has_value("properties.episodeid") {
                event.rename("properties.episodeid", "gdacs.episode_id")?;
            }

            if event.has_value("properties.eventtype") {
                event.rename("properties.eventtype", "gdacs.event_type")?;
            }

            if event.has_value("properties.eventname") {
                event.rename("properties.eventname", "gdacs.event_name")?;
            }

            if event.has_value("properties.name") {
                event.rename("properties.name", "gdacs.name")?;
            }

            if event.has_value("properties.description") {
                event.rename("properties.description", "gdacs.description")?;
            }

            if event.has_value("properties.htmldescription") {
                event.rename("properties.htmldescription", "gdacs.html_description")?;
            }

            if event.has_value("properties.glide") {
                event.rename("properties.glide", "gdacs.glide")?;
            }

            if event.has_value("properties.country") {
                event.rename("properties.country", "gdacs.country")?;
            }

            if event.has_value("properties.iso3") {
                event.rename("properties.iso3", "gdacs.iso3")?;
            }

            if event.has_value("properties.alertlevel") {
                event.rename("properties.alertlevel", "gdacs.alert_level")?;
            }

            if event.has_value("properties.alertscore") {
                event.rename("properties.alertscore", "gdacs.alert_score")?;
            }

            if event.has_value("properties.episodealertlevel") {
                event.rename("properties.episodealertlevel", "gdacs.episode_alert_level")?;
            }

            if event.has_value("properties.episodealertscore") {
                event.rename("properties.episodealertscore", "gdacs.episode_alert_score")?;
            }

            if event.has_value("properties.iscurrent") {
                event.rename("properties.iscurrent", "gdacs.is_current")?;
            }

            if event.has_value("properties.istemporary") {
                event.rename("properties.istemporary", "gdacs.is_temporary")?;
            }

            if event.has_value("properties.source") {
                event.rename("properties.source", "gdacs.source")?;
            }

            if event.has_value("properties.sourceid") {
                event.rename("properties.sourceid", "gdacs.source_id")?;
            }

            if event.has_value("properties.Class") {
                event.rename("properties.Class", "gdacs.class")?;
            }

            if event.has_value("properties.polygonlabel") {
                event.rename("properties.polygonlabel", "gdacs.polygon_label")?;
            }

            if event.has_value("geometry_doc") {
                event.rename("geometry_doc", "gdacs.geometry_doc")?;
            }

            if event.has_value("geometry_id") {
                event.rename("geometry_id", "gdacs.geometry_id")?;
            }

            if event.has_value("geometry_role") {
                event.rename("geometry_role", "gdacs.geometry_role")?;
            }

            if event.has_value("properties.icon") {
                event.rename("properties.icon", "gdacs.icon")?;
            }

            if event.has_value("properties.url.report") {
                event.rename("properties.url.report", "gdacs.url.report")?;
            }

            if event.has_value("properties.url.details") {
                event.rename("properties.url.details", "gdacs.url.details")?;
            }

            if event.has_value("properties.url.geometry") {
                event.rename("properties.url.geometry", "gdacs.url.geometry")?;
            }

            if event.has_value("properties.severitydata") {
                event.rename("properties.severitydata", "gdacs.severity_raw")?;
            }

            if event.has_value("properties.affectedcountries") {
                event.rename("properties.affectedcountries", "gdacs.affected_countries")?;
            }

            if event.has_value("gdacs.severity_raw.severity") {
                event.rename("gdacs.severity_raw.severity", "gdacs.severity.value")?;
            }

            if event.has_value("gdacs.severity_raw.severitytext") {
                event.rename("gdacs.severity_raw.severitytext", "gdacs.severity.text")?;
            }

            if event.has_value("gdacs.severity_raw.severityunit") {
                event.rename("gdacs.severity_raw.severityunit", "gdacs.severity.unit")?;
            }

            event.remove("gdacs.severity_raw");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("properties.fromdate") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "properties.fromdate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_fromdate")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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
                if let Some(date_str) = event.get_as_string("properties.todate") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "properties.todate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_todate")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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
                if let Some(date_str) = event.get_as_string("properties.datemodified") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.modified", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "properties.datemodified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_datemodified")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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

            if let Some(v) = event
                .get("event.modified")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { !event.has_value("event.modified") };
            if _cond {
                if let Some(v) = event
                    .get("event.start")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            // Painless script, resolved to its runners at generation time
            // Source: if (ctx.gdacs?.is_current != null) {\n  def val = ctx.gdacs.is_current.toString().toLowerCase();\n  ctx.gdacs.is_current = (val == \"true\" || val == \"1\");\n}\nif (ctx.gdacs?.is_temporary != null) {\n  def val = ctx.gdacs.is_temporary.toString().toLowerCase();\n  ctx.gdacs.is_temporary = (val == \"true\" || val == \"1\");\n}\n
            coerce_boolean(
                event,
                &CoerceBoolean::new(
                    vec![
                        ("gdacs.is_current".to_owned(), "gdacs.is_current".to_owned()),
                        (
                            "gdacs.is_temporary".to_owned(),
                            "gdacs.is_temporary".to_owned(),
                        ),
                    ],
                    vec!["true".to_owned(), "1".to_owned()],
                ),
            );

            // Painless script
            // Source: String gdacsRingToWkt(def ring) {\n  StringBuilder builder = new StringBuilder();\n  builder.append(\"(\");\n  for (int i = 0; i < ring.size(); i++) {\n    if (i > 0) { builder.append(\", \"); }\n    def point = ring[i];\n    builder.append(point[0].toString());\n    builder.append(\" \");\n    builder.append(point[1].toString());\n  }\n  builder.append(\")\");\n  return builder.toString();\n}\n\nString gdacsPolygonToWkt(def rings) {\n  StringBuilder builder = new StringBuilder();\n  builder.append(\"(\");\n  for (int i = 0; i < rings.size(); i++) {\n    if (i > 0) { builder.append(\", \"); }\n    builder.append(gdacsRingToWkt(rings[i]));\n  }\n  builder.append(\")\");\n  return builder.toString();\n}\n\nString gdacsShapeToWkt(def geom) {\n  if (geom == null || geom.coordinates == null || geom.type == null) {\n    return null;\n  }\n  if (geom.type == \"LineString\") {\n    return \"LINESTRING \" + gdacsRingToWkt(geom.coordinates);\n  }\n  if (geom.type == \"MultiLineString\") {\n    StringBuilder builder = new StringBuilder();\n    builder.append(\"MULTILINESTRING (\");\n    for (int i = 0; i < geom.coordinates.size(); i++) {\n      if (i > 0) { builder.append(\", \"); }\n      builder.append(gdacsRingToWkt(geom.coordinates[i]));\n    }\n    builder.append(\")\");\n    return builder.toString();\n  }\n  if (geom.type == \"Polygon\") {\n    return \"POLYGON \" + gdacsPolygonToWkt(geom.coordinates);\n  }\n  if (geom.type == \"MultiPolygon\") {\n    StringBuilder builder = new StringBuilder();\n    builder.append(\"MULTIPOLYGON (\");\n    for (int i = 0; i < geom.coordinates.size(); i++) {\n      if (i > 0) { builder.append(\", \"); }\n      builder.append(gdacsPolygonToWkt(geom.coordinates[i]));\n    }\n    builder.append(\")\");\n    return builder.toString();\n  }\n  return null;\n}\n\nif (ctx.gdacs == null) { ctx.gdacs = new HashMap(); }\nif (ctx.gdacs.geo == null) { ctx.gdacs.geo = new HashMap(); }\n\n// Extract centroid from the event-level Point geometry.\ndef geom = ctx.geometry;\nif (geom != null) {\n  String geomType = geom.type;\n  if (geomType == \"Point\" && geom.coordinates != null && geom.coordinates.size() >= 2) {\n    ctx.gdacs.geo.location = ['lon': geom.coordinates[0], 'lat': geom.coordinates[1]];\n  }\n}\n\n// Extract affected area from the enriched polygon geometry.\ndef polyGeom = ctx.polygon_geometry;\nif (polyGeom != null) {\n  String polyType = polyGeom.type;\n  if (polyType == \"Polygon\" || polyType == \"MultiPolygon\" || polyType == \"LineString\" || polyType == \"MultiLineString\") {\n    if (ctx.gdacs == null) { ctx.gdacs = new HashMap(); }\n    ctx.gdacs.affected_area = gdacsShapeToWkt(polyGeom);\n    ctx.gdacs.geometry_type = polyType;\n\n    // Update class and polygon_label from enrichment if present.\n    if (ctx.polygon_class != null) { ctx.gdacs.class = ctx.polygon_class; }\n    if (ctx.polygon_label != null && ctx.polygon_label != \"\") { ctx.gdacs.polygon_label = ctx.polygon_label; }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String gdacsRingToWkt(def ring) {\n  StringBuilder builder = new StringBuilder();\n  builder.append(\"(\");\n  for (int i = 0; i < ring.size(); i++) {\n    if (i > 0) { builder.append(\", \"); }\n    def point = ring[i];\n    builder.append(point[0].toString());\n    builder.append(\" \");\n    builder.append(point[1].toString());\n  }\n  builder.append(\")\");\n  return builder.toString();\n}\n\nString gdacsPolygonToWkt(def rings) {\n  StringBuilder builder = new StringBuilder();\n  builder.append(\"(\");\n  for (int i = 0; i < rings.size(); i++) {\n    if (i > 0) { builder.append(\", \"); }\n    builder.append(gdacsRingToWkt(rings[i]));\n  }\n  builder.append(\")\");\n  return builder.toString();\n}\n\nString gdacsShapeToWkt(def geom) {\n  if (geom == null || geom.coordinates == null || geom.type == null) {\n    return null;\n  }\n  if (geom.type == \"LineString\") {\n    return \"LINESTRING \" + gdacsRingToWkt(geom.coordinates);\n  }\n  if (geom.type == \"MultiLineString\") {\n    StringBuilder builder = new StringBuilder();\n    builder.append(\"MULTILINESTRING (\");\n    for (int i = 0; i < geom.coordinates.size(); i++) {\n      if (i > 0) { builder.append(\", \"); }\n      builder.append(gdacsRingToWkt(geom.coordinates[i]));\n    }\n    builder.append(\")\");\n    return builder.toString();\n  }\n  if (geom.type == \"Polygon\") {\n    return \"POLYGON \" + gdacsPolygonToWkt(geom.coordinates);\n  }\n  if (geom.type == \"MultiPolygon\") {\n    StringBuilder builder = new StringBuilder();\n    builder.append(\"MULTIPOLYGON (\");\n    for (int i = 0; i < geom.coordinates.size(); i++) {\n      if (i > 0) { builder.append(\", \"); }\n      builder.append(gdacsPolygonToWkt(geom.coordinates[i]));\n    }\n    builder.append(\")\");\n    return builder.toString();\n  }\n  return null;\n}\n\nif (ctx.gdacs == null) { ctx.gdacs = new HashMap(); }\nif (ctx.gdacs.geo == null) { ctx.gdacs.geo = new HashMap(); }\n\n// Extract centroid from the event-level Point geometry.\ndef geom = ctx.geometry;\nif (geom != null) {\n  String geomType = geom.type;\n  if (geomType == \"Point\" && geom.coordinates != null && geom.coordinates.size() >= 2) {\n    ctx.gdacs.geo.location = ['lon': geom.coordinates[0], 'lat': geom.coordinates[1]];\n  }\n}\n\n// Extract affected area from the enriched polygon geometry.\ndef polyGeom = ctx.polygon_geometry;\nif (polyGeom != null) {\n  String polyType = polyGeom.type;\n  if (polyType == \"Polygon\" || polyType == \"MultiPolygon\" || polyType == \"LineString\" || polyType == \"MultiLineString\") {\n    if (ctx.gdacs == null) { ctx.gdacs = new HashMap(); }\n    ctx.gdacs.affected_area = gdacsShapeToWkt(polyGeom);\n    ctx.gdacs.geometry_type = polyType;\n\n    // Update class and polygon_label from enrichment if present.\n    if (ctx.polygon_class != null) { ctx.gdacs.class = ctx.polygon_class; }\n    if (ctx.polygon_label != null && ctx.polygon_label != \"\") { ctx.gdacs.polygon_label = ctx.polygon_label; }\n  }\n}\n"#
                ),
            )?;

            // Painless script
            // Source: def countries = ctx.gdacs?.affected_countries;\nif (countries == null || countries.size() == 0) { return; }\n\ndef names = new ArrayList();\ndef iso2_codes = new ArrayList();\ndef iso3_codes = new ArrayList();\n\nfor (def c : countries) {\n  if (c.countryname != null) { names.add(c.countryname); }\n  if (c.iso2 != null) { iso2_codes.add(c.iso2); }\n  if (c.iso3 != null) { iso3_codes.add(c.iso3); }\n}\n\nctx.gdacs.affected_country_names = names;\nctx.gdacs.affected_country_iso2 = iso2_codes;\nctx.gdacs.affected_country_iso3 = iso3_codes;\n\nif (ctx.gdacs == null) { ctx.gdacs = new HashMap(); }\nif (ctx.gdacs.geo == null) { ctx.gdacs.geo = new HashMap(); }\nif (iso2_codes.size() > 0) {\n  ctx.gdacs.geo.country_iso_code = iso2_codes[0];\n}\nif (names.size() > 0) {\n  ctx.gdacs.geo.country_name = names[0];\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def countries = ctx.gdacs?.affected_countries;\nif (countries == null || countries.size() == 0) { return; }\n\ndef names = new ArrayList();\ndef iso2_codes = new ArrayList();\ndef iso3_codes = new ArrayList();\n\nfor (def c : countries) {\n  if (c.countryname != null) { names.add(c.countryname); }\n  if (c.iso2 != null) { iso2_codes.add(c.iso2); }\n  if (c.iso3 != null) { iso3_codes.add(c.iso3); }\n}\n\nctx.gdacs.affected_country_names = names;\nctx.gdacs.affected_country_iso2 = iso2_codes;\nctx.gdacs.affected_country_iso3 = iso3_codes;\n\nif (ctx.gdacs == null) { ctx.gdacs = new HashMap(); }\nif (ctx.gdacs.geo == null) { ctx.gdacs.geo = new HashMap(); }\nif (iso2_codes.size() > 0) {\n  ctx.gdacs.geo.country_iso_code = iso2_codes[0];\n}\nif (names.size() > 0) {\n  ctx.gdacs.geo.country_name = names[0];\n}\n"#
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gdacs.severity.value") {
                    if let Some(val) = event.get("gdacs.severity.value") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.severity.value".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.severity.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_severity_value")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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
                if event.has_value("gdacs.alert_score") {
                    if let Some(val) = event.get("gdacs.alert_score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.alert_score".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.alert_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_alert_score")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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
                if event.has_value("gdacs.episode_alert_score") {
                    if let Some(val) = event.get("gdacs.episode_alert_score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.episode_alert_score".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.episode_alert_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_episode_alert_score",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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
                if event.has_value("gdacs.event_id") {
                    if let Some(val) = event.get("gdacs.event_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.event_id".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.event_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_event_id")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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
                if event.has_value("gdacs.episode_id") {
                    if let Some(val) = event.get("gdacs.episode_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.episode_id".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.episode_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_episode_id")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
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

            // Painless script
            // Source: def typeMap = [\n  'EQ': 'Earthquake',\n  'TC': 'Tropical Cyclone',\n  'FL': 'Flood',\n  'VO': 'Volcano',\n  'DR': 'Drought',\n  'WF': 'Wildfire'\n];\ndef code = ctx.gdacs?.event_type;\nif (code != null && typeMap.containsKey(code)) {\n  ctx.gdacs.event_type_name = typeMap[code];\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def typeMap = [\n  'EQ': 'Earthquake',\n  'TC': 'Tropical Cyclone',\n  'FL': 'Flood',\n  'VO': 'Volcano',\n  'DR': 'Drought',\n  'WF': 'Wildfire'\n];\ndef code = ctx.gdacs?.event_type;\nif (code != null && typeMap.containsKey(code)) {\n  ctx.gdacs.event_type_name = typeMap[code];\n}\n"#
                ),
            )?;

            // Painless script
            // Source: def level = ctx.gdacs?.alert_level;\nif (level == null) { return; }\nlevel = level.toLowerCase();\nif (level == \"red\") {\n  ctx.event.severity = 3;\n  ctx.event.risk_score = 90.0;\n} else if (level == \"orange\") {\n  ctx.event.severity = 2;\n  ctx.event.risk_score = 60.0;\n} else if (level == \"green\") {\n  ctx.event.severity = 1;\n  ctx.event.risk_score = 30.0;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def level = ctx.gdacs?.alert_level;\nif (level == null) { return; }\nlevel = level.toLowerCase();\nif (level == \"red\") {\n  ctx.event.severity = 3;\n  ctx.event.risk_score = 90.0;\n} else if (level == \"orange\") {\n  ctx.event.severity = 2;\n  ctx.event.risk_score = 60.0;\n} else if (level == \"green\") {\n  ctx.event.severity = 1;\n  ctx.event.risk_score = 30.0;\n}\n"#
                ),
            )?;

            event.set("ecs.version", json!("9.4.0"))?;

            event.set("event.kind", json!("alert"))?;

            event.set("event.dataset", json!("gdacs.events"))?;

            event.set("event.module", json!("gdacs"))?;

            event.set("event.provider", json!("gdacs"))?;

            // Painless script, resolved to its runners at generation time
            // Source: if (ctx.event == null) { ctx.event = new HashMap(); }\ndef parts = new ArrayList();\nif (ctx.gdacs?.event_id != null) { parts.add(ctx.gdacs.event_id.toString()); }\nif (ctx.gdacs?.episode_id != null) { parts.add(ctx.gdacs.episode_id.toString()); }\nif (ctx.gdacs?.geometry_id != null) { parts.add(ctx.gdacs.geometry_id.toString()); }\nif (parts.size() > 0) {\n  ctx.event.id = String.join(\"-\", parts);\n}\n
            join_present_fields(
                event,
                &JoinPresentFields::new(
                    vec![
                        "gdacs.event_id".to_owned(),
                        "gdacs.episode_id".to_owned(),
                        "gdacs.geometry_id".to_owned(),
                    ],
                    "-",
                    "event.id",
                ),
            );

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if let Some(v) = event
                .get("gdacs.url.report")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            if let Some(v) = event
                .get("gdacs.url.details")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "message",
                    json!(
                        event
                            .get("gdacs.description")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if let Some(v) = event
                .get("gdacs.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("gdacs.geo.name", v)?;
            }

            event.remove("properties");
            event.remove("geometry");
            event.remove("bbox");
            event.remove("type");
            event.remove("polygon_geometry");
            event.remove("polygon_class");
            event.remove("polygon_label");
            event.remove("geometry_doc");
            event.remove("geometry_id");
            event.remove("geometry_role");
            event.remove("gdacs.geometry_type");
            event.remove("gdacs.geometry_doc");
            event.remove("gdacs.geometry_id");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
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
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
