# mm2-stdlib

`mm2-stdlib` is a helper library for [MORK](https://github.com/iCog-Labs-Dev/MORK). It adds relational operators, boolean operations, and list manipulations (`fold`, `car`, `cons`, `length`, `first`, `append`, `sort`, etc.) without bloating the core MORK kernel. Since the main branch of MORK does not include these operations natively, integrating `mm2-stdlib` provides them in a modular way.

---

## Integration

To use `mm2-stdlib` with your MORK fork, apply the following four changes.

### Step 1: Add `bool` deserialization to MORK

The upstream MORK library does not implement `DeserializableExpr` for `bool`. Open `expr/src/macros.rs` in your MORK project and add the following block after the `impl DeserializableExpr for &str { ... }` block:

```rust
impl DeserializableExpr for bool {
    #[inline(always)]
    fn advanced(e: Expr) -> usize {
        unsafe {
            let Tag::SymbolSize(arity) = byte_item(*e.ptr) else { panic!("wrong symbol for bool") };
            1usize + (arity as usize)
        }
    }

    #[inline(always)]
    fn check(e: Expr) -> bool {
        unsafe {
            let Tag::SymbolSize(arity) = byte_item(*e.ptr) else { return false; };
            let s = std::ptr::slice_from_raw_parts(e.ptr.add(1), arity as _);
            let bytes = s.as_ref().unwrap();
            bytes == b"true" || bytes == b"false"
        }
    }

    #[inline(always)]
    fn deserialize_unchecked(e: Expr) -> Self {
        unsafe {
            let Tag::SymbolSize(arity) = byte_item(*e.ptr) else { unreachable!() };
            let s = std::ptr::slice_from_raw_parts(e.ptr.add(1), arity as _);
            let bytes = s.as_ref().unwrap();
            bytes == b"true"
        }
    }
}
```

### Step 2: Add `mm2-stdlib` as a dependency

Open `kernel/Cargo.toml` in your MORK workspace and add:

```toml
[dependencies]
mm2-stdlib = { git = "https://github.com/abnsol/mm2-stdlib" }
```

### Step 3: Register the library in the kernel

Open `kernel/src/sinks.rs` and make two changes:

1. Add the import at the top of the file:
```rust
use mm2_stdlib;
```

2. Inside the `PureSink` constructor, register `mm2-stdlib` alongside the built-in `pure` functions:
```rust
impl Sink for PureSink {
    fn new(e: Expr) -> Self {
        let mut scope = EvalScope::new();
        pure::register(&mut scope);
        mm2_stdlib::register(&mut scope);  // <-- add this line
        PureSink { e, unique: PathMap::new(), scope }
    }
    // ...
}
```

### Step 4: Unify the crate graph with `[patch]`

MORK's workspace already contains `eval`, `eval-ffi`, and `mork-expr` as path dependencies. When `mm2-stdlib` fetches the same crates via git, Cargo treats them as separate copies with incompatible types. Add a `[patch]` section in MORK's root `Cargo.toml` to redirect the git copies back to your local workspace versions:

```toml
[patch."https://github.com/trueagi-io/MORK"]
eval = { path = "experiments/eval" }
eval-ffi = { path = "experiments/eval-ffi" }
mork-expr = { path = "expr" }
```

For a complete working example, see the [`test-mm2stdlib`](https://github.com/abnsol/MORK/tree/test-mm2stdlib) branch on the author's fork.

---

## Features

| MM2 function name | Arguments | Returns | Description |
|---|---|---|---|
| `bool_from_string` | `(String)` | `Bool` | Parse a boolean from a string. |
| `and_bool`, `or_bool`, `xor_bool`, `implies_bool` | `(Bool, Bool)` | `Bool` | Logical operators. |
| `u8_from_string`, `u16_from_string`, `u32_from_string`, `u64_from_string`, `u128_from_string` | `(String)` | `u*` | Parse unsigned integer from a string. |  
| `<op>_<type>` | `(Type, Type)` | `Bool` | Numeric comparisons: `lt`, `gt`, `le`, `ge`, `eq`, `ne` for `u8`–`u128`, `i8`–`i128`, `f32`, `f64`. |
| `length`, `size-atom` | `(List)` | `Number` | Number of elements in a list. |
| `car`, `car-atom`, `first`, `first-from-pair` | `(List)` | `Expr` | First element of a list. |
| `cdr`, `cdr-atom` | `(List)` | `List` | Rest of the list after the first element. |
| `second-from-pair` | `(List)` | `Expr` | Second element of a list. |
| `last` | `(List)` | `Expr` | Last element of a list. |
| `cons` | `(Expr, List)` | `List` | Prepend an element to a list. |
| `decons` | `(List)` | `(Expr, List)` | Split list into head and tail. |
| `append` | `(List, List)` | `List` | Concatenate two lists. |
| `reverse` | `(List)` | `List` | Reverse a list. |
| `index-atom` | `(Number, List)` | `Expr` | Element at index (index first, then list). |
| `is-member` | `(Expr, List)` | `Bool` | Check membership. |
| `exclude-item` | `(Expr, List)` | `List` | Remove all instances of an element. |
| `unique-atom` | `(List)` | `List` | Remove duplicates. |
| `union-atom` | `(List, List)` | `List` | Set union. |
| `intersection-atom` | `(List, List)` | `List` | Set intersection. |
| `subtraction-atom` | `(List, List)` | `List` | Set subtraction. |
| `min-atom`, `max-atom` | `(List)` | `Expr` | Minimum / maximum numeric element. |
| `sort-math` | `(List)` | `List` | Numeric sort. |
| `sort-atom` | `(List)` | `List` | Lexicographic (byte) sort. |
| `foldl` | `(Func, Init, List)` | `Expr` | Left fold over a list. |
| `map-atom` | `(Op, Operand, List)` | `List` | Map an operation over a list (`self` operand uses element as both args for `+`/`*`). |
| `reduce` | `(Op, List)` | `Expr` | Reduce a list with an associative operation. |
| `assertEqual` | `(Expr, Expr)` or `(' (Expr Expr))` | `Bool` | Byte-level equality of any two expressions (bare or packed). |

---

## Test

Download the `.mm2` test files from the [`test/` directory](https://github.com/abnsol/mm2-stdlib/tree/main/test) and run them from your MORK workspace:

```bash
cargo run -p mork -- run path/to/and.mm2
```
