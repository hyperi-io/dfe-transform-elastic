# Parsers and processors

The three-layer parser strategy that replaces grok and regex, every ingest processor the
transforms use, and how far the Painless matchers reach. The code map around it is
[architecture.md](architecture.md), and the speed numbers behind the layers are
[performance.md](performance.md).

---

## Three layers over grok and regex

Three-layer strategy for replacing grok/regex with native Rust, implemented in `dfe-parse`.
Layer 1 is WIRED, for the two whole-pattern forms `grok_cache::native_form` recognises;
everything else still falls back to the compiled regex the grok expands to. The dispatch table
below is the current state, and widening it is the gap this architecture exists to close.

```mermaid
flowchart TD
    input[Grok Pattern String] --> expand[Expand grok aliases<br/>%{IP} → regex]
    expand --> analyse{Analyse each<br/>capture group}

    analyse -->|All replaceable| l1[Layer 1: Native Parsers<br/>parse_ipv4, parse_int, etc.<br/><b>10-20x faster</b>]
    analyse -->|Some replaceable| l2[Layer 2: Composite<br/>Chain L1 parsers +<br/>literal separators<br/><b>10-15x faster</b>]
    analyse -->|Irreducibly complex| l3[Layer 3: DFA Fallback<br/>regex-automata pre-compiled<br/><b>2-5x faster</b>]

    l1 --> output[Structured fields]
    l2 --> output
    l3 --> output

    style l1 fill:#2d5,stroke:#1a3,color:#fff
    style l2 fill:#28a,stroke:#167,color:#fff
    style l3 fill:#a72,stroke:#841,color:#fff
```

---

## Layer 1: common pattern replacements (`ip.rs`, `numeric.rs`, `string.rs`, `timestamp.rs`)

Zero-regex native Rust parsers for the 15 most common grok patterns:

| Grok Pattern | Rust Parser | Technique |
|---|---|---|
| `%{IPV4}` | `parse_ipv4()` | Octet validation, direct byte scan |
| `%{IPV6}` | `parse_ipv6()` | Colon-group parser |
| `%{IPORHOST}` | `parse_ip_or_host()` | Try IP first, fall back to hostname |
| `%{INT}` | `parse_int::<T>()` | Direct digit scan, generic over type |
| `%{NUMBER}` | `parse_number()` | Integer or float with optional exponent |
| `%{WORD}` | `take_word()` | Byte-level alphanumeric scan |
| `%{GREEDYDATA}` | `take_greedy()` | Return remaining slice (zero cost) |
| `%{TIMESTAMP_ISO8601}` | `parse_iso8601()` | Fixed-position field extraction |
| `%{SYSLOGTIMESTAMP}` | `parse_syslog_timestamp()` | Month lookup table + digit parse |
| `%{LOGLEVEL}` | `parse_loglevel()` | Case-insensitive keyword match |
| `%{QUOTEDSTRING}` | `take_quoted()` | memchr for quote, handle escapes |
| `%{HOSTNAME}` | `parse_hostname()` | Label-dot-label (RFC 1123) |
| `%{MAC}` | `parse_mac()` | Fixed 6-group hex parse |
| `%{URI}` | `parse_uri()` | scheme://authority/path?query#fragment |
| `%{UUID}` | `parse_uuid()` | Fixed 8-4-4-4-12 hex pattern |

**What is WIRED is a much smaller set than what exists.** The table above is
`dfe-parse`'s capability, not the dispatch. `grok_cache::native_form` recognises
whole-pattern forms only, and by design refuses a near-miss rather than guessing
at one — a pattern with literal text around its captures stays on the regex,
because the regex engine is good at exactly that. Three forms are dispatched
today:

| Pattern | Path | Measured |
|---|---|---|
| `^%{IPV4:f}$` | `dfe_parse::ip::parse_ipv4` | 29 ns against 214 ns |
| `^%{IPV4:a}:%{PORT:p}$` | the two chained | 47 ns against 208 ns |
| `%{GREEDYDATA:f}`, anchored or not | `split_once('\n')` | 11 ns against 1,098 ns |

The third reaches no parser at all: `line_anchored` makes it `(?m)^.*$` because
joni anchors to lines, so it captures the FIRST LINE and the native form is a
newline scan. It covers 202 call sites over 37 distinct patterns, 4.7% of the
4,299 grok sites in the generated tree (3,496 `cached_grok!` plus 803
`cached_grok_mapped!`).

Those are DERIVED, not typed: re-count them from the generated tree rather than
editing them. The previous pair -- 194 sites, 10.2% of 1,906 -- was measured before the
tree was regenerated whole, and the share fell because the denominator more
than doubled, not because the native form lost ground.

`^%{DATA:f}$` is deliberately excluded despite reading the same: `%{DATA}` is
the lazy `.*?`, and on a `\r\n` line ending greedy keeps the `\r` in the capture
where lazy stops before it.

Every dispatched form carries an equivalence test running both paths over the
same inputs, because the native path is an optimisation over the regex and never
a replacement for it.

**Parser convention:** All parsers take `&str`, return `ParseResult<'_, T>` where
`T` is the parsed value (often `&str` for zero-copy):

```rust
pub type ParseResult<'a, T> = Result<(&'a str, T), ParseError>;
```

The return tuple contains `(remaining_input, parsed_value)`, following winnow/nom convention.

---

## Layer 2: composite parser builder (`composite.rs`)

Chains Layer 1 parsers with literal separators for full grok-equivalent patterns:

```rust
// Grok: %{IPORHOST:source_ip}:%{INT:source_port} -> %{IPORHOST:dest_ip}:%{INT:dest_port}
// Becomes:
let parser = CompositeParser::builder()
    .capture("source_ip", parse_ip_or_host)
    .literal(":")
    .capture("source_port", parse_int::<u16>)
    .literal(" -> ")
    .capture("dest_ip", parse_ip_or_host)
    .literal(":")
    .capture("dest_port", parse_int::<u16>)
    .build();
```

---

## Layer 3: pre-compiled DFA fallback (`dfa.rs`)

For patterns that can't be decomposed into Layer 1/2:

- Compile regex to DFA at build time via `regex-automata`
- Serialise DFA bytes, load at runtime with zero compilation cost
- Named capture groups mapped to field names

---

## Processor taxonomy

Every processor the transforms use, by implementation weight, runtime status, and which
sources exercise it.

### Simple (~11) — direct `Event` API calls, no parser dependency

| Processor | Status | Used By | Code Pattern |
|---|---|---|---|
| `set` | Done | All | `event.set(path, value)?` |
| `append` | Done | All | `event.append(path, value)?` |
| `remove` | Done | All | `event.remove(path)` |
| `rename` | Done | All | `event.rename(from, to)?` |
| `uppercase` | Done | Panw | `event.set(path, s.to_uppercase())?` |
| `lowercase` | Done | All | `event.set(path, s.to_lowercase())?` |
| `trim` | Done | Cisco | `event.set(path, s.trim())?` |
| `convert` | Done | All | Type coercion: `str→i64`, `str→f64`, `i64→str` |
| `drop` | Done | All | `return Ok(TransformResult::Drop)` |
| `split` | Done | O365 | `event.set(path, s.split(sep).collect())?` |
| `join` | Done | SentinelOne | `join_values(value, sep)`, `None` on a non-array |
| `uri_parts` | Done | Panw | Scheme, host, port, path, query components |
| `dot_expander` | Done | GCP | `dot_expand(event, path, field)` |
| `fail` | Done | CrowdStrike | Returns a `TransformError` carrying the message |
| `terminate` | Done | Defender | Stops the pipeline and KEEPS the document |
| `urldecode` | Done | Zscaler | `url_decode`, form-encoded so `+` is a space |
| `sort` | Done | M365 Defender | `sort_values`, errors on anything with no natural ordering |

### Medium (~8) — read a field value and reshape it

| Processor | Status | Used By | Code Pattern |
|---|---|---|---|
| `grok` | Done (regex fallback) | All | `grok_to_regex` builds a pattern, `regex::Regex` matches it |
| `dissect` | Done | Cisco, Meraki | Tokenizer-based split parser |
| `json` | Done | All | `codegen_api::parse_json_str`, on `serde_json` |
| `kv` | Done | Okta | Key-value split, configurable delimiters |
| `csv` | Done | Fortinet | Separator/quote config |
| `foreach` | Done | Okta, O365, sysmon | Each element bound to `_ingest._value`, the inner processor run, the list rebuilt |
| `date` | Done | All | `chrono` against the formats a source emits |
| `gsub` | Done | O365 | `regex::Regex::replace_all` |

### Complex (~8) — enrichment runtime or a Painless pattern match

| Processor | Status | Used By | Code Pattern |
|---|---|---|---|
| `script` (Painless) | Pattern-match, see below | All | Rust matching a recognised script pattern, or a no-op |
| `geoip` | Done (DB-IP) | All with IPs | `enrichment::geoip::enrich(event, field, prefix)?` |
| `user_agent` | Done | O365, Okta | `enrichment::user_agent::enrich(event, field, prefix)?` |
| `community_id` | Done | Panw, Fortinet | `enrichment::community_id::enrich(event)?` |
| `registered_domain` | Done | Panw | `codegen_api::registered_domain_lookup`, a `psl` lookup against Mozilla's Public Suffix List |
| `network_direction` | Done | Fortinet, Panw | CIDR-based internal/external classification |
| `fingerprint` | Done | O365, M365 | `fingerprint_default`: SHA-1, base64, NUL before each value. `salt` and `method` are refused, so the default is the only path. |
| `pipeline` (nested) | Done | most multi-pipeline packages -- derive with `rg -l "Begin nested pipeline"` | Inlined into the caller, not a call |

Not implemented (not used by any vendored pipeline): bytes, cef, date_index_name,
enrich, geo_grid, html_strip, inference, redact, reroute, set_security_user.
The generator ERRORS on one rather than skipping it, so a package that starts
using one fails to onboard and says which -- that is how `join`, `sort`,
`urldecode`, `dot_expander`, `fail` and `terminate` got written.

---

## Painless coverage

A script the runtime cannot execute is skipped rather than failing the event, so the skips have
to be counted or they are indistinguishable from a script that did nothing. `painless_exec`
lives in `crates/dfe-painless/src/plan.rs` and is re-exported through
`crates/dfe-runtime/src/codegen_api.rs` for the generated call sites. It resolves a
hand-transcribed runner from `crates/dfe-painless/src/bespoke/` first, then the params patterns
in `params.rs`, then the `known_patterns` ladder in `common.rs`, and records the outcome
through `crates/dfe-painless/src/stats.rs`.

**Two measurements, and only the second is honest about reach.**

`crates/dfe-transforms/tests/painless_coverage.rs` drives the committed fixtures and holds a
floor of 100%, none skipped. That floor is real but its driver is narrow -- a few dozen fixture
files against the compat corpus's several hundred data streams -- so it says the fixtures are
fully covered, not that the runtime is.

Running the same counters over the compat corpus is what says that. Both figures are run
outputs rather than constants, so derive them rather than quoting a number from here: the
corpus run prints handled, skipped and distinct-script counts, `never_ran` is ratcheted in
`tests/compat-baseline.json`, and `DFE_PAINLESS_UNHANDLED=<path>` on the corpus test writes the
per-script detail.

The gap between the two is the point. A pattern can MATCH a script statically and then decline at
run time, which reads as covered from everywhere except that dump - so
`scripts/pattern_reach.py` joins it against the static census
(`DFE_BINDING_DUMP=<path>` on `painless_binding`) to name the patterns that claim a script and
never apply it.

---

## Future: dfe-parsers, a standalone parser crate

**Vision:** extract the per-source message-parsing logic in `dfe-transforms` into a standalone
`dfe-parsers` crate that any DFE Rust project can depend on. It would take `&str` / `Value`
input and return structured output with no knowledge of Beats, Elastic Agent, or transport, so
a syslog feed project outside this repo could parse a raw line and get the same structured
output as if it had come through Beats.

`dfe-transforms` would become a thin layer over it: Beats/Agent envelope handling, ECS field
naming, and the enrichment calls, with the actual message parsing delegated out.

The nearer half of that is already standing: `dfe-parse` is a workspace crate of its own, and
dfe-transform-splack runs a copy of it -- [shared-crates.md](shared-crates.md) is the account of
what the two projects share and what it would take to extract it.
