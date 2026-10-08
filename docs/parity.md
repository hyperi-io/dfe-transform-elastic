# Parity

What this service promises against Elastic's own output, what it deliberately leaves alone, and
how the claim is measured. The code map it sits beside is [architecture.md](architecture.md),
and the mechanics of the measurement are [compat.md](compat.md) and
[compat-policy.md](compat-policy.md).

**Goal:** Reliable 1:1 (or better) parity with Elastic ingest pipeline output for all
supported Beats and Elastic Agent integrations.

This is NOT reverse engineering Elasticsearch. We are building an independent transform
service that produces the same normalised ECS output as the Elastic ingest pipeline would.
Customers sending Beats/Agent data through DFE should get identical field mapping, type
coercion, and enrichment as if the data went directly to Elasticsearch.

**"Or better" means:** Where Elastic pipelines have known limitations (regex-only parsing,
single-threaded Painless execution, per-document GeoIP lookups), we can exceed their
performance while maintaining output compatibility.

---

## What we replicate

The Elastic ingest pipeline is the primary transform layer. Data arrives from Beats/Agent
as JSON events, and the ingest pipeline applies processors sequentially to normalise,
enrich, and route the data.

| Elastic Component | dfe-transform-elastic Equivalent | Status |
|-------------------|----------------------------------|--------|
| **Ingest processors** (27 used) | Rust processor implementations, one per Elastic processor type | Done |
| **Painless scripts** | A hand-transcribed runner from `crates/dfe-painless/src/bespoke/` where one is registered for that script, otherwise pattern-matched against `params.rs` and the ladder in `common.rs`; unrecognised scripts are skipped | see [Painless coverage](parsers.md#painless-coverage) -- the figure is a corpus-run output, not a constant, so re-derive it rather than quoting one from here |
| **Foreach processor** | Per-event loop inside the transform function | Okta, O365 |
| **Pipeline chaining** | A nested pipeline is INLINED into the calling module between `// Begin nested pipeline` / `// End nested pipeline` markers, so there is no call and no dispatch on the hot path | Done. `rg -c "Begin nested pipeline" crates/dfe-transforms/src/` counts today's call sites |
| **GeoIP enrichment** | Global MMDB enricher, auto-detected at startup, LRU-cached | Done (DB-IP Lite) |
| **User Agent parsing** | Regex-based parser | Done (minor diffs from Elastic UA parser) |
| **Community ID** | Hash-based network flow ID | Done |
| **Conditional evaluation** | Native Rust `if`/`match` expressions per condition pattern | Nearly. A condition the transpiler cannot read emits `if false`, making the processor it guards dead code -- `skipped_processors` in `tests/ratchets.json` is the live count, with a per-source table beside it |
| **On-failure handlers** | Wrapped around EVERY processor by the generator, alongside `if` and `ignore_failure`, so a new processor cannot quietly omit them. A raised error runs the handlers and records `_ingest.on_failure_message` and `_ingest.on_failure_processor_type` | Done |

---

## What we do not replicate

| Elastic Component | Why Not | Our Approach |
|-------------------|---------|--------------|
| Beat/Agent collection | We receive from Kafka, not from endpoints | Upstream responsibility |
| Transport parsing (syslog, CEF) | Done by Beat/Agent, or by dfe-receiver for the syslog envelope | Upstream responsibility |
| Agent-side processors (`add_host_metadata`, `add_cloud_metadata`) | Run on source machine | Fields arrive pre-populated in event |
| `@custom` pipelines | User-specific, not part of integration | Not applicable |
| `final_pipeline` | Elasticsearch-specific routing | Not applicable |
| `enrich` processor | Requires Elasticsearch enrich index | Alternative enrichment via dfe-loader |
| `inference` processor | ML model execution | Not applicable for DFE |
| `reroute` processor | Elasticsearch index routing | Kafka topic routing instead |

The Beat or Agent collects raw data, adds host/cloud metadata, applies its own local
processors and JSON-encodes the result before it ever reaches Kafka (see
[System context](architecture.md#system-context) for the full flow). Everything from there is
this service's job: JSON parse, ingest pipeline logic, enrichment, ECS normalisation.

---

## How parity is verified

**Parity is measured ONLY against the compat corpus.**
`crates/dfe-transforms/tests/compat_corpus.rs` compares per FIELD against output captured from a
real Elasticsearch and ratchets `tests/compat-baseline.json`, which holds every per-source score.
[compat.md](compat.md) and [compat-policy.md](compat-policy.md) are the full account: how the
corpus is generated, what `tests/compare-policy.yaml` excludes and why, and how the ratchet
refuses to be lowered.

The `.log` / `-expected.json` fixtures copied from Elastic predate the current pipelines and disagree with what Elasticsearch emits now, so **their parity assertions are retired**. They are Elastic-licensed and kept outside this repository, where a CI job runs `integration/` and `untested_sources.rs` over them as panic, error and enrichment floors. On a fresh clone of this repository, where neither they nor the corpus exist, the floors that hold are `unencumbered.rs` and the `public` cases in `untested_sources.rs`, over the licence-clean samples in `tests/fixtures/unencumbered/`.

Per-processor runtime status and code pattern: see
[Processor taxonomy](parsers.md#processor-taxonomy).
