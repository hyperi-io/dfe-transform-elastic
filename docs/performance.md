# Performance

What the hot path is built out of -- SIMD, zero-copy, reused buffers, the release profile and
the libraries underneath -- plus the targets the native parsers are held to and where the
benchmarks live. The code map around it is [architecture.md](architecture.md), and the parsers
themselves are [parsers.md](parsers.md).

---

## SIMD acceleration

| Library | Use | SIMD Features |
|---|---|---|
| `memchr` | Delimiter scanning in parsers (`string.rs`) | AVX2/SSE2 byte search |
| `regex-automata` | Layer 3 DFA fallback (`dfa.rs`) | Compiled DFA (no backtracking) |

JSON deserialisation is NOT on this list. simd-json was measured against
`serde_json` on this workload and lost on both paths -- see
[Event model](architecture.md#event-model).

Build targets in `.cargo/config.toml`:
- **x86_64 release:** `-C target-cpu=x86-64-v3` (AVX2, BMI1/2, FMA — Haswell+)
- **aarch64 release:** `-C target-cpu=generic` (NEON baseline)
- **Development:** `-C target-cpu=native`

---

## Zero-copy patterns

1. **Kafka → Event:** `Event::from_bytes` borrows from the message buffer rather than copying it
2. **Parser returns:** `ParseResult<'a, &'a str>` — parsed values reference input slice
3. **Field access:** `event.get_str()` returns `Option<&str>` borrowing from inner Value
4. **Cow for transforms:** `Cow<'_, str>` when field may or may not need modification

---

## Buffer reuse

- Pre-allocated `Vec<u8>` buffers for serialisation output
- `String` buffers cleared and reused across events for format operations
- `Vec::with_capacity()` based on expected batch sizes

---

## Release profile

```toml
[profile.release]
lto = "thin"
codegen-units = 1
panic = "abort"
strip = true
opt-level = 3
```

---

## Native parser targets

| Operation | Regex/Grok | Native Rust | Expected Speedup |
|---|---|---|---|
| IP address parse | ~200ns | ~15ns | ~13x |
| Timestamp (ISO8601) | ~500ns | ~40ns | ~12x |
| Integer parse | ~100ns | ~10ns | ~10x |
| Syslog header (5 fields) | ~2us | ~100ns | ~20x |
| Full DNS log line (8 fields) | ~5us | ~300ns | ~15x |

JSON parsing is deliberately absent: it is not a grok replacement, and the one
candidate that would have belonged here lost its own benchmark
(`crates/dfe-runtime/benches/json_parse.rs`).

---

## Benchmark framework

- **Per-parser:** Criterion benchmarks comparing each dfe-parse parser against its regex equivalent (`crates/dfe-parse/benches/parsers.rs`)
- **End-to-end:** Full pipeline (JSON deserialise → transform chain → serialise)

---

## Key dependencies

The libraries the hot path is built on. `Cargo.toml` carries the version ranges and the reason
for each bound; this is the map of which crate does what.

| Crate | Version | Purpose |
|---|---|---|
| `serde` / `serde_json` | >=1.0 | Serialisation framework, and both JSON paths. `preserve_order` is on, so object keys keep document order |
| `simd-json` | >=0.14, <0.15 | DEV-dependency of `dfe-runtime` only, for `crates/dfe-runtime/benches/json_parse.rs` -- the bench that demoted it |
| `serde_yaml_ng` | >=0.10 | YAML config parsing |
| `memchr` | >=2.7 | SIMD byte search (`crates/dfe-parse/src/string.rs`) |
| `regex-automata` | >=0.4 | Pre-compiled DFA regex (`crates/dfe-parse/src/dfa.rs`) |
| `regex` | >=1.10 | Grok-derived pattern matching in `dfe-transforms` |
| `fancy-regex` | >=0.13 | Backtracking fallback for the vendor patterns `regex` rejects by design -- lookaround and backreferences. Reached only when `regex` fails to COMPILE, so the hot path is unchanged |
| `psl` | >=2.1 | Mozilla's Public Suffix List, compiled in, for `registered_domain` |
| `parking_lot` | >=0.12.5, <0.13 | The lock in front of the GeoIP cache, taken per event across one transform thread per partition |
| `chrono` | >=0.4 | Timestamp handling |
| `maxminddb` | >=0.27, <0.28 | GeoIP lookups (mmap, simdutf8) |
| `rustc-hash`, `base64` | >=2.0, >=0.22 | Fingerprinting and Community ID hashing |
| `sha1`, `sha2` | >=0.10, <0.11 | The same, capped: 0.11 returns a `hybrid_array::Array` with no `LowerHex`, so the hex formatting stops compiling |
| `url` | >=2.5 | URL parsing (`uri_parts`) |
| `csv` | >=1.3 | CSV processor |
| `scalo` | >=2.11.1, <3 | Config, logging, metrics, Kafka transport, deployment, memory guard, scaling — the service binary's runtime |
| `clap` | >=4.5 | Service CLI surface |
| `tokio` | >=1.48 | Async runtime |
| `thiserror` | >=2.0 | Error types in every crate, including the service binary's `src/error.rs` |
| `anyhow` | >=1.0 | Error handling inside `dfe-runtime` and `dfe-transforms` |
| `tracing` | >=0.1 | Logging |
| `criterion` | >=0.5, <0.6 | Benchmarking framework |
| `testcontainers` | >=0.27, <0.28 | Ephemeral Kafka broker for round-trip tests |
