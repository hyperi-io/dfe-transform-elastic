# Transform Development Cycle

> Ported from Derek's elastic spike project. Refined during dfe-transform-elastic Phase 4.

## Why Semi-Automated

Full automation of Elastic pipeline → Rust is neither feasible nor sensible. The pipelines contain Painless scripts with arbitrary Java-like logic, complex conditionals with implicit Elastic runtime behaviour, and field mapping conventions that aren't formally specified. Attempting 100% mechanical translation would produce brittle, unoptimised code that's harder to maintain than hand-tuning.

This semi-automated approach (codegen ~70% → hand-tune ~30%) worked well for the VRL transforms in DFE 2.1 and we extend it for the native Rust transforms in DFE 2.2+. The codegen gives you the structural skeleton and the easy processors (set, rename, remove, convert, etc.); you hand-tune the complex logic, optimise the hot paths, and verify against real test data.

---

This is the standard development cycle for each Elastic source (Beat or Agent integration). The cycle is applied per-source (e.g., Okta, CrowdStrike, Azure) and repeats when Elastic upstream updates their pipeline YAMLs.

---

## Per-Source Cycle (Steps 1-6)

### Step 1: Codegen from Elastic Pipeline YAML

Generate Rust transform code from the Elastic ingest pipeline YAML using `dfe-codegen`.

```bash
cargo run -p dfe-codegen -- generate \
    --pipeline /path/to/default.yml \
    --output crates/dfe-transforms/src/filebeat/<source>/
```

The codegen produces ~70% correct code. Conditionals are transpiled to Rust `if` guards. Painless scripts fall back to `painless_exec()` stubs. This is the automated starting point — not the finished product.

**Done when:** Generated code compiles (`cargo check -p dfe-transforms`).

### Step 2: Review Source Agents for Missed Field Mapping + Parsing

Review the actual Beats/Agent source code and Elastic integration docs for:

- **Field mapping** — raw fields → ECS namespace renames (e.g., `actor.alternateId` → `okta.actor.alternate_id`). This is where most of the pipeline's work lives.
- **Local parsing** — any parsing or enrichment the agent performs before the ingest pipeline (e.g., JSON decode, field extraction from structured logs).
- **Enhancement** — fields the pipeline derives that aren't simple renames (e.g., `event.action` from `eventType`, `event.outcome` from `outcome.result` lowercased).

**Explicitly ignore:** Upstream transport parsing (syslog RFC, CEF, GELF, etc.). Assume that's done before data reaches us. We receive structured JSON from Kafka.

**Done when:** All expected ECS fields accounted for in the transform.

### Step 3: Triage Painless Scripts

Classify each Painless script in the pipeline:

| Category | Action | Example |
|----------|--------|---------|
| **Common pattern** | Implement once as shared runtime function | Drop null/empty values recursively |
| **Type coercion** | Codegen handles via `convert` processor | String → int, lowercase |
| **Field extraction** | Hand-tune with dfe-parse or regex | Extract username from email |
| **Complex logic** | Hand-tune in Rust | Conditional field derivation, array manipulation |
| **Skip** | Leave as `painless_exec()` stub | Rarely-hit edge cases |

**Done when:** Each Painless script has a classification and a plan.

### Step 4: Iterate Codegen + Test Until Match Rate >90%

Run the integration test against real Elastic fixture data:

```bash
cargo test -p dfe-transforms --test integration_<source> -- --nocapture
```

Fix issues iteratively:
- Codegen bugs → fix in `dfe-codegen`, regenerate
- Missing field mappings → add to transform
- Painless logic → implement in Rust (common patterns) or hand-tune

**Done when:** >90% match rate on happy-path fixtures (Semantic mode — skips @timestamp, GeoIP, @metadata).

### Step 5: Add Fuzzing, Known-Bad Inputs, Edge Cases

Harden the transform beyond the happy path:

- **Fuzzing:** Property-based testing with `proptest` — random field values, missing fields, null values, type mismatches.
- **Known-bad inputs:** Web search for real-world malformed examples for this source type (e.g., truncated JSON, encoding issues, missing required fields).
- **Edge cases:** Empty arrays, deeply nested objects, fields at max length, Unicode in field values.

**Done when:** Transform handles all edge cases without panicking. Error paths return `TransformResult::Continue` with appropriate error logging (don't drop events on parse failures).

### Step 6: Update Test Data with Complex Real-World Examples

Go beyond the Elastic-provided test fixtures:

- Web search for real-world sample events from this source (security blogs, vendor docs, community forums).
- Add multi-event sequences that exercise conditional branches.
- Add events that trigger enrichment paths (GeoIP, User Agent, Community ID).

**Done when:** Test fixtures cover all major code paths in the transform.

---

## Cross-Source Cycle (Steps 7-10)

These steps operate across all sources, not per-source. Run after multiple sources have completed Steps 1-6.

### Step 7: Common Pattern Abstraction

Review all transforms for repeated patterns and lift them to shared modules:

- **Rust macros** for repetitive codegen patterns (e.g., `field_rename!`, `conditional_set!`).
- **Shared runtime functions** for common operations (e.g., `drop_empty_values()`, `keys_to_snake_case()`, `extract_username_from_email()`).
- **Parser composition** — identify parsers that are duplicated across sources and extract to `dfe-parse`.

**Done when:** No pattern is repeated more than twice across transforms. Shared functions have their own tests.

### Step 8: Batch Processing Review

Identify where row-by-row processing can be converted to batch operations:

- **Row → batch:** Where are we processing one event at a time when we could process 20k?
- **Regex → native parse:** Where are regexes still used that could be replaced with `memchr`/`dfe-parse`?
- **Clone → borrow:** Where are we allocating when we could borrow from the Kafka message buffer?
- **Sequential → SIMD:** Where can field operations be vectorised across a batch?
- **Columnar access:** Where can we extract the same field from all events in a batch in one pass?

**Done when:** Hot-path functions show zero allocations in `dhat` profiling. Batch operations identified and documented even if not yet implemented.

### Step 9: Efficiency Review

Profile CPU and memory under realistic load:

- **CPU profiling:** `cargo flamegraph` to identify hot functions. Target: transform is not the bottleneck (Kafka I/O should dominate).
- **Memory profiling:** `dhat` to identify allocation sites. Target: zero allocations per event on the hot path after init.
- **Memory cap pressure:** Test with constrained cgroup memory to verify no OOM under batch load.
- **Latency distribution:** p50/p95/p99 per-event transform latency.

**Done when:** Flame graph shows no surprises. Memory profile shows pre-allocated structures only. Latency meets target.

### Step 10: Bake-Off

Benchmark the optimised Rust transforms against:

1. **Generated baseline** — the Step 1 codegen output before hand-tuning (measures optimisation payoff).
2. **Original VRL/Elastic pipeline** — the Vector/Elastic ingest pipeline being replaced (measures the 10-20x claim).
3. **Multiple implementation options** — if Step 8-9 identified alternative approaches, benchmark them head-to-head.

Use `criterion` with `Throughput::Elements(batch_size)` for consistent measurement.

**Done when:** Results documented in `BENCHMARKS.md`. Performance targets met.

---

## Upstream Sync

When Elastic releases new Beats/Agent versions with updated pipeline YAMLs:

1. Update pipeline YAMLs from upstream (git submodule or manual copy).
2. Re-run Step 1 (codegen) for affected sources.
3. Diff the regenerated code against the current hand-tuned version.
4. Merge new processors/fields while preserving hand-tuned optimisations.
5. Re-run integration tests to verify.

---

## Iteration

If time permits, repeat Steps 1-6 per source, then Steps 7-10 cross-source. Each iteration should show measurable improvement in match rate, test coverage, or performance.

The cycle is designed to be incremental — each step builds on the previous one, and you can stop at any step and have a working (if not yet optimal) transform.
