<!-- SPDX-License-Identifier: BUSL-1.1 -->
<!-- Copyright (c) 2026 HYPERI PTY LIMITED -->

# Schema Generation

This service transforms Beats/Agent JSON into normalised events. It is also the
only place that knows, per data stream, **what fields actually come out** — the
ingest pipelines declare intent, the compat corpus records what Elasticsearch
really emitted, and only here do the two meet.

That knowledge is published as ClickHouse schemas to
[dfe-schemas](https://github.com/hyperi-io/dfe-schemas), by pull request.

## Where the pieces live

| Repo | Role |
|---|---|
| `dfe-schemas` | The schemas themselves. Version-tree YAML, immutable published versions, 13 primitive types, the `@source:`/`@generated:` expression language. |
| `dfe-engine` | Defines how schemas work: `SchemaLoader`, `SchemaBuilderV2`, `MetaSchema`, the Elastic index-template importer, and `fieldmap/` for the sigma/ecs/cim views. |
| `dfe-transform-elastic` (here) | Generates the Elastic-side schemas and field maps. |
| `dfe-transform-elastic-dev` | Where generation runs. Dev generates, main ships. |

## The deployable unit is a data stream

Measured over the compat corpus:

| | |
|---|---|
| distinct leaf field paths | 50,109 |
| data streams / packages | 939 / 335 |
| median data stream width | 47 fields |
| widest package | 4,573 (amazon_security_lake) |
| distinct top-level namespaces | 1,010 |

A "complete filebeat" **table** is therefore not a thing anyone deploys. A
complete-filebeat **vocabulary** is. So:

- **Meta schema** — the shared definition every sub schema inherits (ECS).
- **Sub schema** — a package's own namespace, grouping its data streams.
- **Deployable table** — common header + meta + sub, at data-stream grain.

The grouping needs no new taxonomy. `src/registry.rs` already holds 1,092
entries shaped
`("<beat>.<module>.<pipeline>", transform, intake, "<package>.<data_stream>")`,
and the fourth element is the grouping key.

## Types come from `fields/*.yml`, not from the corpus

The corpus gives JSON types only. It cannot separate `keyword` from `text`
(a lowcardinality dimension against a fulltext index), or `long` from
`scaled_float` — and those distinctions are exactly what drive the DDL.

Types come from the integrations' `fields/*.yml`, pinned to the
`integrations_sha` that every corpus `meta.json` already records, so a generated
schema and the parity corpus describe the same upstream commit.

`fields/*.yml` also carries `external: ecs`, which is the only reliable
ECS/vendor discriminator. Inferring it from the top-level namespace is wrong in
both directions: `process.*` is ECS *and* vendor-extended, and some vendor
namespaces collide with ECS names.

## Port the importer, do not fork it

`dfe-engine`'s `services/schema/elastic_schema_service.py` walks
`mappings.properties` into `SchemaColumn`s. Two pieces must stay identical
across the Python and the Rust port, or the generated views break against the
generated tables:

- `_map_es_type` — the 13 primitives
- `_column_name_from_field_path` — dotted path to underscored column

A conformance test holds them together, the same way the compat corpus holds
the transforms to Elasticsearch.

## Every column carries its evidence

A schema built from Elastic's declaration alone creates columns that are always
NULL for fields we never emit. Each generated column records which it is:

| Evidence | Meaning |
|---|---|
| declared and emitted | In `fields/*.yml` and produced in the corpus. Ship it. |
| declared, never emitted | A parity gap, or a field this data never carries. |
| emitted, not declared | We produce it; the mapping does not declare it. |

The third row matters beyond schemas: the corpus ratchet counts only missing and
mismatched fields, so the 11,962 *extra* fields it reports are gated by nothing.
Schema generation is the first thing that would surface them.

## Regeneration must be deterministic

Published versions in dfe-schemas are immutable, so a regeneration **adds** a
version and never rewrites one. The same corpus plus the same `integrations_sha`
must produce byte-identical output — otherwise every run raises a spurious
version bump. That is a correctness property, and it has a test.

## Sigma, ECS and CIM mapping: data, not in-stream renaming

`dfe-engine` already builds sigma/ecs/cim ClickHouse VIEWs from field-map tables
(`fieldmap/FieldMapRegistry`, `ViewGenerator`, surfaced through
`SchemaBuilderV2`'s `view_ddls`). The format is
`mappings: {<standard field>: <column>}`.

We generate those tables. We do not rename fields in the stream:

1. The query-time mechanism already exists; in-stream duplicates it.
2. Renaming would destroy ECS, which our output *is* — `ecs.version` is stamped
   at 1,003 sites across 998 generated transform modules. It would bake one
   Sigma rule generation into stored data that can never be reinterpreted.
3. SigmaHQ ships rule changes constantly. As data, a change is a pull request;
   in-stream, it is a fleet redeploy plus a backfill.
4. Per-event renaming costs throughput for a query-time concern.
5. One ECS field serves several Sigma logsources; in-stream forces one choice.

Where no SigmaHQ logsource correspondence exists, we emit **nothing**. A wrong
mapping is worse than a missing one, because the rule then silently matches the
wrong column.

The ECS map needs no judgement — it is the dotted path with dots as
underscores, matching `_column_name_from_field_path`.
