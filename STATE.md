## Design Philosophy — READ FIRST

**This is a deliberate black-box application service.** Not a library. Not a framework. Not general purpose.

**What it does:** Beats/Elastic Agent JSON in → DFE-parsed, normalised events out.

**How it does it:** Optimised within an inch of its life. Every code path is scrutinised for:
- **Batch processing** over row-by-row iteration (default: 20k Kafka batch size)
- **SIMD acceleration** for JSON parsing, string search, byte operations
- **Zero-copy** where possible — borrow from Kafka message buffers, don't clone
- **Pre-allocation** — all structures sized at init, not grown on the hot path
- **Columnar access patterns** where field operations can be vectorised across a batch

**At every step** we look for opportunities to refactor from row-iterate to batch-process, from regex to native parse, from allocate to borrow.

**Codegen automation** is critical — Elastic pipeline YAML → Rust code generation must be automated. Even if codegen gets 70% right, we hand-tune the rest. The alternative (hand-writing 70+ transforms) doesn't scale.

---

## CI

CI is live via `hyperi-ci`. Run `hyperi-ci check` (or `make check`) locally before pushing.

---

## Logger / Metrics

**Handled independently by Derek.** Do not implement bespoke logging or metrics infrastructure — `hyperi-rustlib` logger and metrics features will be integrated separately. Use `tracing` macros for now (info!, warn!, error!) but don't set up subscribers or exporters.

---

# Project State

**Project:** dfe-transform-elastic
**DFE:** Data Fusion Engine
**Purpose:** Black-box Rust service — Beats/Agent JSON in, DFE-parsed events out, optimised for throughput

---

## Licensing

| Component | Value |
|-----------|-------|
| License | FSL-1.1-ALv2 (Functional Source License) |
| Licensor | HYPERI PTY LIMITED (ABN 31 622 581 748) |
| SPDX ID | `FSL-1.1-ALv2` |
| Apache 2.0 Conversion | 2 years after each release |

**Source file headers:**
```rust
// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED
```

---

## Build Settings

```bash
export CARGO_BUILD_JOBS=2
```

**SIMD Flags:** `.cargo/config.toml` configures `-C target-cpu=native` for simd-json, memchr, aho-corasick SIMD optimisations. Release builds target x86-64-v3 (AVX2).

---

## Architecture

### Data Flow

```
Kafka (20k batch) -> simd-json batch deserialise -> Rust transforms -> batch output
                                                          |
                                          ┌───────────────┼───────────────┐
                                          │               │               │
                                     field extract   field mutate    enrichment
                                     (native parse,  (rename, set,   (geoip, ua,
                                      zero-copy)      convert, etc)   community_id)
```

**Input:** JSON event batches from Kafka (filebeat, winlogbeat, metricbeat, auditbeat, heartbeat, packetbeat, elastic-agent)
**Transform:** Elastic ingest pipeline logic (Painless scripts + processors) converted to native Rust
**Output:** Transformed, normalised event batches

### Performance Model

- **Batch size:** 20,000 events default (configurable)
- **JSON deserialise:** simd-json with known-key for ECS fields, zero-copy from Kafka buffer
- **Transform:** Compiled Rust — no interpreter, no VM, no regex on hot paths
- **Enrichment:** Pre-loaded databases (GeoIP MMDB, UA regexes pre-compiled with OnceLock)
- **Target:** Process batches at wire speed — transform should not be the bottleneck

### Workspace Crates

| Crate | Purpose |
|-------|---------|
| `dfe-parse` | High-performance parser library replacing grok/regex with native Rust |
| `dfe-runtime` | Core runtime: Event type, Transform trait, enrichment modules |
| `dfe-codegen` | Elastic ingest pipeline to Rust code generator (forked from elastic_to_vrl) |
| `dfe-transforms` | Generated + hand-tuned transform modules per Beats source |

### Codegen Workflow

```
1. Elastic pipeline YAML -> dfe-codegen -> .rs transform files (automated, ~70% coverage)
2. Hand-tune remaining ~30% (complex conditionals, Painless scripts)
3. Generated .rs files use dfe-runtime::prelude::* and dfe-parse parsers
4. cargo test validates 1:1 against Beats/Agent test data
```

**Codegen must be automated.** Manual transform writing doesn't scale across 70+ pipeline files. Codegen provides the rough start; we optimise from there.

---

## Key Decisions

### Black-Box Service (Not a Library)

**Decision:** dfe-transform-elastic is a standalone service, not a reusable library.
**Rationale:** Purpose-built for one job: transform Elastic data at maximum throughput. Every abstraction must justify its performance cost. No plugin system, no user-extensible transforms, no general-purpose event processing.

### Batch-First Processing

**Decision:** Default 20k event Kafka batches. All operations designed for batch processing.
**Rationale:** Amortises per-event overhead (Kafka commit, serialisation, memory allocation). Enables SIMD vectorisation across batch fields. Enables columnar access patterns for field operations.

### Monorepo (Converter + Runtime Together)

**Decision:** elastic_to_rust converter lives inside this repo, not separately.
**Rationale:** Tight coupling between generated code and runtime API. Single CI validates everything end-to-end. Atomic changes when API evolves.

### Grok/Regex -> Native Rust Parsers

**Decision:** Replace grok patterns and regexes with zero-copy native Rust parsers.
**Rationale:** Grok/regex is the #1 hot-path bottleneck (10-20x slower than purpose-built parsers). Three-layer approach: Layer 1 (common pattern replacements), Layer 2 (composite parser codegen), Layer 3 (pre-compiled DFA fallback).

### winnow Over nom

**Decision:** Use winnow for parser combinators in new code.
**Rationale:** Better DX, improved error messages, successor to nom. nom used in elastic_to_vrl (reused as-is in codegen).

### simd-json for Kafka Deserialization

**Decision:** simd-json with `known-key` feature for JSON parsing.
**Rationale:** 2-3x faster than serde_json. known-key optimises ECS field access. Zero-copy BorrowedValue from Kafka message buffer.

### Painless Parser as Separate Component

**Decision:** Keep the ANTLR4 Painless parser as its own component within dfe-codegen, callable independently.
**Rationale:** Reused directly from elastic_to_vrl. May have future uses beyond codegen (analysis, validation). Clean separation of concerns.

### Enrichment Ownership (GeoIP, Reputation, Community ID, User Agent)

**Decision:** dfe-loader is the primary owner of GeoIP and IP reputation enrichment. dfe-transform-elastic re-uses dfe-loader's implementation. When running standalone, dfe-transform-elastic performs enrichment itself.
**Rationale:** GeoIP is used in ~81% of Elastic pipelines (21 pipeline files, 81 lookup calls). dfe-loader already has a mature implementation. Two modes: standalone (enrichment runs locally) and pass-through (dfe-loader handles it downstream).

### Hot-Reload vs Restart Config (Phase 5)

**Hot-reloaded (takes effect on next batch):**
- `retry.max_retries`, `retry.dlq_topic`
- `scaling.enabled`, `scaling.*`
- `source.batch_size`

**Requires pod restart:**
- `source.transport.*` — Kafka consumer connections established at startup
- `sink.transport.*` — output connections established at startup
- `enrichment.*` — MMDB databases loaded at startup
- `http.*` — health endpoint binds at startup
- `pipeline_name` — metrics labels set at startup

---

## External Dependencies

### Related Projects

- **elastic_to_vrl** (`/projects/elastic_to_vrl/`) - Predecessor: Elastic -> VRL converter. ~70-80% codebase reused.
- **dfe-vector-templates** (`/projects/dfe-vector-templates/`) - Existing VRL transform templates being replaced.
- **dfe-loader** (`/projects/dfe-loader/`) - Downstream consumer: Kafka -> ClickHouse loader.
- **hyperi-rustlib** (crates.io) - Shared library for config, logger, metrics, transport.

### Elastic Sources

- [elastic/beats](https://github.com/elastic/beats) - filebeat, winlogbeat, metricbeat, auditbeat, heartbeat, packetbeat
- [elastic/elastic-agent](https://github.com/elastic/elastic-agent) - Elastic Agent integrations
- Test data: 1,691 fixture files in `testdata/integrations/` (copied from elastic_to_vrl)

---

## Resources

- [RESEARCH.md](RESEARCH.md) - Full research on parsing libraries, architecture, effort estimation
- [docs/](docs/) - Architecture and design documentation
- [elastic_to_vrl ADRs](/projects/elastic_to_vrl/adr/) - Architecture Decision Records from predecessor

---

## Notes for AI Assistants

This file contains **static project context only**.

**DO NOT add:** version numbers, progress/tasks (use TODO.md), dates, session history.
**DO add:** architecture decisions, key component descriptions, how things work.

**DO NOT implement:** logger or metrics infrastructure (Derek is handling separately with hyperi-rustlib).
**DO implement:** transform logic, codegen improvements, parser optimisations, batch processing patterns.

**Performance mindset:** At every change, ask: can this be batched? Can this avoid allocation? Can this use SIMD? Can this borrow instead of clone? If the answer is yes, do it.
