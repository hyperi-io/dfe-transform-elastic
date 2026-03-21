# Transform Development Cycle

> Ported from Derek's elastic spike project. Battle-tested on CrowdStrike, Okta, Azure.

## The Deal

We can't fully automate Elastic pipeline → Rust. The pipelines are full of Painless
scripts (arbitrary Java-like code), implicit Elastic runtime behaviour, and field mapping
conventions that aren't formally specified anywhere. Trying to mechanically translate
100% of it would produce worse code than writing it by hand.

What works: codegen gets you ~70%, you hand-tune the rest. Same approach we used for
VRL in DFE 2.1 — worked then, works now. The codegen gives you the skeleton and the
easy bits (set, rename, remove, convert). You do the hard bits (complex conditionals,
Painless logic, enrichment wiring) and validate against real test data.

The key innovation is **Step 4a** — after hand-tuning a source, push reusable patterns
back into the codegen so the NEXT source needs less work. That's how codegen coverage
creeps from 70% toward 85%+.

---

## Per-Source (Steps 1-6)

Run these for each source (Okta, CrowdStrike, Azure, etc.). Order matters.

### 1. Codegen from Pipeline YAML

```bash
cargo run -p dfe-codegen -- generate \
    --pipeline /path/to/default.yml \
    --output crates/dfe-transforms/src/filebeat/<source>/
```

Produces ~70% correct Rust. Conditionals get transpiled to `if` guards. Painless
scripts fall back to `painless_exec()` stubs. Starting point, not finished product.

Done: generated code compiles (`cargo check -p dfe-transforms`).

### 2. Review Agents for Field Mapping

Look at the actual Beats/Agent source and Elastic integration docs:

- **Field mapping** — raw → ECS renames. This is where most pipeline work lives.
  e.g., `actor.alternateId` → `okta.actor.alternate_id`
- **Local parsing** — anything the agent does before the ingest pipeline
  (JSON decode, field extraction from structured logs)
- **Derived fields** — things the pipeline creates that aren't simple renames
  e.g., `event.action` from `eventType`, `event.outcome` from lowercased `outcome.result`

Ignore upstream transport parsing (syslog RFC, CEF, GELF). That's done before us.
We get structured JSON from Kafka.

Done: all expected ECS fields accounted for in the transform.

### 3. Triage Painless Scripts

Classify each Painless script in the pipeline:

- **Common pattern** → implement once as shared runtime function (drop nulls, email split, keys_to_snake_case)
- **Type coercion** → codegen handles via convert processor
- **Field extraction** → hand-tune with dfe-parse or regex
- **Complex logic** → hand-tune in Rust (conditional field derivation, array manipulation)
- **Skip** → leave as `painless_exec()` stub (rarely-hit edge cases)

Done: every Painless script has a classification and a plan.

### 4. Iterate Until >90% Match Rate

```bash
cargo test -p dfe-transforms --test integration_<source> -- --nocapture
```

Fix things as they come up:
- Codegen bugs → fix in dfe-codegen, regenerate
- Missing field mappings → add to transform
- Painless logic → implement in Rust or hand-tune

**Parity infrastructure** — assess per source, implement only where needed:
- **Foreach** — only if pipeline iterates arrays (Okta ip_chain, O365 Actor/Target)
- **Pipeline chaining** — only if pipeline references sub-pipelines (Fortinet, Cisco Meraki)
- **`_conf` injection** — only if pipeline uses `ctx._conf.*` (O365 tenant lookup)
- **On-failure handlers** — only if on_failure does real fallback (most are just log+continue)

Done: >90% match rate on happy-path fixtures (Semantic mode — skips @timestamp, GeoIP).

### 4a. Codegen Feedback

After hitting >90%, ask: **"Should the codegen have generated this?"**

Examples from real sessions:
- Epoch length conditional → added `try_string_length` to transpiler
- Negated list contains → added prefix negation to transpiler
- Parenthesised OR groups → added `strip_parens` to transpiler
- Grok field dot names → added `grok_to_regex_with_map` for name restoration

After fixing codegen:
1. Regenerate the current source
2. Re-test to verify
3. Regenerate OTHER sources that haven't been hand-tuned yet (they get the fix for free)

This is how the codegen gets smarter. Every hand-tune pushed back means less work
on the next source.

Done: all hand-tune patterns either in codegen or documented as "source-specific".

### 5. Fuzzing + Edge Cases

Harden beyond the happy path:
- `proptest` fuzzing — random field values, missing fields, nulls, type mismatches
- Web search for real-world malformed examples for this source type
- Edge cases — empty arrays, deeply nested objects, Unicode, max-length fields

Done: transform handles all edge cases without panicking. Errors return
`TransformResult::Continue` with logging, not event drops.

### 6. Complex Real-World Test Data

Go beyond Elastic's test fixtures:
- Web search for real-world sample events (security blogs, vendor docs, community forums)
- Multi-event sequences that exercise conditional branches
- Events that trigger enrichment (GeoIP, UA, Community ID)

Done: test fixtures cover all major code paths.

---

## Cross-Source (Steps 7-10)

Run after multiple sources have completed Steps 1-6.

### 7. Common Pattern Abstraction

Look for repeated patterns across transforms and lift them:
- **Macros** for repetitive codegen patterns (`field_rename!`, `conditional_set!`)
- **Shared runtime functions** (already have: `drop_empty`, `keys_to_snake_case`, `email_split`)
- **Parser extraction** to `dfe-parse` where parsers are duplicated across sources

Done: no pattern repeated more than twice across transforms.

### 8. Batch Processing Review

Where are we processing row-by-row when we could batch?
- Row → batch: process 20k events at once, not one at a time
- Regex → native parse: replace with `memchr`/`dfe-parse`
- Clone → borrow: use references from Kafka message buffer
- Sequential → SIMD: vectorise field operations across batch
- Columnar: extract same field from all events in one pass

Done: hot-path functions show zero allocations in `dhat` profiling.

### 9. Efficiency Review

Profile under realistic load:
- **CPU:** `cargo flamegraph` — transform shouldn't be the bottleneck (Kafka I/O should dominate)
- **Memory:** `dhat` — target zero allocations per event after init
- **Pressure:** test with constrained cgroup memory, verify no OOM
- **Latency:** p50/p95/p99 per-event transform latency

Done: no surprises in profiles. Pre-allocated structures only. Targets met.

### 10. Bake-Off

Benchmark against:
1. **Generated baseline** — Step 1 output before hand-tuning (measures optimisation payoff)
2. **Original VRL/Elastic** — the pipeline being replaced (validates the 10-20x claim)
3. **Alternative approaches** — if Steps 8-9 identified options, benchmark head-to-head

Use `criterion` with `Throughput::Elements(batch_size)`.

Done: results in `BENCHMARKS.md`. Performance targets met.

---

## Upstream Sync

When Elastic updates pipeline YAMLs (new Beats/Agent release):
1. Pull updated YAMLs from upstream
2. Re-run Step 1 for affected sources
3. Diff regenerated vs current hand-tuned code
4. Merge new processors/fields, keep hand-tuned optimisations
5. Re-run integration tests

---

## Iteration

If time permits, repeat Steps 1-6 per source, then 7-10 cross-source. Each pass
should show measurable improvement in match rate, coverage, or performance.

Step 4a is the compound interest — without it, every source costs the same effort.
With it, the codegen gets smarter and the hand-tune shrinks each round.
