# Homun Language Reference

> A lightweight scripting language designed for ECS game engine client-side logic, in rust.
> Compiles / transpiles directly to Rust. Designed to be embeddable, minimal, and expressive.

---

## Overview

Homun is a statically-typed, expression-oriented scripting language that targets the Rust compiler as its backend. It is designed to run on the **client side** of a Rust-based game engine, providing a safe, readable, and concise scripting surface without requiring game developers to write raw Rust. Every valid Homun program maps 1-to-1 to valid Rust code, which means you get Rust's performance and safety guarantees at runtime, while writing code that feels closer to Python or Go in day-to-day use.

The philosophy of Homun is radical simplicity:

- No classes, no interfaces, no inheritance — only structs and enums.
- No functions as a distinct construct — everything callable is a lambda.
- No imports — the engine exposes its API through the environment.
- No semicolons — newlines are enough.
- No mutable/immutable split in the declaration syntax — assignments are clear and uniform.
- No `=` operator at all — `:=` binds, `==` compares. There is no ambiguity between assignment and equality.
- All collections share the `@` prefix — lists `@[]`, dicts `@{}`, sets `@()` — visually consistent and parser-friendly.
- Strict naming conventions: variables and lambdas are `snake_case`, structs and enums are recommended as `PascalCase`.
- 1-based indexing throughout, matching the intuition of designers and artists who may not be programmers.

---

## Design Goals

| Goal | Decision |
|---|---|
| Familiar to Python/Go developers | `:=` assignment, Python keywords like `and`, `or`, `in`, `not` |
| No ceremony around functions | All callables are lambdas `\|\|{...}` |
| Safe mapping to Rust | No raw pointers, no unsafe constructs |
| Readable game logic | `if () do {} else {}`, `match` on enums |
| Minimal syntax noise | No `;`, no `mut`, no `import`, no bare `=` |
| Unambiguous equality | Only `==` for comparison — `:=` is the sole assignment form |
| Consistent collection syntax | All collections prefixed with `@`: `@[]` `@{}` `@()` |
| Clear naming at a glance | Variables/lambdas `snake_case`, types `PascalCase` (recommended) |
| Designer-friendly indexing | 1-based, inclusive slicing |

---

## Variable Assignment

Homun uses `:=` for all variable bindings, inspired by Go. There is no `var`, `let`, or `const` keyword. There is also no `mut` keyword — mutability rules are handled at the Rust transpilation layer, keeping the scripting surface clean.

```
x := 10
name := "hero"
speed := float(3.14)
active := true
```

Types are inferred automatically from the right-hand side, similar to `auto` in C++. You may also be explicit by wrapping the value in a type constructor:

```
hp := int(100)
ratio := float(0.5)
label := str("player_one")
```

If you assign a new value to an existing binding, it simply rebinds the name. There is no distinction between declaration and reassignment in the syntax — the compiler resolves this contextually.

---

## Naming Conventions

Homun enforces and recommends strict naming conventions to keep code readable across a team of engineers and non-engineers alike.

### Variables and Lambdas — enforced `snake_case` (all lowercase)

All variable names and lambda names **must** be `lower_case` or `snake_case`. Uppercase letters are **not permitted** in variable names. The compiler will reject names like `mySpeed`, `PlayerHP`, or `X`. This is a hard rule, not a style suggestion.

```
-- VALID
player_hp := 100
move_speed := 3.5
on_death := || -> none { respawn() }
base_attack_damage := 25

-- INVALID — compiler error
playerHp := 100       -- camelCase not allowed
MoveSpeed := 3.5      -- uppercase start not allowed
X := 10               -- single uppercase letter not allowed
```

### Structs and Enums — recommended `PascalCase`

Type definitions (structs and enums) are recommended to use `PascalCase`. This is not enforced by the compiler but is the strong convention for all Homun code and makes it immediately obvious when a name refers to a type versus a value:

```
-- Recommended: PascalCase for types
PlayerState := struct { hp: int, alive: bool }
Direction := enum { North, South, East, West }
WeaponKind := enum { Sword(int), Bow(int), Staff }

-- Allowed but discouraged: lowercase type names
player_state := struct { hp: int }    -- works, but confusing
```

The reason this asymmetry exists is intentional: types exist at compile time and are structural definitions, while values exist at runtime. Keeping their naming visually distinct helps tools, LLMs, and human readers parse code intent without requiring a type-inference pass.

---

## Operators and Equality

Homun has **no bare `=` operator**. This eliminates an entire class of bugs common in C-family languages where `=` and `==` are accidentally swapped.

| Operator | Meaning |
|---|---|
| `:=` | Bind a name to a value (declaration or rebinding) |
| `==` | Equality comparison, returns `bool` |
| `!=` | Inequality comparison, returns `bool` |
| `<`, `>`, `<=`, `>=` | Numeric comparison |
| `and`, `or`, `not` | Boolean logic (Python-style keywords) |
| `+`, `-`, `*`, `/`, `%` | Arithmetic |

There is no `=` for anything. Using a bare `=` is a syntax error. This means that if you ever see `:=`, you know it is a binding. If you ever see `==`, you know it is a comparison. No ambiguity exists anywhere in the language.

```
x := 10           -- bind x to 10
y := x == 10      -- y is true (bool)
z := x != 5       -- z is true (bool)

if (x == 10) do {
  print("ten")
}
```

---

## Primitive Types

| Type | Example |
|---|---|
| `int` | `42`, `int(42)` |
| `float` | `3.14`, `float(3.14)` |
| `bool` | `true`, `false` |
| `str` | `"hello"` |
| `none` | used as a return type hint only |

---

## String Interpolation

Strings support inline variable interpolation using `${}` syntax:

```
name := "Aria"
greeting := "Hello, ${name}! Welcome to the dungeon."
level := 5
msg := "You are level ${level}."
```

Any expression can go inside `${}`:

```
damage := base * multiplier
log := "Dealt ${damage * 2} total damage after crit."
```

---

## Lambdas (All Callables)

There are no named functions in Homun. Every callable value is a lambda. Lambdas are first-class values and can be assigned to names, passed as arguments, or returned from other lambdas.

### Basic Lambda

```
greet := |name| { "Hello ${name}" }
```

The last expression in a lambda body is implicitly returned. No `return` keyword is needed.

### Lambda with No Arguments

```
tick := || { update_physics() }
```

### Lambda with Type Hints

You may optionally annotate the return type. Use `-> Type` after the parameter list:

```
add := |a, b| -> int { a + b }
```

If the lambda explicitly returns nothing, use `-> none`:

```
log_event := |msg| -> none { print(msg) }
```

If you want to be explicit that a lambda returns unit/void but don't care to annotate, you can write `-> ()`:

```
fire := || -> () { spawn_bullet(pos, dir) }
```

### Calling Lambdas

```
result := add(3, 5)
greet("player")
```

### Lambdas as Values

```
transform := |x| { x * 2 }
values := @[1, 2, 3, 4]
doubled := values.map(transform)
```

---

## Pipe Operator `.`

The `.` operator is the pipe/method-call syntax. It is syntactic sugar for passing the left-hand side as the first argument to the right-hand side:

```
x.map(f)        -- desugars to: map(x, f)
x.filter(pred)  -- desugars to: filter(x, pred)
x.reduce(acc)   -- desugars to: reduce(x, acc)
```

You can chain pipes naturally:

```
result := @[1, 2, 3, 4, 5]
  .filter(|x| { x > 2 })
  .map(|x| { x * 10 })
  .reduce(|a, b| { a + b })
```

---

## Collections

All collection literals are prefixed with `@`. This applies uniformly to lists, dicts, and sets — the shape of the bracket that follows tells you which kind. `@[]` is a list, `@{}` is a dict, `@()` is a set. This keeps the parser simple, makes collection usage visually obvious in dense game logic code, and means you never have to guess whether a bare `(...)` is a set, a tuple, or a grouping expression — it is always a grouping expression, and sets are always `@(...)`.

### Lists (Dynamic Arrays)

```
empty := @[]
items := @[1, 2, 3]
names := @["Alice", "Bob", "Charlie"]
```

List comprehension:

```
squares := @[x * x for x in upto(10)]
evens   := @[x for x in upto(20) if x % 2 == 0]
```

`upto(n)` generates a 1-based range from 1 to n inclusive.

### Dicts (Hash Maps)

```
empty_map := @{}
scores    := @{"alice": 100, "bob": 80}
```

Dict comprehension:

```
inverted := @{v: k for k, v in scores}
squared  := @{x: x*x for x in upto(5)}
```

### Sets

Sets use the `@()` prefix — consistent with the `@` convention for all collection types. Unlike lists and dicts, sets are unordered and contain no duplicate values.

```
visited  := @(1, 3, 5, 7)
flags    := @("fire", "ice", "poison")
empty_s  := @()
```

Set comprehension:

```
unique_floors := @(x for x in floor_list)
```

Because all three collection types share the `@` prefix, the bracket shape tells you the kind: `@[]` is a list (ordered, indexed), `@{}` is a dict (key-value), and `@()` is a set (unordered, unique).

---

## Slicing and Indexing

Homun is **1-based**. The first element of any list is at index `1`, not `0`. This applies to all slicing and indexing operations.

### Single Index

```
first := items[1]
third := items[3]
```

### Slicing `[start..end, step?]`

Slices use `..` notation. The range is **inclusive on both ends**. The optional step follows a comma.

```
-- Items 1 through 3
sub := items[1..3]       -- [items[1], items[2], items[3]]

-- Items 3 down to 1 (reverse)
rev := items[3..1, -1]   -- [items[3], items[2], items[1]]

-- Every other item from 1 to 10
alt := items[1..10, 2]   -- [items[1], items[3], items[5], ...]
```

Because the language is 1-based and slices are inclusive, `[3..1]` yields three elements: the 3rd, 2nd, and 1st — in that order.

```
x := @[10, 20, 30, 40, 50]
x[1..3]       -- [10, 20, 30]
x[3..1, -1]   -- [30, 20, 10]
x[2..5]       -- [20, 30, 40, 50]
```

---

## Control Flow

### `if` / `else`

The `if` syntax always uses `do` before the body to distinguish the condition from the block without relying on indentation:

```
if (hp <= 0) do {
  die()
} else {
  recover()
}
```

One-liner form:

```
if (x > 10) do { print("big") } else { print("small") }
```

There is no `elif` — chain `else if` instead:

```
if (score > 90) do {
  grade := "A"
} else if (score > 75) do {
  grade := "B"
} else {
  grade := "C"
}
```

Boolean operators use Python-style keywords:

```
if (alive and not frozen) do { move() }
if (on_fire or in_water) do { apply_effect() }
```

---

## Loops

### `for` over a range

```
for i in upto(10) do {
  print("Step ${i}")
}
```

### `for` over a list

```
for item in inventory do {
  use(item)
}
```

### `while`

```
while (enemies_remaining > 0) do {
  attack_nearest()
}
```

---

## Structs

Homun has no classes. Data is organized using structs. Behavior is attached by assigning lambdas that accept the struct as a parameter — the engine binds these at the Rust level.

```
Player := struct {
  name: str
  hp: int
  speed: float
}

create_player := |n, h, s| -> Player {
  Player { name: n, hp: h, speed: s }
}

p := create_player("Aria", 100, 3.5)
print(p.name)
```

Structs are value types. Assigning a struct copies it unless passed through a reference wrapper provided by the engine.

---

## Enums and Match

Enums define a closed set of named variants, optionally carrying data.

```
Direction := enum {
  North
  South
  East
  West
}

Element := enum {
  Fire(int)
  Ice(int)
  Neutral
}
```

Pattern matching with `match`:

```
result := match dir {
  Direction.North => move(0, 1)
  Direction.South => move(0, -1)
  Direction.East  => move(1, 0)
  Direction.West  => move(-1, 0)
}
```

Matching with bound data:

```
dmg := match element {
  Element.Fire(power)    => power * 2
  Element.Ice(power)     => power * 1.5
  Element.Neutral        => 0
}
```

`match` is an expression — its result can be directly assigned.

---

## Built-in Utilities

These are provided by the engine runtime environment:

| Name | Description |
|---|---|
| `upto(n)` | 1-based range from 1 to n inclusive |
| `print(x)` | Output to engine console |
| `len(col)` | Length of a list, dict, or set |
| `keys(d)` | Keys of a dict as a list |
| `values(d)` | Values of a dict as a list |
| `zip(a, b)` | Pair two lists element-wise |
| `map(col, f)` | Apply f to each element (also via pipe `.map(f)`) |
| `filter(col, f)` | Keep elements where f returns true |
| `reduce(col, f)` | Fold a list using a binary lambda |
| `floor(x)` | Floor of a float |
| `ceil(x)` | Ceiling of a float |
| `clamp(x, lo, hi)` | Clamp x to [lo, hi] |
| `abs(x)` | Absolute value |

---

## Rust Transpilation Notes

Homun compiles to idiomatic Rust. The following mappings are used:

| Homun | Rust |
|---|---|
| `:=` binding | `let` (or `let mut` when the compiler detects reassignment) |
| `==` equality | `==` (no bare `=` exists in Homun) |
| Lambda `\|\|{...}` | Rust closure or `fn` depending on context |
| `@[...]` list | `Vec<T>` |
| `@{...}` dict | `HashMap<K, V>` |
| `@(...)` set | `HashSet<T>` |
| Struct | `struct` with `#[derive(Clone)]` |
| Enum | `enum` |
| `match` | `match` |
| `.` pipe | function call with first arg |
| String `${}` | `format!()` macro |
| `and`, `or`, `not` | `&&`, `\|\|`, `!` |
| `none` return | `()` unit type |
| Variable name `snake_case` | Rust `snake_case` (matches Rust convention natively) |
| Type name `PascalCase` | Rust `PascalCase` struct/enum (matches Rust convention natively) |
| 1-based slice | transpiled with index offset + bounds check |

---

## Quick Reference Card

```
-- Variable (snake_case only)
x := 42
player_name := "hero"

-- Lambda (snake_case only)
double := |x| { x * 2 }
greet  := |name| -> str { "Hi ${name}" }
tick   := || -> none { update() }

-- Equality (== only, no bare =)
x == 42          -- true
x != 0           -- true

-- Call / Pipe
double(5)
@[1,2,3].map(double)

-- If
if (x > 0) do { print("pos") } else { print("neg") }

-- For
for i in upto(5) do { print(i) }

-- List / Dict / Set  (all use @ prefix)
items  := @[1, 2, 3]
scores := @{"a": 10, "b": 20}
flags  := @(1, 2, 3)

-- Slice (1-based, inclusive)
items[1..3]       -- first three
items[3..1, -1]   -- last three, reversed

-- Struct (PascalCase recommended)
Pos := struct { x: float, y: float }
p := Pos { x: 1.0, y: 2.0 }

-- Enum + match (PascalCase recommended)
Dir := enum { Up, Down }
match dir {
  Dir.Up   => move_up()
  Dir.Down => move_down()
}
```

---

## License

Homun is part of the game engine runtime. See `LICENSE` for terms.