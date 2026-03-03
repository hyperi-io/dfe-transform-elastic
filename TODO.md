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

## Phase 5: Packaging + Deployment

### 5.1 CLI Module

- [ ] Runtime CLI binary (clap): `dfe-transform run --config <path>`
- [ ] Wire in codegen CLI as subcommand or separate binary
- [ ] Config loading (YAML): pipeline paths, enrichment database paths, Kafka settings
- [ ] Tracing subscriber setup (json/text, log level, env filter)
- [ ] Graceful shutdown (tokio signal handling)

> **Done when:** `dfe-transform --help` works, config validation passes.

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

- [ ] Kafka integration (rdkafka consumer for end-to-end testing)
- [ ] Prometheus metrics (per-transform latency, throughput)
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
- [x] dfe-codegen: Medium processor codegen — gsub, json, csv, kv, dissect, grok, foreach, date (28 integration tests)
- [x] dfe-codegen: Complex processor codegen — registered_domain, network_direction, fingerprint, pipeline, geoip, user_agent, community_id, script (42 integration tests, 346 workspace tests)
- [x] All 27 processors have exhaustive codegen coverage (no catch-all match arm needed)
- [x] dfe-runtime enrichment: Community ID v1 (SHA-1, base64, 7 tests), GeoIP (maxminddb 0.27, 3 tests), User Agent (regex, 9 tests) — 53 runtime tests, 367 workspace tests

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

**Last Updated:** 2026-03-04
