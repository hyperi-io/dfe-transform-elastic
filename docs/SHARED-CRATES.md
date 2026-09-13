# Shared crates: what dfe-transform-splack took a copy of

## Today: independent copies

`dfe-transform-splack` (the Splunk equivalent) took a copy of `dfe-parse` and
`dfe-runtime`. The two projects evolve their copies independently, and that is
deliberate: both are still changing shape, and extracting them now would make
every breaking change block both projects at once.

The duplication costs less than that coordination would.

**The workspace has since split to five crates** — `dfe-core`, `dfe-painless`,
`dfe-parse`, `dfe-runtime`, `dfe-transforms` — so what was one copy boundary is
now four. `dfe-runtime` re-exports the moved modules at their old paths, so a
copy taken against the old layout still compiles; it is reading a compatibility
surface, not the current one.

## What is common, conceptually

| Component | dfe-core | dfe-parse | dfe-runtime |
|-----------|----------|-----------|-------------|
| IP parsers (v4, v6, host) | | Yes | |
| Timestamp parsers (ISO 8601, syslog, epoch) | | Yes | |
| Numeric parsers (int, float) | | Yes | |
| String parsers (word, quoted, greedy) | | Yes | |
| Network parsers (MAC, hostname, URI) | | Yes | |
| Composite parser builder | | Yes | |
| DFA fallback (pre-compiled regex) | | Yes | |
| `Event` (dotted-path JSON wrapper) | Yes | | |
| Error types | Yes | | |
| Date formats and syslog priority | Yes | | |
| Regex cache and `cached_regex!` | Yes | | |
| `Transform` trait and `TransformChain` | | | Yes |
| Enrichment (GeoIP, user agent, community ID) | | | Yes |
| Grok cache | | | Yes |
| Prelude re-exports | | | Yes |
| Test utilities (harness, diff, flatten) | | | Yes |

## What is not

| Project | Specific code |
|---------|---------------|
| dfe-transform-elastic | the whole `dfe-painless` crate, and `codegen_api.rs` in `dfe-runtime` — Painless semantics have no Splunk counterpart |
| dfe-transform-splack | Splunk eval expressions and lookup transforms |

Putting the Painless matchers in their own crate makes that boundary explicit
in the manifest rather than a note in a table. It does not by itself prove
non-use: `dfe-runtime` re-exports `dfe-painless` at the old `painless_*` paths,
and `dfe-transforms` reaches the matchers that way rather than through a direct
edge. So a copy has to be checked for BOTH — a `dfe-painless` dependency, and
the re-export paths through `dfe-runtime`.

## Extracting later

When both sides settle:

1. Diff the two copies to find what is genuinely common, rather than assuming.
2. Stand up a shared private workspace repo. **The name `dfe-core` is not
   available for it**, and there are now two reasons rather than one:
   `hyperi-io/dfe-core` is an existing repo, and this workspace ships a crate
   called `dfe-core` that is a different thing — workspace-internal, never
   published. Anything extracted needs a third name.
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
