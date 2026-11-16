# Parity

The goal is the same ECS output Elastic's ingest pipeline gives for each supported Beats and Elastic Agent integration. Parity is scored per source, and some sources match fully and others do not yet. The code map is [architecture.md](architecture.md), and what counts as a difference is [compat-policy.md](compat-policy.md).

This is an independent transform service, not a port of Elasticsearch. Data sent through DFE should get the field mapping, type coercion and enrichment it would get going straight to Elasticsearch. Where Elasticsearch's output is wrong, the service emits the correct value and records the divergence ([compat-policy.md](compat-policy.md#correct-beats-bug-compatible)).

[compat-policy.md](compat-policy.md) covers `tests/compare-policy.yaml`, which decides what counts as a difference.

This repository's floors run on every clone: `integration/`, `untested_sources.rs` and `unencumbered.rs` in `crates/dfe-transforms/tests/`, over the licence-clean samples in `tests/fixtures/unencumbered/` and the synthetic samples in `tests/fixtures/synthetic/`. They assert no panic, no new errors and enrichment, never parity ([testing.md](testing.md)).

## What we replicate

Beats and Elastic Agent send JSON events, and the ingest pipeline applies its processors in order to normalise, enrich and route them. Each row below is how the generated code does the same.

| Elastic component | dfe-transform-elastic equivalent | Status |
|---|---|---|
| Ingest processors | Rust per processor type, listed in [parsers.md](parsers.md#processor-taxonomy) | Done for every processor a shipped pipeline uses |
| Painless scripts | A hand-written runner from `crates/dfe-painless/src/bespoke/` where one is registered for the script, else a match against `params.rs` and the ladder in `common.rs`. An unrecognised script is counted and skipped | Partial, see [Painless coverage](parsers.md#painless-coverage) |
| `foreach` | A loop inside the transform function | Done |
| Pipeline chaining | A nested pipeline is inlined into the calling module between `// Begin nested pipeline` and `// End nested pipeline` markers, so there is no call on the hot path. `rg -c "Begin nested pipeline" crates/dfe-transforms/src/` counts the sites | Done |
| GeoIP enrichment | One MMDB enricher, found at startup, behind a bounded cache | Done, on DB-IP Lite |
| User agent parsing | Regex-based parser | Done, with small differences from Elastic's parser |
| Community ID | Hash-based network flow ID | Done |
| Conditions | Native Rust `if` per condition. A condition the transpiler cannot read emits `if false`, so the processor it guards never runs | Partial. `skipped_processors` in `tests/ratchets.json` counts them, with a per-source table |
| `on_failure` handlers | Generated around every processor that declares them, beside `if` and `ignore_failure` ([architecture.md](architecture.md#a-failed-processor-ends-in-one-of-four-places)) | Done |

The service leaves these out on purpose:

| Elastic component | Why not | Our approach |
|---|---|---|
| Beat or Agent collection | Events arrive from Kafka or a push, not from endpoints | Upstream |
| Transport parsing (syslog, CEF) | Done by the Beat or Agent, or by dfe-receiver for the syslog envelope | Upstream |
| Agent-side processors (`add_host_metadata`, `add_cloud_metadata`) | Run on the source machine | The fields arrive set |
| `@custom` pipelines | Per user, not part of an integration | Not applicable |
| `final_pipeline` | Elasticsearch index routing | Not applicable |
| `enrich` processor | Needs an Elasticsearch enrich index | dfe-loader's enrichment columns |
| `inference` processor | ML model execution | Not applicable |
| `reroute` processor | Elasticsearch index routing | Kafka topic routing |

The Beat or Agent collects the data, adds host and cloud metadata, runs its local processors and JSON-encodes the result before it reaches Kafka. From there the job is this service's: JSON parse, ingest pipeline logic, enrichment and ECS normalisation.

One difference is deliberate: `event.original` is not a second copy of the line. Elastic keeps it, and the default output here is the extracted JSON alone. When `pipeline.rs` emits an event:

- tagged `preserve_original_event` (Elastic's opt-in, which its pipelines add on failure), `event.original` stays.
- holding any field beyond the static and shipper fields in `ENVELOPE` (`crates/dfe-runtime/src/transform.rs`), `event.original` is removed.
- holding nothing else, so nothing was parsed, `event.original` moves back to `message`, counted on `events_unparsed_original_restored_total`.

A transform called directly still sets `event.original`.
