# Shared Crates Strategy: dfe-parse + dfe-runtime

## Current State: Independent Copies

`dfe-parse` and `dfe-runtime` exist as workspace crates in this project. A copy of both was taken by `dfe-transform-splack` (the Splunk equivalent). Both projects evolve their copies independently during the spike phase.

This is intentional. Both projects are changing fast and will break fast while maturing. Extracting shared crates now would mean every breaking change blocks both projects simultaneously — a coordination tax that outweighs the duplication cost.

## What's Shared (Conceptually)

| Component | In dfe-parse | In dfe-runtime |
|-----------|-------------|----------------|
| IP parser (v4, v6, host) | Yes | |
| Timestamp parsers (ISO8601, syslog, epoch) | Yes | |
| Numeric parsers (int, float) | Yes | |
| String parsers (word, quoted, greedy) | Yes | |
| Network parsers (MAC, hostname, URI) | Yes | |
| Composite parser builder | Yes | |
| DFA fallback (pre-compiled regex) | Yes | |
| Event type (dotted-path JSON wrapper) | | Yes |
| Transform trait + TransformChain | | Yes |
| Enrichment (GeoIP, UA, Community ID) | | Yes |
| Error types | | Yes |
| Prelude re-exports | | Yes |
| Test utilities (harness, diff, flatten) | | Yes |

## What's NOT Shared (Project-Specific)

| Project | Specific Code |
|---------|--------------|
| dfe-transform-elastic | `painless_helpers.rs`, `painless_common.rs`, `codegen_api.rs` |
| dfe-transform-splack | (TBD — Splunk eval expression helpers, lookup transforms) |

## Plan: Extract When Stable

When both projects reach beta stability (post-spike):

1. Diff the two copies of `dfe-parse` and `dfe-runtime` to identify what's actually common
2. Create `hyperi-io/dfe-core` private GitHub repo as a Cargo workspace
3. Move genuinely shared code there
4. Both projects depend via git: `dfe-parse = { git = "ssh://git@github.com/hyperi-io/dfe-core.git" }`
5. Project-specific code stays in each project's workspace

## Coordination Between LLM Sessions

During the spike, both projects are developed largely by LLMs in separate sessions.

**No cross-project coordination needed.** Each LLM works in its own project workspace. If a parser or Event method is needed that doesn't exist, add it locally. Divergence is expected and acceptable.

**Periodic human review:** Derek reviews both projects periodically and identifies convergent patterns. These inform the eventual extraction.

## Signals That It's Time to Extract

- Both projects have stabilised their parser sets (no new parsers weekly)
- Event type API has settled (no breaking changes to get/set/remove)
- Transform trait is identical in both
- Test utilities are copy-pasted between projects with no changes
- A change to shared logic requires updating both projects manually
