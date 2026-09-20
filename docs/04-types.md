# PickleScript — Type System Design

## Foundations

- **Static, sound, structurally-checked where helpful.** Every expression has
  a type known at compile time. Runtime type checks exist where soundness
  requires them (`is` / dynamic downcast), visible at the call site.
- **Values vs references** is decided by the type: `struct`/primitive =
  value; `class`/`string`/collections = reference.
- **Nullability is part of the type.** `T` is non-null. `T?` is `T` or
  `none`. There is no nullable-quirk class (no stringly nulls, no
  `NullPointerException`). Forcing `none` into a non-option type is a
  compile error.

## Primitive types

| Type     | Meaning                                  |
|----------|------------------------------------------|
| `bool`   | true/false                                |
| `byte`   | unsigned 8-bit                            |
| `i8`..`i64` | signed ints, default int literal      |
| `u8`..`u64`, `usize` | unsigned                     |
| `int`    | 64-bit signed (alias of `i64`)            |
| `float`  | 64-bit IEEE-754 (alias of `f64`)          |
| `f32`    | 32-bit float                              |
| `char`   | Unicode scalar value (32-bit)             |
| `string` | immutable UTF-8 string                    |

Numeric literal inference: integer literals adapt to the expected type
(`u16`, `byte`, ...); range errors are compile errors. Explicit suffixes
override inference.

## String and char model

The model is locked. A `string` is an immutable, contiguous string of **UTF-8
bytes**, and a `char` is a **Unicode scalar value** (a single code point,
never a surrogate half). Strings are *not* arrays of `char`:

- `s[i]` and `for (c in s)` address *bytes*, like Rust `s.as_bytes()[i]`:
  O(1), bounds-checked, and they yield the static type **`byte`** (a raw UTF-8
  byte, never a decoded scalar — `len("é")` is `2` and `"é"[0]` is byte `195`).
  This keeps indexing and iteration cheap and deterministic.
- `byte` is a real primitive but sits at the **`int` ABI**: comparisons and
  arithmetic are done as 64-bit ints (there is still no implicit `byte ->
  int` assignment widening). `byte` values compare with `int`s and with ASCII
  `char` literals; a non-ASCII `char` literal (e.g. `'é'`) has no `byte` value
  and is a compile error in a byte comparison, and a `byte` can never silently
  reach a `char` context.
- Converting a `char` to a `string` encodes the scalar as UTF-8; converting
  the other way always consumes exactly one scalar. `"{c}"`/`print(c)` on a
  `char` write the scalar's UTF-8 bytes, so `len("{'é'}")` is `2` and the
  copy loop `for (c in "é") { out += "{c}" }` round-trips — a `byte`
  interpolated as `"{b}"` writes that single byte.
- Scalar/byte *views* (`s.chars()`, `s.bytes()`) are reserved spellings for
  future explicit iteration and are not part of the primitive surface yet.

This is deliberate: pretending UTF-8 is an array of scalar values invites
expensive re-decoding and surprising `len()`. The Rust model (`str` /
`char` / `as_bytes`) is the reference design.

## Composite types

- **`struct`** — value type. Field-for-field equality, copied on
  assignment, allocated inline in its container (no GC pressure). Methods are
  allowed. Inheritance is not (use composition).

```
struct Vec2 {
    x: float = 0
    y: float = 0

    fn len2() -> float => x * x + y * y
}
```

- **`class`** — reference type, GC-managed. Identity semantics, shared by
  assignment, single inheritance.
- **`enum`** — closed sum type.

```
enum Status { Idle Running Dead }

enum Result {
    Ok(value: int)
    Err(message: string, code: int)
}
```

- **`interface`** — contract; implemented by classes (and structurally by
  structs/option-payload enums when declared).

## Generic types and functions

```
class Vec<T> {
    var items: List<T>
    fn push(item: T) { items.add(item) }
    fn get(i: usize) -> T => items[i]
}

fn max<T>(a: T, b: T) -> T where T: Comparable => a > b ? a : b
```

- Bounds via `where T: SomeInterface` (or a second decl form
  `fn max<T: Comparable>(a: T, ...)`).
- Variance: generics are invariant; `List<Player>` is not `List<Entity>`.
  Covariance-by-interface is provided explicitly through read-only
  interfaces if needed later.
- Monomorphized per instantiation at compile time; no runtime generic
  machinery, so generics cost nothing at run time.

> **Implemented subset (current compiler).** Generic *classes and structs*
> (`class Box<T>`, `struct Pair<A, B>`) and generic *calls* (`f<int>(…)`,
> `Box<int>(…)`) are monomorphized: each distinct concrete type-argument list
> produces its own instantiation with concrete field types, slot layouts, and
> mangled symbols (`pkl_Box_new__int`, `pkl_Box_read_int`), deduplicated across
> call sites. Generic functions may take, return, and construct generic types
> (`unbox<T>(b: Box<T>)`, `make_pair<A, B>`), including inside other generic
> functions (`Box<T>` under an in-progress `T` resolves once `T` becomes
> concrete). Type arguments can be **omitted when the arguments pin them**:
> `id(3)`, `first([1, 2, 3], 0)`, `swap(b, 9)` infer `int` and land on the
> *same* instantiation as the explicit `id<int>(…)` form; an unpin-able call
> (`id()` with an impossible-to-infer parameter) errors asking for explicit
> `<...>`. Generic functions work as **values** when their type arguments are
> pinned: give them explicitly (`let idI = id<int>`) or pass the bare name to a
> `fn`-typed parameter that pins them (`apply(id, 4)`); the concrete
> instantiation is shared with direct calls and a `let idU = id` with nothing
> to pin is an error (an unresolved type variable leaks into its uses). Still
> bails cleanly ("not lowered yet"): `where` bounds, and lambdas **declared
> inside** generic class/struct bodies (lambdas passed **into** an
> instantiation lower fine). Lambdas **declared inside a generic function
> body** (such as `fn make_identity<T>() -> fn(T) -> T { (x: T) => x }`) are
> supported: each instantiation that reaches the lambda lowers its own hoisted
> copy with that instantiation substituted into the signature, captures, and
> return type. See docs/06-compiler.md "Generic classes…".

## Function types

`(int, string) -> bool`. Callable values:

```
fn apply(f: (int) -> int, x: int) -> int => f(x)
```

Lambdas capture by reference implicitly (GC keeps captured objects alive).
Closure capture is per-variable by value for values and by-reference for
managed references.

## Optional (`T?`)

```
let maybe: string? = findToken()
if (let t = maybe) { print(t) } else { print("none") }
let t2 = maybe ?? "fallback"
let len = maybe?.length()      // none short-circuits
```

`T?` is represented as a tagged pointer: bitmask on references, `none` is the
all-zeros pattern. Flattening: `T??` == `T?`.

Pattern matching covers `Some(v)` / `none` for enums and option.

## Structural subtyping

Only interfaces participate structurally. A function that takes
`Drawable` accepts any non-null argument of a class that `implements
Drawable`. Assignment of `Entity` to `Player` requires an explicit cast:

```
let p = entity as Player     // runtime check, panics (Option: `as?`)
if (entity is Player) { ... }
let maybe = entity as? Player // gives Player?
```

## Type inference

Inference is bidirectional and unification-based:
- literals and calls propagate expected types downward (`Vec<X>` from return
  type context when appropriate) — no, inference is local: `let x = expr`
  infers from `expr`; annotations flow in as expected types where useful
  (arguments of lambdas: `(x) -> x * 2` infers `x` from the function type).
- No global whole-program inference. Type variables resolve within a single
  declaration or signature. This keeps errors local and fast.

## Type checking philosophy

- Errors are reported at the offending expression with the expected vs found
  type and a hint. The compiler is not a linter: it refuses to guess.
- `none`-related errors suggest the fix (`use ??`, `.?`, or an `if (let ...)`
  guard).
- No implicit numeric coercion except widening `byte -> int` is NOT implicit;
  always explicit via `n as i64`? No — avoid multiple ways: assignment of
  wider to narrower and vice versa must use `as`. Conversion syntax is a
  single builtin set of coercions via `Convert` interface methods named
  `toInt()`, `toFloat()`, `toString()`... — hmm. Decide: numeric conversion
  uses one syntax: `value as int` (try semantics for lossless? No, plain
  truncating cast) — `as` is the single numeric conversion operator too.
- Operator overloading goes through `operator name` methods; builtin
  operators are never silently replaced for primitives.

## Soundness notes

- `unsafe` is the escape hatch; safe code is memory-safe under the GC.
- `as` on references performs a runtime check (class id walk); `is` too.
- Enum payload access requires pattern matching (`case Value(x)`), never raw
  field indexing, so impossible states are unrepresentable.