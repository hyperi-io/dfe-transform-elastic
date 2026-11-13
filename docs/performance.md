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

| Crate | Purpose |
|---|---|
| `serde` / `serde_json` | Serialisation framework, and both JSON paths. `preserve_order` is on, so object keys keep document order |
| `simd-json` | DEV-dependency of `dfe-runtime` only, for `crates/dfe-runtime/benches/json_parse.rs` -- the bench that demoted it |
| `scalo` (`yaml`) | YAML config parsing, through `scalo::yaml` on serde-saphyr |
| `memchr` | SIMD byte search (`crates/dfe-parse/src/string.rs`) |
| `regex-automata` | Pre-compiled DFA regex (`crates/dfe-parse/src/dfa.rs`) |
| `regex` | Grok-derived pattern matching in `dfe-transforms` |
| `fancy-regex` | Backtracking fallback for the vendor patterns `regex` rejects by design -- lookaround and backreferences. Reached only when `regex` fails to COMPILE, so the hot path is unchanged |
| `psl` | Mozilla's Public Suffix List, compiled in, for `registered_domain` |
| `parking_lot` | The lock in front of the GeoIP cache, taken per event across one transform thread per partition |
| `chrono` | Timestamp handling |
| `maxminddb` | GeoIP lookups (mmap) |
| `base64` | Fingerprinting and Community ID encoding |
| `sha1`, `sha2` | Fingerprint and Community ID digests, and Painless's `sha1()` and `sha256()` as lower-case hex |
| `csv` | CSV processor |
| `scalo` | Config, logging, metrics, Kafka transport, deployment, memory guard, scaling -- the service binary's runtime |
| `clap` | Service CLI surface |
| `tokio` | Async runtime |
| `thiserror` | Error types in every crate, including the service binary's `src/error.rs` |
| `tracing` | Logging |
| `criterion` | Benchmarking framework |
| `testcontainers` | Ephemeral Kafka broker for round-trip tests |
