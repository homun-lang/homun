# homunc — Homun → Rust Compiler (Haskell)

A text-to-text transpiler that compiles the **Homun** scripting language into **Rust** source code. Written entirely in Haskell with zero external dependencies beyond `base` and `containers`.

---

## Architecture

```
Source (.hom)
    │
    ▼
┌─────────┐
│  Lexer  │  src/Lexer.hs   — tokenises Homun source into [Token]
└────┬────┘
     │ [Token]
     ▼
┌─────────┐
│  Parser │  src/Parser.hs  — recursive-descent Pratt parser → AST
└────┬────┘
     │ Program (AST)
     ▼
┌──────────┐
│   Sema   │  src/Sema.hs   — semantic analysis:
└────┬─────┘    • snake_case enforcement
     │           • recursion detection & marking
     │ Program   • mutual recursion error
     ▼           • undefined reference check
┌──────────┐
│ Codegen  │  src/Codegen.hs — walks AST, emits Rust text
└────┬─────┘
     │ Rust source (.rs)
     ▼
   rustc
```

---

## Build

```bash
# Requires GHC >= 9.2 and cabal >= 3.0
cd homun-compiler
cabal build
```

Or compile directly with GHC:

```bash
cd homun-compiler/src
ghc -O2 Main.hs Lexer.hs AST.hs Parser.hs Sema.hs Codegen.hs -o homunc
```

---

## Usage

```bash
# Compile to stdout
./homunc examples/quicksort.hom

# Compile to file
./homunc examples/fizzbuzz.hom -o output.rs

# Then compile the Rust
rustc output.rs -o program
```

---

## Language Coverage

| Homun Feature | Rust Output |
|---|---|
| `x := 10` | `let mut x = 10;` |
| `fn := (a, b) -> { a + b }` | `pub fn fn<T,U>(a: T, b: U) -> _ { a + b }` |
| `fn := (a: int) -> int { ... }` | `pub fn fn(a: i32) -> i32 { ... }` |
| `fib := (n) -> { fib(n-1) }` | recursive fn via `fn fib<T>(...) -> _` |
| `@[1,2,3]` | `vec![1, 2, 3]` |
| `@{"a": 1}` | `HashMap::from([("a", 1)])` |
| `@("x","y")` | `HashSet::from(["x","y"])` |
| `list \| filter(f) \| map(g)` | `map(filter(list, f), g)` |
| `if (c) do { x } else { y }` | `if c { x } else { y }` |
| `match x { pat => body }` | `match x { pat => body, }` |
| `for i in range(n) do { }` | `for i in (0..n) { }` |
| `break => value` | `break value` |
| `"Hello ${name}"` | `format!("Hello {}", name)` |
| `Player := struct { hp: int }` | `pub struct Player { pub hp: i32, }` |
| `Direction := enum { North }` | `pub enum Direction { North, }` |
| `load_ron("f.ron") as Map` | `ron::from_str::<Map>(...)` |
| `save_ron(data, "f.ron")` | `std::fs::write(...)` |

---

## Runtime Helpers (auto-emitted preamble)

The compiler prepends a small Rust preamble to every output file:

| Helper | Description |
|---|---|
| `homun_slice(v, start, end, step)` | Python-style negative-index slicing |
| `len(c)` | Works on Vec, HashMap, HashSet, String |
| `range(n)` / `range2(s,e)` / `range3(s,e,step)` | Range iterators |
| `filter(v, f)` | Functional filter over Vec |
| `map(v, f)` | Functional map over Vec |
| `reduce(v, f)` | Functional reduce over Vec |
| `str_of(x)` | Convert anything Display to String |

---

## Semantic Checks

The `Sema` pass enforces Homun's rules **before** codegen:

1. **snake_case** — all variable and lambda names must be `snake_case`
2. **Recursion detection** — auto-detects self-recursive lambdas
3. **Mutual recursion error** — two functions calling each other → compile error
4. **Undefined references** — references to names not yet defined → compile error

---

## Homun Type → Rust Type Mapping

| Homun | Rust |
|---|---|
| `int` | `i32` |
| `float` | `f32` |
| `bool` | `bool` |
| `str` | `String` |
| `none` | `Option<_>` |
| `@[T]` | `Vec<T>` |
| `@{K:V}` | `HashMap<K, V>` |
| `@(T)` | `HashSet<T>` |
| `_` (void) | `()` |
| unknown | `_` (let Rust infer) |

---

## File Structure

```
homun-compiler/
├── homunc.cabal
├── README.md
└── src/
    ├── Main.hs      — CLI entry point, pipeline orchestration, Rust preamble
    ├── Lexer.hs     — tokeniser
    ├── AST.hs       — abstract syntax tree types
    ├── Parser.hs    — recursive-descent parser
    ├── Sema.hs      — semantic analysis
    └── Codegen.hs   — Rust code emitter
examples/
    ├── quicksort.hom
    └── fizzbuzz.hom
```

---

## TODO: Multi-file `use` (Text Inclusion)

Support `use` for other `.hom` files via textual inclusion (like C `#include`).

### Behavior

```
// main.hom
use math        // finds math.hom, compiles it, inlines the Rust output
use utils       // finds utils.hom, compiles it, inlines the Rust output
```

- `use foo` → compiler looks for `foo.hom` in the same directory
- If found: compile `foo.hom` → inline the resulting Rust into the output
- If not found: fall through to existing behavior (`use foo;` as Rust import)
- `use std` remains special-cased → `include!("std.rs");`

### Dependency Resolution

The compiler must resolve the dependency graph before compilation:

1. **Circular dependency detection** — `a.hom use b.hom, b.hom use a.hom` → compile error
2. **Include guard (deduplicate)** — if `a.hom` uses `b.hom` and `b.hom` uses `c.hom`, the output contains only one copy of `c.hom` (like C header guards / `#pragma once`)
3. **Topological sort** — compile in dependency order (leaves first)

### Algorithm

```
resolve := (file: str, visited: @(str), emitted: @(str)) -> str {
  if (file in visited and not file in emitted) do {
    break => "ERROR: circular dependency on ${file}"
  }
  if (file in emitted) do { break => "" }
  visited := visited + @(file)
  output := ""
  for dep in parse_uses(file) do {
    if (exists("${dep}.hom")) do {
      output := output + resolve("${dep}.hom", visited, emitted)
    }
  }
  emitted := emitted + @(file)
  output := output + codegen(file)
  output
}
```

### Changes Required

| Component | Change |
|---|---|
| **Main.hs** | Add file resolver: check if `foo.hom` exists for each `use foo` |
| **Main.hs** | Implement dependency graph traversal with cycle detection + dedup |
| **Codegen.hs** | `use foo` when `foo.hom` exists → inline compiled output instead of `use foo;` |
| **Sema.hs** | Collect exported names from included files to avoid false "undefined" errors |
