## CI UNAVAILABLE — Commit with [skip ci]

The CI system is being completely rewritten. Until migration instructions are provided:

- **All commits MUST include `[skip ci]` in the commit message**
- Do not trigger CI runs or rely on CI for validation
- Run `./ci/local-build.sh` for local validation if it exists

---

# Project State

**Project:** dfe-transform-elastic
**DFE:** Data Fusion Engine
**Purpose:** Rust-optimised transform pipeline for Elastic Stack data (Beats + Elastic Agent) ingested via Kafka
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

See [LICENSE](LICENSE) for details.

---

## Build Settings

```bash
export CARGO_BUILD_JOBS=2
```

**SIMD Flags:** `.cargo/config.toml` configures `-C target-cpu=native` for simd-json, memchr, aho-corasick SIMD optimisations.

---

## Architecture

### Data Flow

```
Kafka (JSON) -> simd-json deserialise -> Rust transforms -> output
                                              |
                              ┌───────────────┼───────────────┐
                              │               │               │
                         field extract   field mutate    enrichment
                         (grok/regex     (rename, set,   (geoip, ua,
                          -> efficient    convert, etc)   community_id)
                          native parse)
```

**Input:** JSON events from Kafka (filebeat, winlogbeat, metricbeat, auditbeat, heartbeat, packetbeat, elastic-agent)
**Transform:** Elastic ingest pipeline logic (Painless scripts + processors) converted to native Rust
**Output:** Transformed, normalised events

### Workspace Crates

| Crate | Purpose |
|-------|---------|
| `dfe-parse` | High-performance parser library replacing grok/regex with native Rust |
| `dfe-runtime` | Core runtime: Event type, Transform trait, enrichment modules |
| `dfe-codegen` | Elastic ingest pipeline to Rust code generator (forked from elastic_to_vrl) |
| `dfe-transforms` | Generated + hand-tuned transform modules per Beats source |

### Codegen Workflow

```
1. Elastic pipeline YAML -> dfe-codegen -> .rs transform files
2. Generated .rs files use dfe-runtime::prelude::* and dfe-parse parsers
3. cargo test runs 1:1 tests against Beats/Agent test data
```

---

## Key Decisions

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
**Rationale:** GeoIP is used in ~81% of Elastic pipelines (21 pipeline files, 81 lookup calls). dfe-loader already has a mature implementation: `src/enrich/geoip.rs` (LRU cache, private IP fast-path, City+ASN), `src/enrich/reputation.rs` (multi-source blocklists), `src/enrich/risk.rs` (composite scoring). Duplicating this is wasteful. Two modes: standalone (enrichment runs locally with auto-downloaded databases) and pass-through (dfe-loader handles it downstream).

**Enrichment usage in Elastic pipelines:**
- GeoIP: 21/26 pipeline files (~81%), City + ASN databases
- Community ID: 11/26 files (~52%), network-flow logs only
- User Agent: 14/26 files (~67%), web/auth logs
- All three are CPU-bound (no external API calls at runtime)

---

## External Dependencies

### Related Projects

- **elastic_to_vrl** (`/projects/elastic_to_vrl/`) - Predecessor: Elastic -> VRL converter. ~70-80% codebase reused.
- **dfe-vector-templates** (`/projects/dfe-vector-templates/`) - Existing VRL transform templates being replaced.
- **dfe-loader** (`/projects/dfe-loader/`) - Downstream consumer: Kafka -> ClickHouse loader.

### Elastic Sources

- [elastic/beats](https://github.com/elastic/beats) - filebeat, winlogbeat, metricbeat, auditbeat, heartbeat, packetbeat
- [elastic/elastic-agent](https://github.com/elastic/elastic-agent) - Elastic Agent integrations
- Test data from Beats module directories (`module/<name>/test/`)

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
