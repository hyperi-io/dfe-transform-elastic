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

#### 2.2.3 Adapt Painless transpiler for Rust output

- [ ] Modify transpiler to emit Rust expressions instead of VRL
- [ ] Handle type coercion (VRL dynamic → `serde_json::Value` operations)
- [ ] Map Painless built-in functions to dfe-runtime equivalents
- [ ] Unit tests: transpile 5+ Painless snippets, verify output compiles

> **Done when:** Painless transpiler emits valid Rust for basic scripts.

#### 2.2.4 Rust code generation backend — COMPLETE

- [x] `PipelineCodegen` struct in `src/codegen/emit.rs` for .rs file generation
- [x] Output format: `impl Transform for <Name> { fn transform(&self, event: &mut Event) -> Result<TransformResult> }`
- [x] Processor dispatch: `emit_processor()` in `src/codegen/processor.rs` — 11 simple processors
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

### 3.1 Simple Processors (~11, codegen only)

> **Depends on:** 2.2.4 (Rust codegen backend)
> **Parallelism:** All independent, any order.

- [ ] **set** — `event.set()` with conditional, value interpolation `{{{field}}}`
- [ ] **append** — `event.append()` for array fields
- [ ] **remove** — `event.remove()` for one or more fields
- [ ] **rename** — `event.rename()` for field pairs
- [ ] **uppercase** — `get_str()` → `to_uppercase()` → `set()`
- [ ] **lowercase** — `get_str()` → `to_lowercase()` → `set()`
- [ ] **trim** — `get_str()` → `trim()` → `set()`
- [ ] **convert** — type coercion: string↔int, string↔float, int↔string
- [ ] **drop** — `return Ok(TransformResult::Drop)` with optional condition
- [ ] **split** — `split(separator)` → array → `set()`
- [ ] **uri_parts** — parse URI into scheme, host, port, path, query, fragment

> **Done when (each):** Codegen emits correct Rust, generated code compiles, unit test passes.

### 3.2 Medium Processors (~8, dfe-parse integration)

> **Depends on:** 2.1 (composite parsers), 2.1.3 (DFA), 2.2.4 (codegen backend)

- [ ] **grok** — emit L1/L2/L3 parser calls via grok analysis; `pattern_definitions`, multiple patterns
  - *Depends on: 2.1.2 (grok analyser)*
- [ ] **dissect** — emit tokenizer-based split parsers (`%{field}`, `%{field->}`, `%{+field}`)
- [ ] **json** — nested JSON parse via `simd_json::from_str` / `serde_json::from_str`
- [ ] **kv** — key-value parse with configurable field_split, value_split, include/exclude
- [ ] **csv** — CSV field parse with configurable separator, quote, target fields
- [ ] **foreach** — iterate over array fields, apply sub-processor to each element
- [ ] **date** — timestamp parse using dfe-parse; format strings, timezone, target field
  - *Depends on: 1.3.4 (timestamp parsers)*
- [ ] **gsub** — regex substitution via `regex::Regex::replace_all`

> **Done when (each):** Codegen emits correct Rust with dfe-parse calls, generated code compiles, unit test passes.

### 3.3 Complex Processors (~8, enrichment + runtime)

> **Depends on:** 3.4 (enrichment runtime), 2.2.3 (Painless transpiler)

- [ ] **script** (Painless → Rust) — transpiler output, complex control flow
  - *Depends on: 2.2.3*
- [ ] **geoip** — emit `enrichment::geoip::enrich()` calls
  - *Depends on: 3.4.1*
- [ ] **user_agent** — emit `enrichment::user_agent::enrich()` calls
  - *Depends on: 3.4.2*
- [ ] **community_id** — emit `enrichment::community_id::enrich()` calls
  - *Depends on: 3.4.3*
- [ ] **registered_domain** — public suffix list lookup → registered_domain + subdomain
- [ ] **network_direction** — internal/external IP classification via CIDR config
- [ ] **fingerprint** — hash fingerprinting (SHA-256, SHA-1, MD5, MurmurHash3)
- [ ] **pipeline** (nested) — resolve pipeline reference, execute as sub-chain

> **Done when (each):** Codegen emits correct Rust, generated code compiles + links to enrichment, unit test passes.

### 3.4 Enrichment Runtime (`crates/dfe-runtime/src/enrichment/`)

> **Depends on:** 1.2 (Event type)
> **Parallelism:** 3.4.1, 3.4.2, 3.4.3 are independent.

#### 3.4.1 GeoIP (`crates/dfe-runtime/src/enrichment/geoip.rs`)

- [ ] `GeoIpEnrichment` wrapping `maxminddb::Reader` with mmap
- [ ] `lookup(ip) -> Result<GeoIpResult>` — IP → city/country/location
- [ ] `enrich(event, ip_field, target_prefix) -> Result<()>`
- [ ] LRU cache for repeated IPs (configurable size)
- [ ] Unit tests with test MMDB

> **Done when:** GeoIP lookup works with mmap, LRU cache functional, `cargo test -p dfe-runtime` passes.

#### 3.4.2 User Agent (`crates/dfe-runtime/src/enrichment/user_agent.rs`)

- [ ] `UserAgentEnrichment` — regex-based UA parsing
- [ ] `parse(ua_string) -> UserAgentResult` — name, version, os, device
- [ ] `enrich(event, ua_field, target_prefix) -> Result<()>`
- [ ] Unit tests with common UA strings

> **Done when:** UA parsing works for top 10 UA strings, `cargo test -p dfe-runtime` passes.

#### 3.4.3 Community ID (`crates/dfe-runtime/src/enrichment/community_id.rs`)

- [ ] `community_id_v1(src_ip, dst_ip, src_port, dst_port, protocol) -> String`
- [ ] `enrich(event) -> Result<()>` — extract fields, compute hash, set `network.community_id`
- [ ] Unit tests with known test vectors from spec

> **Done when:** Hash matches reference implementation for all test vectors, `cargo test -p dfe-runtime` passes.

---

## Phase 4: Integration + Testing

> **Parallelism:** 4.1 first. 4.2 depends on Phase 3. 4.3 depends on 4.2. 4.4 can run with 4.3.

### 4.1 Test Framework (`crates/dfe-transforms/`)

> **Depends on:** 1.2 (Event type)

#### 4.1.1 Test harness

- [ ] `run_transform_test(input_path, expected_path, transform_fn)` — load, transform, compare
- [ ] `load_test_events(path) -> Vec<Event>` — JSON lines, JSON array, or raw log
- [ ] `load_expected_outputs(path) -> Vec<Value>`

> **Done when:** Harness works with at least one hand-crafted fixture.

#### 4.1.2 JSON comparison utilities

- [ ] `assert_json_eq!` macro — deep JSON comparison with diff output
- [ ] Match modes: `Exact`, `Semantic` (ignore non-deterministic), `Subset` (expected ⊂ actual)
- [ ] `JsonDiff` for readable diff output (path + expected vs actual)

> **Done when:** Clear diff output for all three match modes, `cargo test` passes.

#### 4.1.3 Test fixture import

- [ ] Git submodules: `external/beats`, `external/elastic-agent`
- [ ] Import 242 existing test files from elastic_to_vrl
- [ ] Verify fixtures accessible from `#[test]` functions

> **Done when:** Fixtures from at least one Beats module and one Agent integration are loadable.

### 4.2 Generate + Validate All Transforms

> **Depends on:** Phase 3, 2.2.5 (CLI)

- [ ] Collect all pipeline YAMLs (Beats + Agent)
- [ ] Run `dfe-codegen generate` on all pipelines
- [ ] Fix codegen issues iteratively
- [ ] Generated code compiles: `cargo check -p dfe-transforms` passes
- [ ] Hand-tune ~5% that doesn't auto-generate (`// HAND-TUNED:` comments)

> **Done when:** All pipelines produce compilable Rust, `cargo check -p dfe-transforms` passes.

### 4.3 1:1 Beats Test Data Validation

> **Depends on:** 4.1, 4.2
> **Parallelism:** Each source type independent.

- [ ] **Filebeat** — ~60 modules
- [ ] **Winlogbeat** — ~5 modules
- [ ] **Auditbeat** — ~3 modules
- [ ] **Metricbeat** — ~30 modules
- [ ] **Heartbeat** — ~3 modules
- [ ] **Packetbeat** — ~10 modules
- [ ] **Elastic Agent:**
  - [ ] O365 (36 test files)
  - [ ] Cisco (73 test files)
  - [ ] CrowdStrike (42 test files)
  - [ ] Azure (48 test files)
  - [ ] Okta (2 test files)
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

## Deferred

- [ ] Kafka integration (rdkafka consumer for end-to-end testing)
- [ ] Prometheus metrics (per-transform latency, throughput)
- [ ] Container image + Helm chart
- [ ] hyperi-rustlib integration (transport, config, DLQ)
- [ ] Hot-reload (config-driven transform pipeline changes)
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

---

## Notes

- elastic_to_vrl source: `/projects/elastic_to_vrl/`
- VRL templates being replaced: `/projects/dfe-vector-templates/src/core_templates/`
- simd-json pinned to `<0.15` for Rust 1.83 compat (bump when toolchain updates)
- Painless parser kept as callable component within dfe-codegen per user decision
- Task IDs (e.g., 1.2.3, 2.2.4) used for dependency references across phases
- "Parallelism" notes indicate which tasks can run concurrently within a phase
- Every task's "Done when" criterion includes `cargo check` or `cargo test` for the affected crate

---

**Last Updated:** 2026-03-03
