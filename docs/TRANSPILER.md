# Painless-to-Rust Transpiler

> Ported from Derek's elastic spike research. Phase A done, B-D pending.

## Why Bother

Painless scripts are only 2.5% of pipeline processors (43 of 1,681) but they handle
the hard stuff — recursive map traversal, nested array restructuring, type-polymorphic
field handling, custom parsing. Without transpilation the generated transforms compile
but skip core logic. That's the gap between "it runs" and "it produces correct output".

---

## What The Scripts Actually Do

64 `painless_exec` calls across 14 transform files, from 43 unique scripts in 8 pipelines.

| Category | Count | What |
|----------|-------|------|
| Array/List iteration + transform | 13 | Loop, filter, rebuild structures |
| Simple field manipulation | 10 | Direct assignment, arithmetic |
| Conditional field setting | 8 | if/else to set fields based on input |
| Type conversion + polymorphic | 8 | Handle String/Long/List/Map variants |
| String operations | 4 | split, replace, substring |

Heaviest pipelines: O365 (12 scripts, very complex), Fortinet (8), CrowdStrike (6),
Azure signinlogs (4).

All scripts are pure transformations — no I/O, deterministic, no try/catch or threads.
8+ use `params` for external config (enum maps, field lists, timezone tables).

---

## What We've Got Working (Phase A)

Compile-time transpilation — codegen emits native Rust, no runtime interpretation.

| Component | File | What |
|-----------|------|------|
| IR types | `painless/ir.rs` | 25 Expr + 9 Stmt variants |
| Visitor | `painless/visitor.rs` | ANTLR parse tree → IR |
| Emitter | `painless/emitter.rs` | IR → Rust source string |
| Params | `painless/params.rs` | YAML params → inlined Rust literals |
| Helpers | `painless_helpers.rs` | Runtime bridge (truthiness, arithmetic, type coercion) |

Also working via pattern-matching in `painless_common.rs` (bypass the full transpiler):
- Recursive drop null/empty (`drop(ctx)`)
- `keys_to_snake_case` (recursive camelCase → snake_case)
- Email split (`splitOnToken('@')` → user.email, user.name, user.domain)
- Process command line extraction
- Epoch timestamp conversion (auto-detect precision)

### What's Left (Phases B-D)

**Phase B — Collections + iteration:**
HashMap/ArrayList methods, for loops, entrySet/keySet, removeIf with lambda, instanceof in loops

**Phase C — Functions + complex logic:**
Local function defs, recursion, regex find/match, bitwise ops

**Phase D — Integration:**
Wire into `emit_script()`, graceful fallback, regenerate all transforms, run integration tests

---

## Design Decisions

**Compile-time, not runtime.** The whole point is native Rust performance. All 43 scripts
are known at codegen time. No dynamically-constructed scripts.

**Reuse the ANTLR4 parser.** Already checked in (13,826 lines), compiling, grammar-complete.
Painless grammar is frozen. Writing a winnow replacement would be weeks for zero runtime benefit.

**Thin IR.** Simple intermediate representation, not a general AST. Separate "what does this
mean" from "how do we write it in Rust". Intentionally minimal — just what's needed for
these 43 scripts.

**All variables as `serde_json::Value`.** Painless is dynamically typed. Type inference would
be complex for marginal benefit. `Value` ops are already in the hot path.

**Params inlined at codegen.** Static params from pipeline YAML embedded directly in generated
code. Simple scalars inline, complex params use `lazy_static!`.

**Graceful fallback.** If transpilation fails, keep the `painless_exec()` stub. Incremental
rollout — transpile what we can, flag what we can't.

---

## Feature Mapping (Painless → Rust)

### Field Access
| Painless | Rust |
|----------|------|
| `ctx.event.duration` | `event.get("event.duration")` |
| `ctx.event.duration = val` | `event.set("event.duration", val)?` |
| `ctx?.event?.duration` | `event.get("event.duration")` (returns Option) |
| `ctx.remove('field')` | `event.remove("field")` |

### String Methods
| Painless | Rust |
|----------|------|
| `s.replace(a, b)` | `s.replace(a, b)` |
| `s.toLowerCase()` | `s.to_lowercase()` |
| `s.startsWith(p)` | `s.starts_with(p)` |
| `s.splitOnToken(t)` | `s.split(t).collect()` |
| `s.isEmpty()` | `s.is_empty()` |
| `s.length()` | `s.len()` |

### Collections
| Painless | Rust |
|----------|------|
| `new HashMap()` | `Value::Object(Map::new())` |
| `new ArrayList()` | `Value::Array(vec![])` |
| `m.containsKey(k)` | `obj.contains_key(k)` |
| `l.add(v)` | `arr.push(v)` |
| `l.removeIf(lambda)` | `arr.retain(\|item\| !predicate(item))` |
| `x instanceof Map` | `x.is_object()` |

### Control Flow
| Painless | Rust |
|----------|------|
| `if (cond) { ... }` | `if painless_truthy(&cond) { ... }` |
| `for (int i=0; i<n; i++)` | `while i < n { ... i += 1; }` |
| `for (x : list)` | `for x in arr { ... }` |
| `return;` | `return Ok(TransformResult::Continue);` |

### Arithmetic
| Painless | Rust |
|----------|------|
| `a + b` | `painless_add(&a, &b)` (string concat or numeric) |
| `a & b` | bitwise AND on i64 |
| `a >> b` | right shift on i64 |

---

## Raw Message Feed Reusability

Painless scripts that parse `ctx.message` or `ctx.description` should emit as standalone
reusable parser functions, not just inline transform code. This enables a future "raw
message feed" mode where the same parsers handle both Beats-fed and raw-fed sources.

```
dfe-parsers (standalone crate)
    Pure parsing functions from Painless scripts.
    No knowledge of Beats, Elastic Agent, or transport.
        |
dfe-transforms (thin wrapper)
    Handles Beats/Agent envelope: field naming, ECS mapping.
    Calls dfe-parsers for actual parsing.
```

Pinned scope item in TODO.md.

---

## Key Files

| What | Where |
|------|-------|
| ANTLR4 parser | `crates/dfe-codegen/src/painless/parser/` |
| Parse entry point | `crates/dfe-codegen/src/painless/script.rs` |
| IR types | `crates/dfe-codegen/src/painless/ir.rs` |
| Visitor (tree → IR) | `crates/dfe-codegen/src/painless/visitor.rs` |
| Emitter (IR → Rust) | `crates/dfe-codegen/src/painless/emitter.rs` |
| Params (YAML → inline) | `crates/dfe-codegen/src/painless/params.rs` |
| Runtime helpers | `crates/dfe-runtime/src/painless_helpers.rs` |
| Pattern matching | `crates/dfe-runtime/src/painless_common.rs` |
| Codegen integration | `crates/dfe-codegen/src/codegen/processor.rs` |
| Runtime API | `crates/dfe-runtime/src/codegen_api.rs` |
| Golden test data | `crates/dfe-codegen/src/pipeline/processors/script.rs` (7 test cases) |
