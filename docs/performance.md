# Performance

What the hot path is built from: SIMD byte search, borrowed values, reused buffers, the release profile and the libraries underneath, plus where the benchmarks live. The parts are in [architecture.md](architecture.md), and the grok dispatch and its timings are [parsers.md](parsers.md).

`memchr` scans for delimiters in `crates/dfe-parse/src/string.rs`, and `aho-corasick` finds every agentless marker in one pass over the line. Both dispatch on the CPU features the build targets in `.cargo/config.toml`:

- `x86_64-unknown-linux-gnu`: `-C target-cpu=x86-64-v3` (AVX2, BMI1 and BMI2, FMA, Haswell and later)
- `aarch64-unknown-linux-gnu`: `-C target-cpu=generic` (the NEON baseline), linked with mold
- A build with no `--target`: `-C target-cpu=native`

JSON parsing is not SIMD. simd-json lost to `serde_json` on this workload ([transform.md](transform.md#interfaces)).

Values are borrowed where the types allow it. A parser returns `ParseResult<'a, &'a str>`, a slice of its input, and `event.get_str()` returns an `Option<&str>` into the event. A helper that may or may not change a string, such as `url_decode`, returns an `Option<Cow<'_, str>>`, borrowing unchanged input. Events hold their tree: `Event::from_json` parses each NDJSON line into an owned `serde_json::Value`.

`pipeline.rs` sizes its vectors from the batch with `Vec::with_capacity`, and `serialise_events` writes every event through one reused scratch buffer.

The release profile in `Cargo.toml` is thin LTO, one codegen unit, `panic = "abort"`, stripped, at `opt-level = 3`. The generated transform crates override it with `opt-level = 1` and 16 codegen units, which leaves the transform benches unchanged, because thin LTO re-optimises that code at the binary's level, and keeps each rustc's memory down. Splitting the modules across crates by package is what keeps each rustc inside a 16 GiB runner.

The benchmarks are Criterion, except `allocations`, which counts allocations with its own allocator and prints a report. Run one with `cargo bench -p <crate> --bench <name>`.

| Crate | Benches |
|---|---|
| root | `detect.rs`, `envelope.rs`, `original.rs` |
| `dfe-core` | `codec.rs`, `date_formats.rs`, `event.rs`, `hashing.rs` |
| `dfe-parse` | `parsers.rs`, each parser against its regex equivalent |
| `dfe-runtime` | `grok.rs`, `geoip.rs`, `json_parse.rs` |
| `dfe-transforms` | `transforms.rs`, `allocations.rs` |

## Key dependencies

The libraries the hot path is built on. `Cargo.toml` has the version ranges and the reason for each bound, and this is the map of which crate does what.

| Crate | Purpose |
|---|---|
| `serde` / `serde_json` | Serialisation, and both JSON paths. `preserve_order` is on, so object keys keep document order |
| `simd-json` | Dev-dependency of `dfe-runtime` alone, for `crates/dfe-runtime/benches/json_parse.rs`, the bench that demoted it |
| `scalo` | Config, logging, metrics, Kafka and gRPC transport, deployment, memory guard, scaling and DLQ: the service's runtime. Its `yaml` support parses the config on serde-saphyr |
| `memchr` | SIMD byte search (`crates/dfe-parse/src/string.rs`) |
| `aho-corasick` | The agentless marker scan (`src/detect/`) |
| `regex-automata` | The Layer 3 parser in `crates/dfe-parse/src/dfa.rs` |
| `regex` | Grok-derived pattern matching |
| `fancy-regex` | Backtracking fallback for the patterns `regex` rejects by design, lookaround and backreferences. Reached when `regex` fails to compile, so the hot path is unchanged |
| `psl` | Mozilla's Public Suffix List, compiled in, for `registered_domain` |
| `parking_lot` | The lock in front of the GeoIP cache, taken per event across one transform thread per partition |
| `chrono` | Timestamp handling |
| `maxminddb` | GeoIP lookups, memory-mapped |
| `base64` | Fingerprint and Community ID encoding |
| `sha1`, `sha2` | Fingerprint and Community ID digests, and Painless's `sha1()` and `sha256()` as lower-case hex |
| `csv` | The `csv` processor |
| `clap` | The service's command line |
| `tokio` | Async runtime |
| `thiserror` | Error types in `dfe-core`, `dfe-parse` and the service's `src/error.rs` |
| `tracing` | Logging |
| `criterion` | Benchmarks |
