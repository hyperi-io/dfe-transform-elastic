<!-- SPDX-License-Identifier: BUSL-1.1 -->
<!-- Copyright (c) 2026 HYPERI PTY LIMITED -->

# Schema generation

Status: partly built, and none of it in this repository. The ECS meta schema in dfe-schemas, `meta/elastic/ecs.yaml`, is generated. Per-source package schemas, the evidence each column records and the Sigma field maps below are design, not code. [#16](https://github.com/hyperi-io/dfe-transform-elastic/issues/16) tracks generating schemas for every Beat and Elastic Agent source.

The Elastic side of DFE needs one place that knows, per data stream, which fields come out. The ingest pipelines declare intent and the transforms produce the output, so the two meet where the transforms are built. That knowledge goes to [dfe-schemas](https://github.com/hyperi-io/dfe-schemas) as ClickHouse schemas, by pull request. dfe-engine defines how schemas work (`SchemaLoader`, `SchemaBuilderV2`, `MetaSchema`), imports Elastic index templates, and builds the sigma, ecs and cim views in `fieldmap/`.

## The design

The deployable unit is a data stream. A table holding every filebeat field is not something anyone deploys, and a filebeat vocabulary is. So a schema has three layers:

- the meta schema, the ECS fields every sub schema inherits.
- a sub schema, one package's namespace, grouping its data streams.
- a deployable table, the common header plus meta plus sub, at data-stream grain.

The grouping key exists: every entry in `src/registry.rs` is `("<beat>.<module>.<pipeline>", transform, intake, "<package>.<data_stream>")`. `dfe-transform-elastic sources` prints the first element.

Types come from each package's `fields/*.yml`, not from output. Output gives JSON types alone, and cannot separate `keyword` from `text` or `long` from `scaled_float`, which are the distinctions that drive the DDL. `external: ecs` there is the reliable ECS discriminator: inferring it from the top-level namespace is wrong both ways, because `process.*` is ECS and extended by packages alike. A package's `fields.yml` gives the ECS field names it uses but no types, so the ECS layer takes its types from the ECS spec.

dfe-engine's `services/schema/elastic_schema_service.py` walks an index template's `mappings.properties`, and `fields.yml` is a different layout, so its `_walk_mapping` does not transfer. Two functions must stay identical across the Python and any port, or the generated views break against the generated tables: `_map_es_type` and `_column_name_from_field_path`.

Each column would record its evidence: declared and emitted, declared and never emitted (a parity gap, or a field this data never has), or emitted and not declared. Published versions in dfe-schemas are immutable, so a regeneration adds a version, and the same inputs must produce byte-identical output or every run raises a spurious version.

Sigma, ECS and CIM mappings are data, not in-stream renaming. dfe-engine builds those views from field-map tables (`fieldmap/FieldMapRegistry`, `ViewGenerator`, surfaced through `SchemaBuilderV2`'s `view_ddls`) in the form `mappings: {<standard field>: <column>}`, and the generator would write those tables:

1. The query-time mechanism exists, and renaming in the stream would duplicate it.
2. Renaming would destroy ECS, which the output is: nearly every generated module stamps `ecs.version` (`rg -c ecs.version crates/dfe-transforms/src/` counts them).
3. SigmaHQ changes rules often. As data a change is a pull request, and in the stream it is a fleet redeploy plus a backfill.
4. Per-event renaming costs throughput for a query-time concern.
5. One ECS field serves several Sigma logsources, and renaming in the stream forces one choice.

Where no SigmaHQ logsource corresponds, the map has no entry. A wrong mapping is worse than a missing one, because the rule then matches the wrong column. The ECS map needs no judgement: `_column_name_from_field_path` drops a leading `@` from each segment and joins the segments with underscores.
