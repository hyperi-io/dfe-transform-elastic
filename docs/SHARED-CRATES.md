# Shared crates: dfe-parse and dfe-runtime

## Today: independent copies

`dfe-parse` and `dfe-runtime` are workspace crates here, and `dfe-transform-splack`
(the Splunk equivalent) took a copy of both. The two projects evolve their copies
independently, and that is deliberate: both are still changing shape, and extracting
them now would make every breaking change block both projects at once.

The duplication costs less than that coordination would.

## What is common, conceptually

| Component | dfe-parse | dfe-runtime |
|-----------|-----------|-------------|
| IP parsers (v4, v6, host) | Yes | |
| Timestamp parsers (ISO 8601, syslog, epoch) | Yes | |
| Numeric parsers (int, float) | Yes | |
| String parsers (word, quoted, greedy) | Yes | |
| Network parsers (MAC, hostname, URI) | Yes | |
| Composite parser builder | Yes | |
| DFA fallback (pre-compiled regex) | Yes | |
| `Event` (dotted-path JSON wrapper) | | Yes |
| `Transform` trait and `TransformChain` | | Yes |
| Enrichment (GeoIP, user agent, community ID) | | Yes |
| Error types | | Yes |
| Prelude re-exports | | Yes |
| Test utilities (harness, diff, flatten) | | Yes |

## What is not

| Project | Specific code |
|---------|---------------|
| dfe-transform-elastic | `painless_helpers.rs`, `painless_common.rs`, `codegen_api.rs` — Painless semantics have no Splunk counterpart |
| dfe-transform-splack | Splunk eval expressions and lookup transforms |

## Extracting later

When both sides settle:

1. Diff the two copies to find what is genuinely common, rather than assuming.
2. Stand up a shared private workspace repo. **`dfe-core` is already taken**, so the
   name is an open question.
3. Move the common code there; project-specific code stays put.
4. Both projects depend on it by git reference.

## When to do it

Not on a date — on these signals:

- Both parser sets have stopped growing.
- The `Event` API has settled: no breaking changes to get/set/remove.
- The `Transform` trait is identical in both.
- Test utilities are being copy-pasted between the two with no edits.
- A change to shared logic already requires updating both by hand.

Until then, divergence is expected. If a parser or an `Event` method is missing, add it
locally rather than waiting for the other project.
