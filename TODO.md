# TODO - dfe-transform-elastic

**Project Goal:** Rust-optimised transform pipeline for Elastic Stack data (Beats + Agent) via Kafka

**Target:** Replace VRL transforms with native Rust for 10-20x hot-path performance improvement

**Reference:** `/projects/elastic_to_vrl/` (predecessor), `/projects/dfe-vector-templates/` (existing VRL)

**Architecture:** Cargo workspace with 4 crates: dfe-parse, dfe-runtime, dfe-codegen, dfe-transforms

**Design:** [docs/DESIGN.md](docs/DESIGN.md) — architecture, Mermaid diagrams, API contracts

---

## Phase 0: Design

### 0.1 Architecture Design Document — IN PROGRESS

- [x] Write `docs/DESIGN.md` with system context, crate graph, data flow, API contracts
- [x] Mermaid diagrams: system context, crate deps, event lifecycle, class diagram, transform pipeline, parser layers, codegen flow, enrichment, testing
- [ ] Review and finalise — ensure DESIGN.md aligns with implementation as work progresses

> **Done when:** `docs/DESIGN.md` exists, covers all 4 crates, all API contracts have type signatures.

---

## Phase 1: Foundation

> **Parallelism:** 1.2 (dfe-runtime) and 1.3 (dfe-parse) are fully independent and can run concurrently.

### 1.1 Project Scaffolding — COMPLETE

- [x] Git repo initialised, ai/ci submodules attached
- [x] Cargo workspace with 4 crates (dfe-parse, dfe-runtime, dfe-codegen, dfe-transforms)
- [x] `.cargo/config.toml` with SIMD flags and Artifactory registry
- [x] CI/CD configuration (.hyperi-ci.yaml, workflows, semantic-release)
- [x] rust-toolchain.toml (1.83), clippy.toml, .gitignore, .gitleaks.toml
- [x] STATE.md, RESEARCH.md, LICENSE
- [x] Workspace compiles clean (`cargo check` passes)

### 1.2 dfe-runtime Crate — COMPLETE

- [x] Event struct wrapping `serde_json::Value` with constructors (`new`, `from_json`, `from_bytes`)
- [x] Dotted-path navigation + typed getters (`get`, `get_str`, `get_i64`, `get_f64`, `get_bool`, `get_array`, `get_object`, `has`)
- [x] Setters: `set` (auto-creates intermediates), `remove`, `rename`
- [x] Array operations: `append` (create if missing), `merge` (shallow/deep)
- [x] Error types: `TransformError` enum (`FieldNotFound`, `TypeMismatch`, `ParseError`, `EnrichmentError`, `ProcessorError`, `Json`, `Io`), `with_processor()`, `Result<T>` alias
- [x] Transform trait + chain: `TransformResult` (Continue/Drop), `Transform` trait (object-safe), `TransformChain`
- [x] Prelude module: re-exports Event, Transform, TransformResult, TransformChain, TransformError, Result, json!, Value, chrono types
- [x] 32 tests passing (`cargo test -p dfe-runtime`)

### 1.3 dfe-parse: Layer 1 Parsers — COMPLETE

- [x] Parser trait + error types: `ParseResult<'a, T>`, `ParseError` enum, winnow-style convention
- [x] IP parsers: `parse_ipv4`, `parse_ipv6`, `parse_ip`, `parse_ip_or_host`, `parse_hostname` — octet validation, `::` shorthand, mixed v4/v6, RFC 1123 hostnames
- [x] Numeric parsers: `parse_int` (i64), `parse_nonneg_int` (u64), `parse_pos_int`, `parse_port` (u16), `parse_number` (f64 with exponent), `take_int` (zero-copy)
- [x] Timestamp parsers: `parse_iso8601` (fractional seconds, timezone), `parse_syslog_timestamp` (month lookup), `parse_year`, `parse_monthnum`, `parse_monthday`, `parse_time`, `parse_iso8601_tz`
- [x] String parsers: `take_word`, `take_notspace`, `take_greedy`, `take_until_byte`, `take_until_str`, `take_quoted` (escape handling, memchr), `expect_space`, `skip_space`, `expect_byte`, `expect_str`, `parse_loglevel`
- [x] Network parsers: `parse_mac` (colon/hyphen/cisco dot), `parse_uri` (full decomposition), `parse_uri_scheme`, `parse_email` (local@domain), `parse_uuid` (8-4-4-4-12)
- [x] Criterion benchmark harness: all parser families covered
- [x] 105 tests passing (`cargo test -p dfe-parse`), clippy clean

---

## Phase 2: Codegen Core

> **Parallelism:** 2.1 (Layer 2-3) and 2.2 (codegen fork) can run in parallel. 2.1 depends on 1.3. 2.2 depends on 1.2.

### 2.1 dfe-parse: Layer 2-3 (`crates/dfe-parse/`)

> **Depends on:** 1.3.1 (parser trait), at least 1.3.2-1.3.3 (IP + numeric parsers)

#### 2.1.1 Composite parser builder — COMPLETE

- [x] `CompositeParser` builder with `Literal`, `Capture`, `SkipSpace` steps
- [x] `parse()` and `parse_with_remainder()` — zero-copy field extraction
- [x] 8 unit tests: 2-field, connection pattern, skip_space, word pairs, literal mismatch, remainder, empty, hostname

#### 2.1.2 Grok pattern analyser

- [ ] `GrokAnalyser` — classify grok pattern into Layer 1 (all replaceable), Layer 2 (composite), Layer 3 (DFA)
- [ ] Grok alias expansion (`%{IP}` → regex → map to Layer 1 parser)
- [ ] Output: `AnalysisResult` with recommended layer + parser chain
- [ ] Unit tests: classify ~10 real grok patterns from Beats pipelines

> **Done when:** Analyser correctly classifies real grok patterns, `cargo test -p dfe-parse` passes.

#### 2.1.3 DFA fallback — COMPLETE

- [x] `DfaParser` wrapping `regex_automata::meta::Regex` with named capture groups
- [x] `parse()`, `parse_with_remainder()`, `is_match()`, `group_names()`
- [x] 7 unit tests: named capture, remainder, no match, is_match, group names, syslog pattern, optional groups

### 2.2 dfe-codegen: Fork + Adapt (`crates/dfe-codegen/`)

> **Depends on:** 1.2 (dfe-runtime Event + Transform trait must be defined)

#### 2.2.1 Import ANTLR4 Painless parser — COMPLETE

- [x] ANTLR4 parser files (~18k lines) in `src/painless/parser/`
- [x] Script parsing module in `src/painless/script.rs`
- [x] Transpiler disabled (VRL → Rust rewrite pending)
- [x] `cargo check -p dfe-codegen` passes, clippy clean

#### 2.2.2 Import pipeline + processor structs — COMPLETE

- [x] Pipeline structs, AST, validation, YAML deserialisation in `src/pipeline/`
- [x] 27 processor configs in `src/pipeline/processors/`
- [x] VRL Transpile trait replaced with Codegen trait
- [x] `serde_yaml` → `serde_yaml_ng`, all VRL deps stripped
- [x] 116 tests passing, clippy clean

#### 2.2.3 Adapt Painless transpiler for Rust output — IN PROGRESS

**Phase A: Foundation — COMPLETE**

- [x] IR types (`ir.rs`): 25 Expr variants, 9 Stmt variants, PathSegment, BinOp, UnaryOp, CompoundOp
- [x] Emitter (`emitter.rs`): IR → Rust source string for all IR variants (6 tests)
- [x] Params module (`params.rs`): YAML params → inlined Rust literals (5 tests)
- [x] Runtime helpers (`painless_helpers.rs`): painless_truthy, add/sub/mul/div/mod, to_i64/f64/string, eq/cmp, drop_empty, keys_to_snake_case (11 tests)
- [x] Visitor (`visitor.rs`): ANTLR4 parse tree → IR for all node types (18 tests incl. 2 e2e)
- [x] Wired into `script.rs` with `Script::transpile()` public entry point

**Phase B: Collections + iteration — PENDING**

- [ ] HashMap/ArrayList method calls in emitter
- [ ] for loops, entrySet/keySet iteration patterns
- [ ] removeIf with lambda
- [ ] instanceof type checks in iteration context

**Phase C: Functions + complex logic — PENDING**

- [ ] Local function definitions and recursion
- [ ] Regex find/match operations
- [ ] Bitwise operations in emitter

**Phase D: Integration — PENDING**

- [ ] Modify `emit_script()` in `processor.rs` to call transpiler
- [ ] Fallback to `painless_exec` no-op for scripts that fail to transpile
- [ ] Regenerate all transforms
- [ ] Run integration tests

> **Done when:** Painless transpiler emits valid Rust for all 43 scripts, or gracefully falls back.

#### 2.2.4 Rust code generation backend — COMPLETE

- [x] `PipelineCodegen` struct in `src/codegen/emit.rs` for .rs file generation
- [x] Output format: `impl Transform for <Name> { fn transform(&self, event: &mut Event) -> Result<TransformResult> }`
- [x] Processor dispatch: `emit_processor()` in `src/codegen/processor.rs` — all 27 processors
- [x] `ignore_failure`: wrap in closure with `let _ =`
- [x] `ignore_missing`: wrap in `if event.has()` check
- [x] Template string interpolation: `{{{field}}}` → `format!()` with `event.get()`
- [x] Generate `use dfe_runtime::prelude::*;` headers
- [x] 24 unit tests, generated code validates via `syn::parse_file`

#### 2.2.5 CLI tool (clap) — COMPLETE

- [x] `main.rs` with clap: `dfe-codegen generate --pipeline <path> --output <dir>`
- [x] Accept single YAML or directory of YAMLs
- [x] `--dry-run` and `--verbose` flags
- [x] `module_name_from_path()` derives module names from filenames
- [x] 3 unit tests for module name derivation

#### 2.2.6 End-to-end codegen integration test — COMPLETE

- [x] 13 integration tests in `tests/codegen_integration.rs`
- [x] Parse YAML → generate Rust → validate syntax with `syn::parse_file()`
- [x] Covers: set, remove, rename, convert (int/string/float), lowercase, uppercase, trim, split, append, drop, template interpolation, copy_from, override:false, boolean/number values, multi-processor pipelines
- [x] 306 tests passing across workspace

---

## Phase 3: Processors

> **Parallelism:** 3.1 starts after 2.2.4. 3.2 depends on 2.1 + 2.2.4. 3.3 depends on 3.4 + 2.2.3. 3.4 can run in parallel with 3.1/3.2.

### 3.1 Simple Processors (~11, codegen only) — COMPLETE

> Implemented in 2.2.4 — all 11 simple processors have codegen emitters with unit tests.

- [x] **set** — `event.set()` with conditional, value interpolation `{{{field}}}`
- [x] **append** — `event.append()` for array fields
- [x] **remove** — `event.remove()` for one or more fields
- [x] **rename** — `event.rename()` for field pairs
- [x] **uppercase** — `get_str()` → `to_uppercase()` → `set()`
- [x] **lowercase** — `get_str()` → `to_lowercase()` → `set()`
- [x] **trim** — `get_str()` → `trim()` → `set()`
- [x] **convert** — type coercion: string↔int, string↔float, int↔string
- [x] **drop** — `return Ok(TransformResult::Drop)` with optional condition
- [x] **split** — `split(separator)` → array → `set()`
- [x] **uri_parts** — parse URI into scheme, host, port, path, query, fragment

### 3.2 Medium Processors (~8, dfe-parse integration) — COMPLETE

> All 8 medium processors have codegen emitters. Grok uses regex fallback (pending 2.1.2 grok analyser for L1/L2/L3 optimisation). Date handles ISO8601, UNIX, UNIX_MS natively.

- [x] **gsub** — `regex::Regex::replace_all` with target_field support
- [x] **json** — `serde_json::from_str` with target_field support
- [x] **csv** — `csv::ReaderBuilder` with separator, quote, target_fields
- [x] **kv** — key-value split with field_split, value_split, target_field, trim
- [x] **dissect** — pattern parser with `%{field}` extraction, right-padding (`->`)
- [x] **grok** — regex fallback with `grok_to_regex()` placeholder (TODO: L1/L2/L3 via 2.1.2)
- [x] **foreach** — array iteration with inner processor codegen
- [x] **date** — ISO8601 (chrono), UNIX, UNIX_MS with target_field (default `@timestamp`)
- [x] CSV/Foreach validators relaxed to support common fields (separator, ignore_missing, ignore_failure)
- [x] 28 integration tests, 321 workspace tests passing

### 3.3 Complex Processors (~8, enrichment + runtime) — COMPLETE

> All 8 complex processors have codegen emitters. Enrichment processors (geoip, user_agent, community_id) emit calls to runtime functions (implemented in Phase 3.4). Script emits Painless stub (full transpiler in 2.2.3). All 27 Processor enum variants now have exhaustive codegen coverage.

- [x] **registered_domain** — public suffix list lookup via `registered_domain_lookup()`, target_field prefix support
- [x] **network_direction** — `is_internal_ip()` classification, source/dest vs internal networks → direction
- [x] **fingerprint** — SHA-256 hash of field values, default target `_id`, ignore_missing support
- [x] **pipeline** (nested) — inline expansion of inner pipeline processors via `parse_with_context()`
- [x] **geoip** — `geoip_lookup()` call with City/Country/ASN database support, property filtering
- [x] **user_agent** — `parse_user_agent()` call, sets name/version/os/device under target field
- [x] **community_id** — `community_id_v1()` hash, custom source/dest field support
- [x] **script** — `painless_exec()` stub with source embedding (TODO: full transpiler 2.2.3)
- [x] 42 integration tests, 346 workspace tests passing, exhaustive match (no catch-all)

### 3.4 Enrichment Runtime (`crates/dfe-runtime/src/enrichment/`) — COMPLETE

> All 3 enrichment modules implemented with full test coverage. 53 runtime tests passing.

#### 3.4.1 GeoIP (`crates/dfe-runtime/src/enrichment/geoip.rs`) — COMPLETE

- [x] `GeoIpEnrichment` wrapping `maxminddb::Reader<Vec<u8>>` with auto db-type detection
- [x] `GeoIpDbType` enum: City, Country, ASN
- [x] `lookup(ip) -> Result<GeoIpResult>` — IP → HashMap<String, Value>
- [x] `enrich(event, ip_field, target_prefix, properties, ignore_missing) -> Result<()>`
- [x] Field extraction: city, country, continent, region, timezone, location, ASN
- [x] Updated for maxminddb 0.27 API (LookupResult → decode())
- [x] 3 unit tests (city, country, ASN field extraction)

> **Note:** GeoIP will be redesigned for multi-provider + auto-download — see Phase 5.

#### 3.4.2 User Agent (`crates/dfe-runtime/src/enrichment/user_agent.rs`) — COMPLETE

- [x] `parse(ua_string) -> UserAgentResult` — regex-based browser, OS, device detection
- [x] Browser detection: Chrome, Firefox, Safari, Edge, Opera, IE (ordered by specificity)
- [x] OS detection: Windows (NT version map), Mac OS X, Android, iOS, Linux
- [x] Device detection: phone, tablet, pc, bot
- [x] `enrich(event, ua_field, target_prefix, ignore_missing) -> Result<()>`
- [x] 9 unit tests (Chrome/macOS, Firefox/Windows, Safari/iPhone, Edge, Android, empty, enrich, ignore_missing, error)

#### 3.4.3 Community ID (`crates/dfe-runtime/src/enrichment/community_id.rs`) — COMPLETE

- [x] `community_id_v1(src_ip, dst_ip, src_port, dst_port, transport, seed) -> Result<String>`
- [x] SHA-1 hash with base64 encoding, "1:" prefix, IP/port ordering per spec
- [x] `protocol_number()` IANA mapping (TCP, UDP, ICMP, SCTP, etc.)
- [x] `CommunityIdConfig` struct + `enrich(event, config) -> Result<()>`
- [x] 7 unit tests including known test vector validation

---

## Phase 4: Integration + Testing

> **Parallelism:** 4.1 first. 4.2 depends on Phase 3. 4.3 depends on 4.2. 4.4 can run with 4.3.

### 4.1 Test Framework (`crates/dfe-transforms/`) — COMPLETE

> **Depends on:** 1.2 (Event type)

#### 4.1.1 Test harness — COMPLETE

- [x] `run_transform_test(input_path, expected_path, transform_fn)` — load, transform, compare
- [x] `load_test_events(path) -> Vec<Event>` — JSON lines, JSON array, or raw log
- [x] `load_expected_outputs(path) -> Vec<Value>`
- [x] Message wrapping (auto-detect `message` field presence)
- [x] Config field loading from pipeline YAML for transforms that need it

#### 4.1.2 JSON comparison utilities — COMPLETE

- [x] Deep JSON comparison with diff output
- [x] Match modes: `Exact`, `Semantic` (ignore non-deterministic), `Subset` (expected ⊂ actual)
- [x] `JsonDiff` for readable diff output (path + expected vs actual)
- [x] Auto-format detection for test fixture files

#### 4.1.3 Test fixture import — COMPLETE

- [x] Test data in `testdata/integrations/` (1,691 fixture files from elastic_to_vrl)
- [x] Verify fixtures accessible from `#[test]` functions
- [x] CrowdStrike, Azure, Okta test data loaded and running

### 4.2 Generate + Validate All Transforms — IN PROGRESS

> **Depends on:** Phase 3, 2.2.5 (CLI)

- [x] Collect all pipeline YAMLs (Beats + Agent)
- [x] Run `dfe-codegen generate` on all pipelines
- [x] Fix codegen issues iteratively (conditional transpiler covers 94% of 1,180 conditionals)
- [x] Generated code compiles: `cargo check -p dfe-transforms` passes
- [ ] Hand-tune remaining transforms for match rate improvement
- [ ] Improve sub-pipeline append to handle array values correctly

> **Done when:** All pipelines produce compilable Rust, `cargo check -p dfe-transforms` passes, match rates >90%.

### 4.3 1:1 Beats Test Data Validation — IN PROGRESS

> **Depends on:** 4.1, 4.2
> **Parallelism:** Each source type independent.

**Current match rate: 44% (30/68 events)** `[IN PROGRESS]`
  - CrowdStrike non-CSPM: 30/32 (94%) — sentinel values + dedup fixed
  - Next: Fix event-stream epoch issue (2 events), SensorGroupingTags parsing, then start Okta
  - Blockers: GeoIP DB mismatch (DB-IP vs MaxMind — skipped in Semantic mode)

- [ ] **Filebeat** — ~60 modules
- [ ] **Winlogbeat** — ~5 modules
- [ ] **Auditbeat** — ~3 modules
- [ ] **Metricbeat** — ~30 modules
- [ ] **Heartbeat** — ~3 modules
- [ ] **Packetbeat** — ~10 modules
- [ ] **Elastic Agent:**
  - [ ] O365 (36 test files)
  - [ ] Cisco (73 test files)
  - [ ] CrowdStrike (42 test files) — 30/34 events matching (88%)
    - falcon-audit: 13/13 (100%)
    - falcon-events: 3/3 (100%)
    - falcon-sample: 7/7 (100%)
    - event-stream: 7/9 (78%) — 2 remaining: epoch + SensorGroupingTags
    - falcon-tags/tags-list: 0/2 (CSPM events — need sub-pipeline routing)
  - [ ] Azure (48 test files) — 0%
  - [ ] Okta (2 test files) — 0%
  - [ ] Panw (14 test files)
  - [ ] Fortinet (27 test files)

> **Done when:** `cargo test -p dfe-transforms` passes for all source types.

### 4.4 Performance Validation

> **Depends on:** 4.2
> **Parallelism:** Can run with 4.3.

- [ ] Criterion benchmarks: per-parser (dfe-parse vs regex baseline)
- [ ] Criterion benchmarks: per-transform (Rust vs VRL equivalent)
- [ ] End-to-end benchmark: JSON parse → transform chain → serialise
- [ ] Document results in `BENCHMARKS.md`

> **Done when:** `BENCHMARKS.md` exists with measured speedups for 5+ parsers and 3+ transforms.

---

## Phase 5: Packaging + Deployment

### 5.1 Runtime Binary + hyperi-rustlib Integration

> **hyperi-rustlib integration plan.** Features already added: `logger`, `memory` (v1.16.5).
> DfeSource and MemoryGuard types already re-exported from dfe-runtime.
> Phase 5 adds the runtime binary features.

**Runtime binary (`dfe-transform-elastic`):**

- [ ] Runtime CLI binary using rustlib `cli` feature (DfeApp trait, CommonArgs)
- [ ] Wire in codegen CLI as subcommand or separate binary
- [ ] Graceful shutdown (CancellationToken, SIGTERM/SIGINT)

**Config cascade (rustlib `config` + `config-reload`):**

- [ ] Add `config` and `config-reload` features to hyperi-rustlib dependency
- [ ] `settings.yaml` for Kafka brokers, batch size, enrichment paths, pipeline config
- [ ] Env var overrides: `DFE_TRANSFORM_ELASTIC_*` prefix
- [ ] Hot-reload: `retry.*`, `scaling.*`, `source.batch_size` (takes effect on next batch)
- [ ] Restart-only: `source.transport.*`, `sink.transport.*`, `enrichment.*`, `http.*`, `pipeline_name`

**Kafka transport (rustlib `transport-kafka`):**

- [ ] Add `transport-kafka` feature to hyperi-rustlib dependency
- [ ] Kafka consumer (rdkafka) reading from `{source}_land` topics via DfeSource
- [ ] Kafka producer writing to `{source}_load` topics via DfeSource
- [ ] Consumer group: `dfe-transform-elastic-{source}` via `DfeSource::consumer_group()`
- [ ] DLQ support via rustlib `dlq-kafka` feature (if needed)

**Memory backpressure (rustlib `memory` — already added):**

- [ ] Wire MemoryGuard into Kafka consumer loop (Pattern B — pause consumer)
- [ ] `MemoryGuardConfig::from_env_raw("DFE_TRANSFORM_ELASTIC")`
- [ ] Readiness probe: `!memory_guard.under_pressure() && sink.is_healthy()`
- [ ] Metrics: `memory_guard.current_bytes()`, `memory_guard.limit_bytes()`

**Observability (rustlib `metrics` + `http-server`):**

- [ ] Add `metrics` and `http-server` features to hyperi-rustlib dependency
- [ ] Prometheus `/metrics` endpoint: transform latency, throughput, error rate
- [ ] Health endpoints: `/healthz` (liveness), `/readyz` (readiness)
- [ ] Per-transform counters: events processed, matched, dropped, errors

**Deployment (rustlib `deployment` + `scaling`):**

- [ ] Add `deployment` and `scaling` features to hyperi-rustlib dependency
- [ ] `generate_chart()` and `generate_dockerfile()` from contract
- [ ] KEDA autoscaling signals via `ScalingPressure`

**Rustlib feature summary for Phase 5 runtime binary:**

```toml
hyperi-rustlib = { version = ">=1.16.5", features = [
    "logger",           # Already added — structured logging
    "memory",           # Already added — MemoryGuard, cgroup-aware
    "config",           # 8-layer cascade (CLI > env > .env > YAML > defaults)
    "config-reload",    # SharedConfig + ConfigWatcher for hot-reload
    "cli",              # DfeApp trait, CommonArgs (clap integration)
    "metrics",          # Prometheus /metrics export
    "http-server",      # /healthz, /readyz endpoints
    "transport-kafka",  # rdkafka consumer/producer
    "scaling",          # KEDA backpressure signals
    "deployment",       # Dockerfile + Helm chart generation
] }
```

> **Done when:** `dfe-transform-elastic --help` works, config validation passes, Kafka round-trip works locally.

### 5.2 GeoIP + Enrichment — Re-use dfe-loader Implementation

> **Decision:** dfe-loader is the primary owner of GeoIP/reputation enrichment. dfe-transform-elastic
> should re-use dfe-loader's implementation (auto-download, LRU cache, private IP fast-path, multi-db).
> When dfe-transform-elastic runs standalone (without dfe-loader), it needs to perform enrichment itself.

**Architecture:**
- dfe-loader already has: `src/enrich/geoip.rs` (570 lines), LRU cache (100k), private IP fast-path, City+ASN databases
- dfe-loader also has: `src/enrich/reputation.rs` (775 lines), `src/enrich/risk.rs` (617 lines)
- GeoIP is used in ~81% of Elastic pipelines (21/26 pipeline files, 81 lookup calls)
- Community ID in ~52% (network-flow pipelines), User Agent in ~67%

**Tasks:**
- [ ] Extract shared enrichment code from dfe-loader into a shared crate or copy+adapt
- [ ] Re-use dfe-loader's auto-download + preloading pattern for MMDB databases
- [ ] Re-use dfe-loader's LRU cache (100k entries) and private IP fast-path
- [ ] Support standalone mode: when dfe-transform-elastic runs without dfe-loader, enrichment runs locally
- [ ] Support pass-through mode: when dfe-loader handles enrichment, skip it in transforms
- [ ] Config: enrichment mode (standalone vs pass-through), database paths, download settings
- [ ] Align both projects on same maxminddb version and GeoIP field schema

> **Done when:** GeoIP enrichment works in standalone mode using dfe-loader's patterns, pass-through mode skips enrichment cleanly.

### 5.3 Docker + Helm Artefacts

- [ ] `Dockerfile` — multi-stage build (builder + runtime), replace existing hand-written version
- [ ] `chart/` — Helm chart directory (Chart.yaml, values.yaml, templates/)
- [ ] `docker-compose.yaml` — Compose fragment for local dev
- [ ] Health endpoints (`/health/live`, `/health/ready`, `/health/startup`)
- [ ] Debug utilities (curl, netcat) in runtime image

> **Done when:** `docker build .` succeeds, `helm template chart/` renders, `docker-compose up` starts locally.

---

## Deferred

### PINNED: dfe-parsers — Standalone Parser Crate

> **Do not remove.** This is a pinned architectural goal.

- [ ] Extract Painless-converted parsers into standalone `dfe-parsers` crate
- [ ] Any DFE Rust project can depend on dfe-parsers for parsing
- [ ] Painless is one "tag" (source) for parsers — others may come from bespoke or other sources
- [ ] dfe-transform-elastic layers Beats/Agent envelope handling over dfe-parsers
- [ ] Enables raw message feed mode: same parsers work for syslog/event sources without Beats

> See [docs/DESIGN.md](docs/DESIGN.md) "Future: dfe-parsers" section for full architecture.

### PINNED: Raw Message Feed Mode

> **Do not remove.** This is a pinned feature scope item.

- [ ] Startup option to run in raw message feed mode
- [ ] Reuses same Painless-converted parsers against raw syslog/event messages
- [ ] Enables same parsing without Beats/Agent as the data source
- [ ] Depends on dfe-parsers crate extraction (above)

### PINNED: Rustlib Capability Audit (Before Phase 5)

> **Do not remove.** Review all bespoke code against rustlib features before Phase 5.
> Ensure we don't have bespoke implementations for functions rustlib provides.

- [ ] Audit dfe-runtime for bespoke config, transport, metrics, health code
- [ ] Map Kafka transport to rustlib `transport-kafka` (rdkafka wrapper)
- [ ] Map config loading to rustlib `config` (8-layer cascade)
- [ ] Map health endpoints to rustlib `http-server` (/healthz, /readyz)
- [ ] Map metrics to rustlib `metrics` (Prometheus /metrics)
- [ ] Map CLI framework to rustlib `cli` (DfeApp trait)
- [ ] Map resilience patterns to rustlib `resilience` (circuit breaker, retry)
- [ ] Map scaling signals to rustlib `scaling` (KEDA backpressure)
- [ ] Ensure no bespoke implementations exist for rustlib-provided functions

### Other Deferred

- [ ] WASM extensibility (user-defined transforms)

---

## Completed

### 2026-03-03: Project Initialisation + Phase 1 + Phase 2

- [x] Research: parsing libraries, architecture, effort estimation (RESEARCH.md)
- [x] Git repo, ai/ci submodules attached
- [x] Cargo workspace: dfe-parse, dfe-runtime, dfe-codegen, dfe-transforms
- [x] All config files matching dfe-loader conventions
- [x] STATE.md, TODO.md, RESEARCH.md
- [x] Workspace compiles clean
- [x] docs/DESIGN.md — architecture, Mermaid diagrams, API contracts
- [x] dfe-runtime core: Event struct, typed getters/setters, Transform trait, TransformChain, error types (32 tests)
- [x] dfe-parse Layer 1: IP, numeric, timestamp, string, network parsers + benchmark harness (105 tests)
- [x] dfe-parse Layer 2: Composite parser builder (8 tests)
- [x] dfe-parse Layer 3: DFA fallback parser (7 tests)
- [x] dfe-codegen: Fork elastic_to_vrl, strip VRL deps, import ANTLR4 Painless parser + 27 processor configs (116 tests)
- [x] dfe-codegen: Rust code generation backend for 11 simple processors (24 tests)
- [x] dfe-codegen: CLI with clap — generate, dry-run, verbose (3 tests)
- [x] dfe-codegen: End-to-end integration tests with syn validation (13 tests)
- [x] dfe-codegen: Medium processor codegen — gsub, json, csv, kv, dissect, grok, foreach, date (28 integration tests)
- [x] dfe-codegen: Complex processor codegen — registered_domain, network_direction, fingerprint, pipeline, geoip, user_agent, community_id, script (42 integration tests, 346 workspace tests)
- [x] All 27 processors have exhaustive codegen coverage (no catch-all match arm needed)
- [x] dfe-runtime enrichment: Community ID v1 (SHA-1, base64, 7 tests), GeoIP (maxminddb 0.27, 3 tests), User Agent (regex, 9 tests) — 53 runtime tests, 367 workspace tests
- [x] Painless transpiler Phase A: IR types, emitter, params, visitor, runtime helpers, Script::transpile() entry point — 40 new tests, 443 workspace tests passing

### 2026-03-19: Integration Testing Infrastructure + First Match Rates

- [x] Test harness: message wrapping, config field loading, auto-format detection
- [x] JSON comparison: deep diff with Exact/Semantic/Subset match modes
- [x] Conditional transpiler (`codegen/condition.rs`) — covers 94% of 1,180 conditionals
- [x] Global GeoIP enricher (`enrichment/geoip_global.rs`) — auto-detect MMDB, private IP fast-path
- [x] Painless common patterns (`painless_common.rs`) — drop_empty, keys_to_snake_case, process fields, epoch conversion
- [x] Event::get_string() and get_as_string() — borrow-safe field access
- [x] Epoch precision auto-detect (ported from dfe-loader)
- [x] Elastic-standard timestamp format (%Y-%m-%dT%H:%M:%S%.3fZ)
- [x] CrowdStrike integration tests: 13/68 events matching (19%, from 0%)
- [x] 476 workspace tests passing, 0 failures

---

## Notes

- elastic_to_vrl source: `/projects/elastic_to_vrl/`
- VRL templates being replaced: `/projects/dfe-vector-templates/src/core_templates/`
- Rust 1.94 / Edition 2024 (`gen` is reserved — use `cg` for codegen vars)
- GeoIP test DBs in gitignored `testdata/geoip/` (DB-IP Lite, CC BY 4.0)
- Painless parser kept as callable component within dfe-codegen per user decision
- Task IDs (e.g., 1.2.3, 2.2.4) used for dependency references across phases
- "Parallelism" notes indicate which tasks can run concurrently within a phase
- Every task's "Done when" criterion includes `cargo check` or `cargo test` for the affected crate

---

### 2026-03-20: hyperi-rustlib Integration + CI Migration

- [x] Migrate to hyperi-ci reusable workflows (remove ci/ submodule, add Makefile)
- [x] Upgrade Rust 1.83→1.94 / Edition 2021→2024
- [x] Code review remediation: rustfmt.toml, deny.toml, lint attrs, regex precompile
- [x] Integrate hyperi-rustlib v1.16.5: DfeSource, MemoryGuard, logger
- [x] Re-export DfeSource + ServiceRole from dfe-runtime for topic naming
- [x] Re-export MemoryGuard + MemoryGuardConfig + MemoryPressure for backpressure
- [x] Replace bespoke tracing setup with rustlib logger::setup_default()
- [x] Phase 5 WBS updated with explicit rustlib feature integration plan

---

### 2026-03-20: Rustlib Integration + Codegen Improvements + Test Infra

- [x] Integrate hyperi-rustlib v1.16.5: DfeSource, MemoryGuard, logger
- [x] Phase 5 WBS updated with explicit rustlib feature integration plan
- [x] Codegen: add try_string_length to condition transpiler (epoch ms vs seconds)
- [x] Codegen: add prefix negation for !["a","b"].contains() pattern
- [x] Regenerate CrowdStrike transforms — match rate 38% → 76% (26/34)
- [x] CrowdStrike falcon-audit: 100% (13/13), falcon-events: 100% (3/3)
- [x] Add painless_drop_empty to CrowdStrike transform (empty string cleanup)
- [x] Skip GeoIP fields in Semantic comparison (DB-IP vs MaxMind mismatch)
- [x] Add Step 4a (codegen feedback) to Development Cycle
- [x] Test infrastructure scaffolding: .env, .env.example, TestMode, KafkaTestConfig

---

**Last Updated:** 2026-03-20
