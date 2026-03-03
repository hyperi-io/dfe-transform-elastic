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

### 1.2 dfe-runtime Crate

> **Parallelism:** Tasks 1.2.1-1.2.4 are sequential (each builds on previous). 1.2.5 (error types) can run in parallel with 1.2.1. 1.2.6 (prelude) comes last.

#### 1.2.1 Event struct + constructor (`crates/dfe-runtime/src/event.rs`)

- [ ] `Event` struct wrapping `serde_json::Value`
- [ ] `Event::new(value: serde_json::Value)` — construct from owned Value
- [ ] `Event::from_str(json: &str)` — parse from JSON string (serde_json)
- [ ] `Event::from_bytes(buf: &mut [u8])` — parse from mutable buffer (simd-json)
- [ ] `Event::as_value(&self) -> &Value` and `Event::into_value(self) -> Value`
- [ ] Unit tests: construct from valid JSON, empty object, nested object, array root

> **Done when:** `Event` can be constructed from all three input forms, `cargo test -p dfe-runtime` passes.

#### 1.2.2 Dotted-path navigation + typed getters

- [ ] Internal `resolve_path()` — walk dotted keys (e.g., `source.ip` → nested map traversal)
- [ ] `get(path) -> Option<&Value>`, `get_str(path) -> Option<&str>`, `get_i64(path) -> Option<i64>`
- [ ] `get_f64(path) -> Option<f64>`, `get_bool(path) -> Option<bool>`
- [ ] `get_array(path) -> Option<&Vec<Value>>`, `get_object(path) -> Option<&Map<String, Value>>`
- [ ] `has(path) -> bool`
- [ ] Unit tests: nested paths (`a.b.c`), missing intermediate keys, type mismatches, root-level, empty path

> **Done when:** All typed getters work on 3+ levels of nesting, `cargo test -p dfe-runtime` passes.

#### 1.2.3 Setters (set, remove, rename)

- [ ] `set(path, value) -> Result<()>` — creates intermediate objects
- [ ] `remove(path) -> Option<Value>` — remove and return
- [ ] `rename(from, to) -> Result<()>` — remove + set at new path
- [ ] Unit tests: set creates missing parents, set overwrites, remove returns value, remove missing returns None, rename across parents

> **Done when:** set/remove/rename work with auto-created intermediate objects, `cargo test -p dfe-runtime` passes.

#### 1.2.4 Array operations + merge

- [ ] `append(path, value) -> Result<()>` — append to array (create if missing)
- [ ] `merge(other, deep) -> Result<()>` — shallow or deep merge
- [ ] Unit tests: append to existing array, append creates array, merge shallow/deep, merge conflicting types

> **Done when:** Array append and both merge modes work, `cargo test -p dfe-runtime` passes.

#### 1.2.5 Error types (`crates/dfe-runtime/src/error.rs`)

- [ ] Create `error.rs` module
- [ ] `TransformError` enum via `thiserror` — `FieldNotFound`, `TypeMismatch`, `ParseError`, `EnrichmentError`, `ProcessorError`
- [ ] `with_context(self, processor: &str)` for wrapping errors with processor name
- [ ] `Result<T>` type alias
- [ ] Unit tests: error display includes path and context, `?` propagation works

> **Done when:** All error variants exist, `?` propagation works in transform fns, `cargo test -p dfe-runtime` passes.

#### 1.2.6 Transform trait + chain (`crates/dfe-runtime/src/transform.rs`)

- [ ] `TransformResult` enum — `Continue`, `Drop`
- [ ] `Transform` trait — `name(&self) -> &str`, `transform(&self, event: &mut Event) -> Result<TransformResult>`
- [ ] `TransformChain` — sequential composition, stop on Drop or error
- [ ] Unit tests: single transform, chain of 3, Drop stops early, error propagation

> **Done when:** Transform trait is object-safe, TransformChain composes correctly, `cargo test -p dfe-runtime` passes.

#### 1.2.7 Prelude module (`crates/dfe-runtime/src/prelude.rs`)

- [ ] Re-export `Event`, `Transform`, `TransformResult`, `TransformChain`, `TransformError`, `Result`
- [ ] Re-export `serde_json::Value`, `serde_json::json!`
- [ ] Verify: `use dfe_runtime::prelude::*;` is sufficient to write a complete transform fn

> **Done when:** A test transform compiles using only `prelude::*`, `cargo test -p dfe-runtime` passes.

### 1.3 dfe-parse: Layer 1 Parsers (`crates/dfe-parse/`)

> **Parallelism:** 1.3.1 (parser trait) first. Then 1.3.2-1.3.6 can all run in parallel. 1.3.7 (benchmarks) depends on at least one parser family.

#### 1.3.1 Parser trait + error types

- [ ] `ParseResult<'a, T>` type — `Result<(&'a str, T), ParseError>` (remaining input + parsed value)
- [ ] `ParseError` enum via `thiserror` — `UnexpectedByte`, `UnexpectedEof`, `InvalidFormat`, `OutOfRange`
- [ ] Document winnow-style convention in module doc comment

> **Done when:** Types compile, convention documented, `cargo test -p dfe-parse` passes.

#### 1.3.2 IP parsers (`crates/dfe-parse/src/ip.rs`)

- [ ] `parse_ipv4(input) -> ParseResult<'_, &str>` — octet validation (0-255)
- [ ] `parse_ipv6(input) -> ParseResult<'_, &str>` — colon-group, `::` shorthand, mixed v4/v6
- [ ] `parse_ip(input) -> ParseResult<'_, &str>` — try IPv4 then IPv6
- [ ] `parse_ip_or_host(input) -> ParseResult<'_, &str>` — IP then hostname fallback
- [ ] Unit tests + proptest: edge cases (0.0.0.0, 255.255.255.255, ::1, leading zeros)

> **Done when:** All 4 parsers pass unit + property tests, `cargo test -p dfe-parse` passes.

#### 1.3.3 Numeric parsers (`crates/dfe-parse/src/numeric.rs`)

- [ ] `parse_int<T: FromStr>(input) -> ParseResult<'_, T>` — digit scan, optional sign, generic
- [ ] `parse_number(input) -> ParseResult<'_, f64>` — integer or float, sign, decimal, exponent
- [ ] `parse_hex(input) -> ParseResult<'_, u64>` — hex with optional `0x` prefix
- [ ] Unit tests + proptest: positive/negative, overflow, leading zeros, scientific notation

> **Done when:** All 3 parsers pass unit + property tests, `cargo test -p dfe-parse` passes.

#### 1.3.4 Timestamp parsers (`crates/dfe-parse/src/timestamp.rs`)

- [ ] `parse_iso8601(input) -> ParseResult<'_, DateTime<FixedOffset>>` — fixed-position, timezone
- [ ] `parse_syslog_timestamp(input) -> ParseResult<'_, NaiveDateTime>` — `Mmm dd HH:MM:SS`, month lookup
- [ ] `parse_httpdate(input) -> ParseResult<'_, DateTime<Utc>>` — RFC 2616 format
- [ ] `parse_timestamp(input, format) -> ParseResult<'_, DateTime<FixedOffset>>` — generic chrono format
- [ ] Unit tests: timezone offsets, millisecond/nanosecond precision, month abbreviations

> **Done when:** All 4 parsers handle their formats, `cargo test -p dfe-parse` passes.

#### 1.3.5 String parsers (`crates/dfe-parse/src/string.rs`)

- [ ] `take_word(input) -> ParseResult<'_, &str>` — alphanumeric + underscore
- [ ] `take_quoted(input) -> ParseResult<'_, &str>` — single/double quoted, escape handling, memchr
- [ ] `take_greedy(input) -> ParseResult<'_, &str>` — consume remaining (zero cost)
- [ ] `take_until(input, delimiter) -> ParseResult<'_, &str>` — memchr-accelerated
- [ ] `take_while(input, predicate) -> ParseResult<'_, &str>` — byte-level predicate
- [ ] `parse_loglevel(input) -> ParseResult<'_, &str>` — TRACE/DEBUG/INFO/WARN/ERROR/FATAL
- [ ] Unit tests: empty strings, escaped quotes, nested quotes, Unicode, all log levels

> **Done when:** All 6 parsers pass unit tests, memchr used for delimiters, `cargo test -p dfe-parse` passes.

#### 1.3.6 Network parsers (`crates/dfe-parse/src/network.rs`)

- [ ] `parse_mac(input) -> ParseResult<'_, &str>` — colon or dash separated hex octets
- [ ] `parse_hostname(input) -> ParseResult<'_, &str>` — RFC 1123 label-dot-label
- [ ] `parse_uri(input) -> ParseResult<'_, &str>` — scheme://authority/path?query#fragment
- [ ] `parse_email(input) -> ParseResult<'_, &str>` — local@domain
- [ ] `parse_uuid(input) -> ParseResult<'_, &str>` — 8-4-4-4-12 hex
- [ ] Unit tests: MAC colons vs dashes, hostname trailing dot, URI edge cases, UUID case

> **Done when:** All 5 parsers pass unit tests, `cargo test -p dfe-parse` passes.

#### 1.3.7 Criterion benchmark harness (`crates/dfe-parse/benches/parsers.rs`)

- [ ] Benchmark groups: ip, numeric, timestamp, string, network
- [ ] Each parser benchmarked against equivalent `regex::Regex` pattern
- [ ] Realistic input data from actual log lines

> **Done when:** `cargo bench -p dfe-parse` runs and produces comparison tables.

---

## Phase 2: Codegen Core

> **Parallelism:** 2.1 (Layer 2-3) and 2.2 (codegen fork) can run in parallel. 2.1 depends on 1.3. 2.2 depends on 1.2.

### 2.1 dfe-parse: Layer 2-3 (`crates/dfe-parse/`)

> **Depends on:** 1.3.1 (parser trait), at least 1.3.2-1.3.3 (IP + numeric parsers)

#### 2.1.1 Composite parser builder (`crates/dfe-parse/src/composite.rs`)

- [ ] `CompositeParser` builder — chain of `Literal` and `Capture` steps
- [ ] `CompositeParser::parse(input) -> Result<HashMap<&str, Value>>` — execute chain, return named captures
- [ ] Unit tests: 2-field parse, literal separators, mixed capture types, mismatch failure

> **Done when:** CompositeParser chains Layer 1 parsers with separators, `cargo test -p dfe-parse` passes.

#### 2.1.2 Grok pattern analyser

- [ ] `GrokAnalyser` — classify grok pattern into Layer 1 (all replaceable), Layer 2 (composite), Layer 3 (DFA)
- [ ] Grok alias expansion (`%{IP}` → regex → map to Layer 1 parser)
- [ ] Output: `AnalysisResult` with recommended layer + parser chain
- [ ] Unit tests: classify ~10 real grok patterns from Beats pipelines

> **Done when:** Analyser correctly classifies real grok patterns, `cargo test -p dfe-parse` passes.

#### 2.1.3 DFA fallback (`crates/dfe-parse/src/dfa.rs`)

- [ ] `DfaParser` wrapping `regex_automata::dfa` with named capture mapping
- [ ] `DfaParser::from_pattern(pattern) -> Result<Self>` — compile regex to DFA
- [ ] `DfaParser::parse(input) -> Result<HashMap<&str, &str>>` — execute, extract named captures
- [ ] Unit tests: simple pattern, named groups, serialise/deserialise round-trip

> **Done when:** DfaParser works as regex fallback, `cargo test -p dfe-parse` passes.

### 2.2 dfe-codegen: Fork + Adapt (`crates/dfe-codegen/`)

> **Depends on:** 1.2 (dfe-runtime Event + Transform trait must be defined)

#### 2.2.1 Import ANTLR4 Painless parser

- [ ] Copy ANTLR4 parser files from elastic_to_vrl (~15k lines) into `src/painless/parser/`
- [ ] Copy transpiler module (`transpiler.rs`, `script.rs`) into `src/painless/`
- [ ] Verify compiles: `cargo check -p dfe-codegen` passes

> **Done when:** Painless parser compiles within dfe-codegen.

#### 2.2.2 Import pipeline + processor structs

- [ ] Copy pipeline structs, AST, validation, YAML deserialisation into `src/pipeline/`
- [ ] Copy 28 processor struct definitions into `src/pipeline/processors/`
- [ ] Update `use` paths for new module structure
- [ ] Verify: `cargo check -p dfe-codegen` passes

> **Done when:** Pipeline YAML can be deserialised into Rust structs.

#### 2.2.3 Adapt Painless transpiler for Rust output

- [ ] Modify transpiler to emit Rust expressions instead of VRL
- [ ] Handle type coercion (VRL dynamic → `serde_json::Value` operations)
- [ ] Map Painless built-in functions to dfe-runtime equivalents
- [ ] Unit tests: transpile 5+ Painless snippets, verify output compiles

> **Done when:** Painless transpiler emits valid Rust for basic scripts.

#### 2.2.4 Rust code generation backend

- [ ] `RustTemplate` struct (replaces `ScriptTemplate`) for .rs file generation
- [ ] Output format: `pub fn transform(event: &mut Event) -> Result<TransformResult>`
- [ ] Processor dispatch: each processor maps to a code generation method
- [ ] Conditional handling: pipeline `if` conditions → Rust `if` expressions
- [ ] `on_failure` handling: `match` / `if let Err` blocks
- [ ] `ignore_failure`: wrap in `.ok()` / `let _ =`
- [ ] Generate `use dfe_runtime::prelude::*;` headers
- [ ] Unit tests: generate from 3+ pipeline YAMLs, verify valid Rust via `syn::parse_file`

> **Done when:** Generated Rust is syntactically valid for simple pipelines.

#### 2.2.5 CLI tool (clap)

- [ ] `main.rs` with clap: `dfe-codegen generate --pipeline <path> --output <dir>`
- [ ] Accept single YAML or directory of YAMLs
- [ ] `--dry-run` and `--verbose` flags
- [ ] Integration test: generate from test YAML, verify output file exists + compiles

> **Done when:** `cargo run -p dfe-codegen -- generate --pipeline test.yaml --output /tmp/out` produces valid .rs.

#### 2.2.6 End-to-end codegen integration test

- [ ] Test pipeline YAML with 3-4 simple processors
- [ ] Generate → compile → execute against test event → assert output
- [ ] Validates full YAML-to-execution roundtrip

> **Done when:** Full roundtrip works in a test, `cargo test -p dfe-codegen` passes.

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

### 2026-03-03: Project Initialisation

- [x] Research: parsing libraries, architecture, effort estimation (RESEARCH.md)
- [x] Git repo, ai/ci submodules attached
- [x] Cargo workspace: dfe-parse, dfe-runtime, dfe-codegen, dfe-transforms
- [x] All config files matching dfe-loader conventions
- [x] STATE.md, TODO.md, RESEARCH.md
- [x] Workspace compiles clean
- [x] docs/DESIGN.md — architecture, Mermaid diagrams, API contracts

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
