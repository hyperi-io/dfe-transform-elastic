# Painless-to-Rust Transpiler

Research, design, and implementation notes for transpiling Elastic's Painless
scripting language to native Rust code within dfe-codegen.

**Status:** Phase A complete (IR, emitter, visitor, params, helpers). Phases B-D pending.

---

## Why This Matters

Painless scripts are only 2.5% of pipeline processors (43 of 1,681) but they
handle the most complex transformation logic — recursive map traversal, nested
array restructuring, type-polymorphic field handling, and custom parsing. Without
real transpilation, the generated transforms are hollow shells that compile but
skip core logic.

---

## Scope: What Painless Scripts Actually Do

### Distribution

| Pipeline | Script Count | Complexity |
|----------|-------------|------------|
| o365/default | 12 | Very high — policy rules, nested iteration, enum mapping |
| fortinet/default | 8 | High — custom KV parser, syslog priority, recursive null clean |
| crowdstrike/default | 6 | High — timestamp conversion, command-line parsing, tag extraction |
| azure_signinlogs | 4 | High — recursive keysToSnakeCase, list-to-map |
| okta/default | 3 | Medium — recursive null/empty pruning, behaviour filtering |
| cisco_nexus/default | 3 | Medium — recursive pruning, syslog priority |
| panw/* | 5 | Low-Medium — hash algorithm detection, TLS version parsing |
| azure_activitylogs | 8 | Medium — claims extraction, category mapping, duration conversion |
| azure_auditlogs | 2 | High — deep targetResources restructuring |
| azure_platformlogs | 5 | Medium — category mapping, message replacement |
| cisco_ios | 2 | Medium — timezone mapping |
| cisco_meraki | 2 | Medium — type checking |
| fortinet/utm | 1 | Low |

**Total:** 64 `painless_exec` calls across 14 generated transform files,
sourced from 43 unique Painless scripts in 8 pipeline YAML files.

### Script Categories

| Category | Count | % | Description |
|----------|-------|---|-------------|
| Array/List iteration & transformation | 13 | 30% | Loop, filter, rebuild structures |
| Conditional field setting | 8 | 19% | if/else to set fields based on input |
| Type conversions & polymorphic handling | 8 | 19% | Handle String/Long/List/Map variants |
| Simple field manipulation | 10 | 23% | Direct assignment, arithmetic |
| String operations | 4 | 9% | split, replace, substring |

### Key Properties

- **All scripts are pure transformations** — no I/O, no external API calls, no side effects beyond `ctx` mutation
- **Deterministic** — same input always produces same output
- **No advanced Painless features** — no try/catch, no threads, no class definitions
- **Parameterised** — 8+ scripts use `params` object for external configuration (enum maps, field lists, timezone tables)

---

## Painless Language Features Actually Used

| Feature | Usage Count | Example |
|---------|------------|---------|
| Field access (`ctx.field`, `ctx['field']`) | 64 | All scripts |
| Null-safe navigation (`?.`) | 15+ | `ctx?.azure?.activitylogs?.properties` |
| Type checking (`instanceof`) | 16 | `if (x instanceof Map)` |
| For loops (C-style and for-each) | 18 | `for (def i = 0; i < n; i++)`, `for (x : list)` |
| Function definitions | 5 | `def splitUnquoted(String input, String sep)` |
| HashMap/ArrayList creation | 28 | `new HashMap()`, `new ArrayList()` |
| Regex literals | 6 | `/([a-z])([A-Z]+)/` |
| Lambda/closure | 4 | `removeIf(v -> drop(v))` |
| String methods | 40+ | replace, split, substring, toLowerCase, trim, startsWith |
| Map methods | 20+ | get, put, containsKey, keySet, entrySet, remove |
| List methods | 15+ | add, get, contains, size, removeIf |
| Arithmetic & bitwise | 8 | `priority & 0x7`, `priority >> 3`, `duration * 1e9` |
| Recursion | 2 | `drop(object)`, `keysToSnakeCase(map)` |
| Type casting | 3 | `Long.parseLong()`, `(char)"\"" ` |

### Features NOT Used (Can Ignore)

- `while` / `do-while` loops
- try/catch/finally
- throw
- break/continue (only implicit via return)
- Class definitions
- Ternary operator `? :`
- Elvis operator `?:`
- Pattern.compile (only regex literals)
- Multi-dimensional arrays
- Bitwise NOT, XOR

---

## Predecessor: elastic_to_vrl Painless Transpiler

### Architecture

Located at `/projects/elastic_to_vrl/painless/`. Uses ANTLR4-generated parser
with visitor pattern to walk the Painless AST and emit VRL string output.

### Reusable Components

| Component | Reusability | Notes |
|-----------|------------|-------|
| ANTLR4 Painless parser (checked-in generated code) | **High** | Parses Painless → AST. Grammar-complete. |
| Visitor trait (`PainlessParserVisitor`) | **High** | 40+ visit methods defining all AST node types |
| `DynamicVisitor` for path analysis | **High** | Analyses `ctx.foo[bar].baz` → path segments |
| Method mapping tables (String/List/Map) | **Medium** | What methods exist; output format changes |
| `Script::parse()` entry point | **Medium** | Parser orchestration, reusable with adaptation |

### Throwaway Components

| Component | Why |
|-----------|-----|
| VRL string generation | Different target language entirely |
| gtmpl (Go templating) for code emission | VRL-specific complexity |
| Type-widening blocks (`if false { x = null }`) | VRL type system workaround |
| Control flow flags (`_shouldSkip_N`) | VRL lacks return statements |
| `vrl` crate dependency | Not needed |

### Supported Painless Features (in predecessor)

**Implemented:**
- String methods: length, toLowerCase, toUpperCase, startsWith, endsWith, substring, trim, replace, splitOnToken, isEmpty
- Array methods: length, size, add, get, contains, removeIf (with lambda)
- Map methods: get, put, containsKey, keySet, entrySet, clone, remove
- Type system: Long.parseLong, Pattern.compile, instanceof (String/List/Map/long)
- Control flow: if/else, for (C-style), for-each, for-in, return
- Collections: HashMap, ArrayList, map/list initialisers
- Operators: arithmetic, comparison, boolean, assignment, compound assignment (+=)

**Not implemented in predecessor:**
- while/do-while, try-catch, break/continue, throw
- Ternary `? :`, elvis `?:`
- Regex literals and matcher methods
- Pre/post increment/decrement
- Reference type casts

### Key Dependencies

- `antlr-rust` 0.3.0-beta — ANTLR4 Rust runtime (parser infrastructure)
- No `.g4` grammar files in repo — generated parser code is checked in

---

## Current Codegen Pipeline

### Flow

```
Pipeline YAML → dfe-codegen parse → Processor list → emit_processor() → .rs file
                                          |
                              script processor detected
                                          |
                              emit_script() → painless_exec(event, "source")?;
```

### Current Script Emission (to be replaced)

**File:** `crates/dfe-codegen/src/codegen/processor.rs` (lines 1236-1261)

```rust
fn emit_script(p: &script::Script, pad: &str) -> Result<String> {
    // Emits: painless_exec(event, r#"<escaped_source>"#)?;
}
```

### Current Runtime Stub (to be replaced)

**File:** `crates/dfe-runtime/src/codegen_api.rs:85-97`

```rust
pub fn painless_exec(event: &mut Event, script: &str) -> Result<()> {
    debug!("painless_exec: script skipped (transpiler pending)");
    Ok(())  // NO-OP — this is what we're fixing
}
```

### Generated Code Context

Scripts are emitted inline within `Transform::transform()` method bodies:

```rust
impl Transform for Default {
    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        // ... other processors (set, rename, convert, etc.) ...

        // Painless script
        // Source: ctx.event.duration = ctx.event.duration * 1000000000
        painless_exec(event, r#"ctx.event.duration = ctx.event.duration * 1000000000"#)?;

        // ... more processors ...
        Ok(TransformResult::Keep)
    }
}
```

Wrappers used:
- `ignore_failure: true` → `let _ = (|| -> Result<()> { ... Ok(()) })();`
- Conditional → `// TODO: conditional: <expr>` + braces

---

## Representative Painless Scripts (by complexity)

### Simple: Duration conversion
```painless
ctx.event.duration = ctx.event.duration * 1000000000
```
**Rust equivalent:** `event.set("event.duration", json!(event.get_i64("event.duration").unwrap_or(0) * 1_000_000_000))?;`

### Medium: Enum mapping with params
```painless
def schemaId = ctx.o365audit.RecordType.toString();
def schema = params[schemaId];
if (schema != null) {
  if (ctx.event == null) { ctx.event = new HashMap(); }
  ctx.event.code = schema;
}
```

### Complex: Recursive null/empty pruning
```painless
boolean drop(Object object) {
  if (object == null || object == '') { return true; }
  else if (object instanceof Map) {
    ((Map) object).values().removeIf(v -> drop(v));
    return (((Map) object).size() == 0);
  } else if (object instanceof List) {
    ((List) object).removeIf(v -> drop(v));
    return (((List) object).length == 0);
  }
  return false;
}
drop(ctx);
```

### Very Complex: Custom KV parser with quoting
```painless
def splitUnquoted(String input, String sep) {
  def tokens = [];
  def startPosition = 0;
  def isInQuotes = false;
  char quote = (char)"\"";
  for (def currentPosition = 0; currentPosition < input.length(); currentPosition++) {
    if (input.charAt(currentPosition) == quote) { isInQuotes = !isInQuotes; }
    else if (input.charAt(currentPosition) == (char)sep && !isInQuotes) {
      def token = input.substring(startPosition, currentPosition).trim();
      if (!token.equals("")) { tokens.add(token); }
      startPosition = currentPosition + 1;
    }
  }
  def lastToken = input.substring(startPosition);
  if (!lastToken.equals(sep) && !lastToken.equals("")) { tokens.add(lastToken.trim()); }
  return tokens;
}
// ... builds HashMap from key=value pairs
```

---

## Design Decisions

### 1. Compile-time transpilation (not runtime interpretation)

**Decision:** Codegen emits native Rust code directly. No `painless_exec` calls remain.

**Rationale:** The entire point of this project is native Rust performance.
Runtime interpretation would negate the performance gains. All 43 scripts are
known at codegen time — there are no dynamically-constructed scripts.

### 2. Reuse ANTLR4 parser (already in dfe-codegen)

**Decision:** Use the existing ANTLR4 parser at `crates/dfe-codegen/src/painless/parser/`.

**Rationale:**
- Already checked in and compiling (13,826 lines of generated Rust)
- `antlr-rust` 0.3.0-beta already in `Cargo.toml`
- Painless grammar is frozen — Elastic hasn't changed it
- Parser is codegen-only (never runs at event processing time)
- `Script::parse()` entry point already works
- Writing a winnow replacement would be weeks of effort for zero runtime benefit
- Risk mitigation: generated parser code compiles standalone if crate disappears

### 3. Thin IR with string-based Rust emission

**Decision:** Build a simple intermediate representation (IR), then render to Rust source strings.

**Rationale:** Direct string emission (like predecessor) is workable but fragile.
A thin IR lets us separate "what does this Painless code mean" from "how do we
write it in Rust". The IR is intentionally minimal — not a general AST, just
what's needed for these 43 scripts.

### 4. All variables as `serde_json::Value`

**Decision:** Painless `def x = ...` becomes `let mut x: Value = ...` in Rust.

**Rationale:** Painless is dynamically typed. Type inference would be complex
and error-prone for marginal benefit. `Value` operations are already in the
hot path via the Event API. The performance cost is negligible compared to
the 10-20x gains from eliminating VRL overhead.

### 5. Params inlined at codegen time

**Decision:** Static params from pipeline YAML are embedded directly in
generated Rust code. Simple scalars inline directly; complex params (maps,
arrays) use `lazy_static!` blocks.

**Rationale:** Params are known at codegen time from the pipeline YAML.
No need for runtime injection — keeps the generated code self-contained.

### 6. Recursive functions as Rust helper functions

**Decision:** Painless function definitions emit as standalone Rust functions
above the transform method. Common patterns (e.g., recursive null/empty pruning
used by okta, cisco_nexus, fortinet) become shared runtime helpers.

### 7. Graceful fallback for unsupported scripts

**Decision:** If transpilation fails for a script, fall back to the existing
`painless_exec()` no-op with a compile-time warning. This allows incremental
rollout — transpile what we can, flag what we can't.

---

## Architecture

### Pipeline

```
ANTLR Parse Tree (existing parser)
        |
    PainlessVisitor (walks tree, builds IR)
        |
    PainlessIR (enum-based, Painless-specific)
        |
    RustEmitter (IR → Rust source string)
        |
    Emitted into transform body (replaces painless_exec)
```

### IR Types

```rust
enum PainlessExpr {
    CtxAccess { path: Vec<PathSegment> },           // ctx.foo.bar
    CtxAssign { path: Vec<PathSegment>, value },     // ctx.foo = val
    ParamAccess { path: Vec<String> },               // params.key
    LocalVar { name: String },                       // def x
    Literal { value: Value },                        // "str", 42, true, null
    BinaryOp { left, op, right },                    // a + b, a && b
    UnaryOp { op, operand },                         // !x, -x
    MethodCall { receiver, method, args },            // s.replace(a, b)
    StaticCall { class, method, args },               // Long.parseLong(s)
    InstanceOf { expr, type_name },                   // x instanceof Map
    Cast { type_name, expr },                         // (int) x
    NewHashMap, NewArrayList,                         // new HashMap()
    MapLiteral { entries }, ArrayLiteral { elements },
    Lambda { params, body },                          // v -> drop(v)
    FunctionCall { name, args },                      // local function call
    Regex { pattern },                                // /pattern/
    NullSafeAccess { base, field },                   // x?.field
    BraceAccess { base, index },                      // x[key]
}

enum PainlessStmt {
    Block(Vec<PainlessStmt>),
    Expr(PainlessExpr),
    VarDecl { name, value },
    If { cond, then, else_ },
    ForC { init, cond, update, body },
    ForEach { var, iter, body },
    Return { value },
    FunctionDef { name, params, body },
}

enum PathSegment {
    Static(String),       // .field
    Dynamic(PainlessExpr), // [variable]
}
```

### Module Structure

```
crates/dfe-codegen/src/painless/
    mod.rs              — existing, add transpiler module
    parser/             — existing ANTLR4 parser (untouched)
    script.rs           — existing entry point (add transpile_to_rust method)
    ir.rs               — NEW: PainlessExpr, PainlessStmt, PathSegment
    visitor.rs          — NEW: ANTLR parse tree → IR
    emitter.rs          — NEW: IR → Rust source string
    params.rs           — NEW: YAML params → inlined Rust literals

crates/dfe-runtime/src/
    painless_helpers.rs  — NEW: runtime helpers (painless_truthy, painless_add, etc.)
    codegen_api.rs       — MODIFY: re-export helpers, keep painless_exec as fallback
    prelude.rs           — MODIFY: export new helpers
```

---

## Feature Mapping: Painless → Rust

### Field Access

| Painless | Rust |
|----------|------|
| `ctx.event.duration` | `event.get("event.duration").cloned().unwrap_or(Value::Null)` |
| `ctx.event.duration = val` | `event.set("event.duration", val)?;` |
| `ctx?.event?.duration` | `event.get("event.duration")` (returns `Option`) |
| `ctx['field']` | `event.get("field")` |
| `ctx.event.duration != null` | `event.has("event.duration")` |
| `ctx.remove('field')` | `event.remove("field");` |

### String Methods

| Painless | Rust |
|----------|------|
| `s.replace(a, b)` | `json!(s.as_str().unwrap_or("").replace(a_str, b_str))` |
| `s.toLowerCase()` | `json!(s.as_str().unwrap_or("").to_lowercase())` |
| `s.toUpperCase()` | `json!(s.as_str().unwrap_or("").to_uppercase())` |
| `s.trim()` | `json!(s.as_str().unwrap_or("").trim())` |
| `s.startsWith(p)` | `s.as_str().unwrap_or("").starts_with(p_str)` |
| `s.substring(a, b)` | `json!(&s.as_str().unwrap_or("")[a..b])` |
| `s.splitOnToken(t)` | `json!(s.as_str().unwrap_or("").split(t).collect::<Vec<_>>())` |
| `s.contains(sub)` | `s.as_str().unwrap_or("").contains(sub_str)` |
| `s.charAt(i)` | `json!(s.as_str().unwrap_or("").chars().nth(i).unwrap_or('\0').to_string())` |
| `s.length()` | `json!(s.as_str().unwrap_or("").len())` |
| `s.isEmpty()` | `s.as_str().map_or(true, \|s\| s.is_empty())` |
| `s.equals(other)` | `s == other` |
| `s.toString()` | `painless_to_string(&value)` |

### Map Methods

| Painless | Rust |
|----------|------|
| `new HashMap()` | `Value::Object(Map::new())` |
| `m.get(k)` | `m.as_object().and_then(\|o\| o.get(k)).cloned().unwrap_or(Value::Null)` |
| `m.put(k, v)` | `m.as_object_mut().map(\|o\| o.insert(k.into(), v))` |
| `m.containsKey(k)` | `m.as_object().map_or(false, \|o\| o.contains_key(k))` |
| `m.keySet()` | `json!(m.as_object().map_or(vec![], \|o\| o.keys().collect::<Vec<_>>()))` |
| `m.entrySet()` | Iterate as `(key, value)` pairs |
| `m.remove(k)` | `m.as_object_mut().and_then(\|o\| o.remove(k))` |
| `m.size()` | `json!(m.as_object().map_or(0, \|o\| o.len()))` |

### List Methods

| Painless | Rust |
|----------|------|
| `new ArrayList()` | `Value::Array(vec![])` |
| `l.add(v)` | `l.as_array_mut().map(\|a\| a.push(v))` |
| `l.get(i)` | `l.as_array().and_then(\|a\| a.get(i)).cloned().unwrap_or(Value::Null)` |
| `l.size()` / `l.length` | `json!(l.as_array().map_or(0, \|a\| a.len()))` |
| `l.contains(v)` | `l.as_array().map_or(false, \|a\| a.contains(&v))` |
| `l.removeIf(lambda)` | `l.as_array_mut().map(\|a\| a.retain(\|item\| !predicate(item)))` |

### Type Checks

| Painless | Rust |
|----------|------|
| `x instanceof Map` | `x.is_object()` |
| `x instanceof List` | `x.is_array()` |
| `x instanceof String` | `x.is_string()` |
| `x instanceof long` | `x.is_i64() \|\| x.is_u64()` |
| `(int) x` | `painless_to_i64(&x)` |
| `Long.parseLong(s)` | `json!(s.as_str().unwrap_or("0").parse::<i64>().unwrap_or(0))` |

### Control Flow

| Painless | Rust |
|----------|------|
| `if (cond) { ... }` | `if painless_truthy(&cond) { ... }` |
| `for (int i=0; i<n; i++)` | `let mut i = 0i64; while i < n { ... i += 1; }` |
| `for (x : list)` | `if let Some(arr) = iter_val.as_array() { for x in arr { ... } }` |
| `return;` (top-level) | `return Ok(TransformResult::Keep);` |
| `return value;` (in function) | `return value;` |

### Arithmetic & Bitwise

| Painless | Rust |
|----------|------|
| `a + b` | `painless_add(&a, &b)` (handles int+int, string concat) |
| `a * b` | `painless_mul(&a, &b)` |
| `a & b` | `json!(a.as_i64().unwrap_or(0) & b.as_i64().unwrap_or(0))` |
| `a >> b` | `json!(a.as_i64().unwrap_or(0) >> b.as_i64().unwrap_or(0))` |

### Regex

| Painless | Rust |
|----------|------|
| `/pattern/` | `Regex::new("pattern")` (in `OnceLock` or `lazy_static!`) |
| `pat.matcher(s).replaceAll(r)` | `re.replace_all(s, r).to_string()` |

---

## Runtime Helpers (dfe-runtime)

Small set of functions to bridge Painless dynamic typing to Rust `Value`:

```rust
/// Painless truthiness: null/false/0/"" → false, everything else → true
pub fn painless_truthy(v: &Value) -> bool;

/// Painless addition: string concat if either is string, else numeric add
pub fn painless_add(a: &Value, b: &Value) -> Value;

/// Painless multiply/subtract/divide
pub fn painless_mul(a: &Value, b: &Value) -> Value;
pub fn painless_sub(a: &Value, b: &Value) -> Value;
pub fn painless_div(a: &Value, b: &Value) -> Value;

/// Convert to i64 (from string, float, or int)
pub fn painless_to_i64(v: &Value) -> i64;

/// Convert to string (handles all Value types)
pub fn painless_to_string(v: &Value) -> String;

/// Painless equality (null-safe, type-coercing)
pub fn painless_eq(a: &Value, b: &Value) -> bool;

/// Recursive removal of null/empty values (used by okta, cisco_nexus, fortinet)
pub fn painless_drop_empty(v: &mut Value) -> bool;
```

---

## Architectural Constraint: Raw Message Feed Reusability

**Requirement:** The transpiler must emit message-parsing Painless scripts as
standalone reusable functions, not just inline transform code. This enables a
future "raw message feed" mode where the same parsers work against raw
syslog/event sources (e.g., syslog receiver → message → same parsing as Beats).

**Why this matters:** Most Beats modules are thin transport wrappers — the
real parsing work happens in the ingest pipelines (Painless scripts, grok
patterns). If we emit parsers as reusable units, the same code handles both
Beats-fed and raw-fed sources.

**Design implication for emitter:**
- Painless scripts that parse `ctx.message`, `ctx.description`, or similar
  raw input fields → emit as standalone functions in a `parsers` module
- Field restructuring scripts (rename, reshape nested structures) → emit
  inline in the transform method
- The emitter needs a flag/heuristic to distinguish the two categories
- Standalone parser functions take `&str` input and return structured `Value`
  output, callable from both the transform pipeline and a future raw-feed
  entrypoint

**Target architecture:**

```
dfe-parsers (standalone crate)
    Pure parsing functions from Painless scripts.
    Takes &str / Value input, returns structured output.
    No knowledge of Beats, Elastic Agent, or transport.
    Usable by ANY DFE Rust project.
        |
dfe-transforms (thin layer)
    Handles Beats/Agent-specific envelope:
    field naming, ECS mapping, metadata.
    Calls dfe-parsers for actual message parsing.
        |
    Same parser, different wrappers:
    raw syslog, Beats, Elastic Agent — all use dfe-parsers.
```

**Pinned scope item in TODO.md.**

---

## Implementation Phases

### Phase A: Foundation — simple scripts (~20 scripts)

1. Define IR types (`ir.rs`)
2. Implement runtime helpers (`painless_helpers.rs`)
3. Build emitter skeleton (`emitter.rs`)
4. Build params module (`params.rs`)
5. Wire visitor for: field access/assign, variables, literals, arithmetic,
   if/else, null checks, simple string methods, params lookups

**Target scripts:** Duration conversions, field replacements, simple conditionals,
params-based enum mappings.

### Phase B: Collections and iteration (~15 scripts)

1. Map/List creation, get/put/add/contains/remove
2. For loops (C-style and for-each)
3. entrySet iteration and removeIf with lambda
4. instanceof checks
5. String split/join operations

**Target scripts:** O365 Parameters/ExtendedProperties restructuring, Azure
claims key rewriting, CrowdStrike tag extraction, email splitting.

### Phase C: Functions and complex logic (~10 scripts)

1. Local function definitions (emit as Rust helper functions)
2. Recursion (keysToSnakeCase, drop/handleMap/handleList)
3. Regex operations (literals, matcher().replaceAll())
4. Bitwise operations
5. Complex scripts (fortinet splitUnquoted, crowdstrike convertToUnix)

### Phase D: Integration

1. Modify `emit_script()` in `processor.rs` to call transpiler
2. Graceful fallback for any scripts that fail to transpile
3. Regenerate all 53 transform modules
4. Run integration tests

---

## Golden Test Data

7 commented-out test cases exist in `crates/dfe-codegen/src/pipeline/processors/script.rs`
(lines 170-584) with known Painless inputs and expected outputs:

1. Simple duration multiplication with conditional
2. Params-based multiplication
3. `entrySet()` iteration with conditional merge
4. Recursive `keysToSnakeCase` with nested maps/lists
5. Category-to-event-type mapping with HashMap + forEach
6. Bitwise label extraction with params map
7. Duration + ZonedDateTime arithmetic with params
8. Params array assignment

These provide ready-made validation for the transpiler.

---

## Key Files

| Purpose | Path |
|---------|------|
| ANTLR4 parser (untouched) | `crates/dfe-codegen/src/painless/parser/` |
| Parse entry point | `crates/dfe-codegen/src/painless/script.rs` |
| Existing VRL transpiler (reference only) | `crates/dfe-codegen/src/painless/transpiler.rs` |
| Predecessor transpiler (reference) | `/projects/elastic_to_vrl/painless/src/painless/transpiler.rs` |
| Codegen integration point | `crates/dfe-codegen/src/codegen/processor.rs:1236` |
| Runtime API | `crates/dfe-runtime/src/codegen_api.rs` |
| Script processor struct | `crates/dfe-codegen/src/pipeline/processors/script.rs` |
| Pipeline YAMLs | `tests/pipelines/` |
| Generated transforms | `crates/dfe-transforms/src/filebeat/` |

---

## References

- Predecessor transpiler: `/projects/elastic_to_vrl/painless/src/painless/transpiler.rs`
- Predecessor entry point: `/projects/elastic_to_vrl/painless/src/painless/script.rs`
- Predecessor visitor trait: `/projects/elastic_to_vrl/painless/src/painless/parser/painlessparservisitor.rs`
- Elastic Painless docs: https://www.elastic.co/docs/reference/scripting-languages/painless/painless
- Pipeline YAMLs: `/projects/dfe-transform-elastic/tests/pipelines/`
- Generated transforms: `/projects/dfe-transform-elastic/crates/dfe-transforms/src/filebeat/`
