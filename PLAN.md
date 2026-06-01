# mm2-stdlib — Plan

## Goal

Extract relational operators, boolean operators, and list operations from MORK's
`kernel/src/pure.rs` into an external crate (`mm2-stdlib`) that MORK pulls as a
compile-time dependency.

The original numeric/bitwise/math functions stay in `pure.rs` — only the newly
added functions move out.

---

## What moves to mm2-stdlib

### From `kernel/src/pure.rs`

| What | Lines (approx) | How generated |
|---|---|---|
| Relational ops (`lt_u8` … `ge_f64`) | 754–825 | `op!(relational_binary …)` |
| Boolean ops (`bool_from_string`, `and_bool`, `or_bool`, `xor_bool`, `implies_bool`) | 828–832 | `op!(bool from_string …)` + `op!(relational_binary …)` |
| List operations (`first`, `append`, `foldl`, `sort-atom`, …) | 1004–1472 | hand-written |

### From `kernel/src/`

| File | Reason |
|---|---|
| `list_helpers.rs` | Only used by the list operations above |

### From the `op!` macro (kernel/src/pure.rs:11–407)

Two macro arms must be **duplicated** into mm2-stdlib (~15 lines total):

- `(relational_binary …)` — lines 394–406
- `(bool from_string …)` — lines 99–115

The rest of the `op!` macro stays in `pure.rs`.

---

## What stays in MORK's `pure.rs`

- All other `op!` macro arms (unchanged)
- Numeric/bitwise functions for `u8`–`u128` (lines 409–753)
- Signed int + float math for `i8`–`i128`, `f32`, `f64` (lines 835–1070)
- Conversions, encoding, `ifnz`, `hash_expr` (lines 1072–1320)
- The `register()` function — but with relational/bool/list lines removed

---

## Changes in MORK

### `kernel/Cargo.toml`

Add dependency:

```toml
mm2-stdlib = { path = "../mm2-stdlib" }
```

### `kernel/src/sinks.rs`

Add a second register call in `PureSink::new()`:

```rust
fn new(e: Expr) -> Self {
    let mut scope = EvalScope::new();
    pure::register(&mut scope);         // original functions
    mm2_stdlib::register(&mut scope);   // relational/bool/list ops
    PureSink { e, unique: PathMap::new(), scope }
}
```

### `kernel/src/lib.rs`

Remove `pub mod list_helpers;`

### `kernel/src/pure.rs`

- Remove the relational operator definitions (lines 754–832)
- Remove all list operation definitions (lines ~1004–1472)
- Remove the `use crate::list_helpers` import
- Remove the corresponding `scope.add_func(…)` lines from `register()` (lines ~1880–1942)
- The file remains as the home for original numeric/bitwise/math functions

### Delete

- `kernel/src/list_helpers.rs` (moved to mm2-stdlib)

---

## mm2-stdlib crate structure

```
mm2-stdlib/
├── Cargo.toml
├── PLAN.md
└── src/
    ├── lib.rs              ← pub mod pure; pub mod list_helpers; pub fn register()
    ├── pure.rs             ← relational ops + boolean ops + list ops + register()
    └── list_helpers.rs     ← moved from kernel/src/
```

### `Cargo.toml`

```toml
[package]
name = "mm2-stdlib"
version = "0.1.0"
edition = "2024"

[dependencies]
log = "0.4"
eval-ffi = { path = "../MORK/experiments/eval-ffi" }
eval = { path = "../MORK/experiments/eval" }
mork-expr = { path = "../MORK/expr" }
```

### `src/lib.rs`

```rust
pub mod pure;
pub mod list_helpers;

pub fn register(scope: &mut eval::EvalScope) {
    pure::register(scope);
}
```

### `src/pure.rs`

Copied from MORK's `pure.rs` with only:

- Relational op definitions (the `op!(relational_binary …)` invocations)
- Boolean op definitions
- List operation function definitions (hand-written)
- A `register()` function that registers only these
- Duplicated `(relational_binary …)` and `(bool from_string …)` macro arms
- `use crate::list_helpers` instead of `use crate::list_helpers`

### `src/list_helpers.rs`

Identical copy of the current `kernel/src/list_helpers.rs`.

---

## Why this approach

**Compile-time dependency** — mm2-stdlib is a normal Cargo dependency linked at
build time. No runtime loading, no ABI concerns, no plugin infrastructure.
Zero overhead vs. having everything in one crate.

**Only the new functions move** — minimal churn to MORK's codebase. The existing
350+ numeric/bitwise/math functions stay exactly where they are.

**Both `register()` functions are called** — `PureSink` calls both, so all
functions (old and new) are available to MM2 at runtime with no conflicts.
