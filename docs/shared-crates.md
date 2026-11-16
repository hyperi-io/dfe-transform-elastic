# Shared crates: what dfe-transform-splack took a copy of

dfe-transform-splack, the Splunk counterpart, has copies of `dfe-parse` and `dfe-runtime`, and no extraction is scheduled. The two projects evolve their copies apart on purpose: both are changing, and a shared crate would make every breaking change block both at once. The duplication costs less than that coordination would.

This workspace has since split its library code into five crates, `dfe-core`, `dfe-painless`, `dfe-parse`, `dfe-runtime` and `dfe-transforms`, so the one copy boundary is four. `dfe-runtime` re-exports the moved modules at their old paths, so a copy taken against the old layout still compiles, against a compatibility surface rather than the current one.

The common half, by crate:

| Component | dfe-core | dfe-parse | dfe-runtime |
|---|---|---|---|
| IP parsers (v4, v6, host) | | Yes | |
| Timestamp parsers (ISO 8601, syslog, epoch) | | Yes | |
| Numeric parsers (int, float) | | Yes | |
| String parsers (word, quoted, greedy) | | Yes | |
| Network parsers (MAC, hostname, URI) | | Yes | |
| Composite parser builder | | Yes | |
| DFA fallback | | Yes | |
| `Event` (dotted-path JSON wrapper) | Yes | | |
| Error types | Yes | | |
| Date formats and syslog priority | Yes | | |
| Regex cache and `cached_regex!` | Yes | | |
| `Transform` trait and `TransformChain` | | | Yes |
| Enrichment (GeoIP, user agent, community ID) | | | Yes |
| Grok cache | | | Yes |
| Prelude re-exports | | | Yes |
| Test utilities (harness, diff, flatten) | | | Yes |

What is not common: the whole `dfe-painless` crate and `codegen_api.rs` in `dfe-runtime`, because Painless has no Splunk counterpart, and splack's Splunk eval expressions and lookup transforms. A separate `dfe-painless` makes that boundary visible in the manifest without proving non-use: `dfe-runtime` re-exports `dfe-painless` at the old `painless_*` paths, and the generated transforms reach the matchers that way. A copy has to be checked for both, a `dfe-painless` dependency and the re-export paths.

## Extracting later

When both sides settle:

1. Diff the two copies to find what is common, rather than assuming.
2. Stand up a shared private workspace repo. It needs a name other than `dfe-core`, which this workspace uses for an unpublished internal crate.
3. Move the common code there, and leave project-specific code where it is.
4. Both projects depend on it by git reference.

The signals that it is time: both parser sets have stopped growing, the `Event` API has had no breaking change to get, set or remove, the `Transform` trait is identical in both, test utilities are copied between the two with no edits, and a change to shared logic needs both updated by hand. Until then divergence is expected. A missing parser or `Event` method is added locally rather than waiting for the other project.
