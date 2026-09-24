# Error codes

Every diagnostic may carry a stable code. The code is assigned at the point
the compiler detects the problem — never guessed from rendered text
afterwards — so the code and the message always describe the same fact. A
diagnostic without a code renders as `error -- (uncoded)` rather than a
wrong code; when a new construct is added its diagnostic stays uncoded until
a code exists that says exactly what happened.

This page mirrors the catalogue in `compiler/src/error.rs` (the source of
truth). `pickle explain <CODE>` prints the same rule and example with no
docs to install, and `pickle explain` with no code lists every entry.

Codes are grouped by area:

- `E01xx` — lexing and parsing
- `E02xx` — declarations, name resolution, inheritance
- `E03xx` — type checking and expressions
- `E04xx` — operations, attributes, calls, memory builtins
- `E05xx` — ownership and memory
- `E09xx` — checker accepted, but the code generator has no lowering yet

## E01xx — lexical and syntax errors

### E0101 · lexical error

The source contains a byte sequence the lexer cannot turn into a token; a
literal is unterminated, or a character is not valid in this position.

```
let s = "unterminated
```

### E0111 · syntax error

The token stream does not form a valid declaration, statement, or
expression at this position.

```
fn main() { let = }
```

## E02xx — declarations and name resolution

### E0201 · duplicate declaration

A top-level name (function, class, struct, enum, interface, const) is
declared twice in the same module.

```
fn main() {}  fn main() {}
```

### E0202 · duplicate import alias

Two module imports resolve to the same alias in this module.

```
import a as util  import b as util
```

### E0206 · module not found

An import path does not resolve to a file: it is searched relative to the
importing file's directory, then under `<cwd>/src/` (or `<cwd>/src/<path>.pkl`
when the path maps to a nested folder).

```
import nowhere.utility
```

### E0207 · item not exported by module

An `import { item }` references a name the referenced module does not declare
at top level.

```
import { Missing } from text.lexer
```

### E0208 · module path mismatch

A file declares a `module` path that differs from the path used to load it:
resolving the import locates the file under a different module name than the
one it declares.

```
import b.tool  // b/tool.pkl declares `module a.tool`
```

### E0209 · ambiguous imported name

Two loaded modules both export the same top-level function or const, and a bare
reference in this file cannot tell them apart. Disambiguate with
`import <item> from <mod> as <alias>` first.

```
import helper from a.utils  import helper from b.utils  fn main() { helper() }
```

### E0203 · duplicate member

A class or struct declares two members with the same name.

```
class P {
    var x: int
    var x: string
}
```

### E0204 · duplicate constructor

A class or struct declares the primary constructor more than once.

```
class P { constructor() {}  constructor() {} }
```

### E0205 · duplicate deinit

A class declares more than one `deinit` block.

```
class P { deinit {}  deinit {} }
```

### E0210 · unknown type

A type name does not resolve to a declared class, struct, enum, interface,
generic parameter, or built-in type. Name collection is two-pass, so a type
declared later in the file (including self- and mutually recursive type
graphs) resolves fine; this error only fires for names that are never
declared.

```
fn f(x: Widget) {}
```

### E0211 · type is not generic

A generic instantiation `Name<T>` is applied to a type that takes no type
parameters.

```
let xs: string<int> = ""
```

### E0212 · wrong number of type arguments

A generic instantiation supplies fewer or more type arguments than the type
declares.

```
class Box<T> {}
let b: Box<int, string> = Box<int>()
```

### E0213 · list of references

Lists storing `&T` references are not supported yet; a reference cannot
outlive the borrowed local it points to inside a collection.

```
let refs: List<&int> = []
```

### E0220 · invalid extends

Only a class may extend a class; extends/`implements` targets must be the
right kind of type.

```
class Foo extends SomeStruct
```

### E0221 · override signature mismatch

An `override` method must match its parent's parameter and return types
exactly.

```
class B extends A { override fn m(x: int) -> int }  // A.m takes string
```

### E0222 · missing override

A method redefines an inherited method without the `override` keyword.

```
class B extends A { fn m() {} }  // A already has m()
```

### E0223 · orphan override

A method is marked `override` but no parent in the class chain declares it.

```
class B extends A { override fn n() {} }  // A has no n
```

### E0224 · mixed method kind

A class cannot redeclare an inherited method under the opposite kind: a `static`
method cannot shadow an inherited instance method, and an instance method cannot
shadow an inherited `static` method.

```
class B extends A { static fn m() {} }  // A.m is an instance method
```

## E03xx — type checking and expressions

### E0308 · type mismatch

An expression's type does not satisfy the declared or expected type, after
option/subtyping conversions are tried.

```
fn f() -> int { return "x" }
```

### E0310 · return value error

A function with no return type returns a value, or `if`/`match` branches
produce inconsistent result types.

```
fn f() { return 1 }   fn g(b) { if b { 1 } else { "x" } }
```

### E0320 · break or continue outside a loop

`break` and `continue` are only valid inside `while`/`for` bodies.

```
fn f() { break }
```

### E0340 · condition is not bool

`if`, `while`, `for` conditions, and `&&`/`||` operands must be `bool`.

```
if (1) {}
```

### E0341 · for-in requires a sequence

`for (x in seq)` needs a sequence: a List, a string, a Map, or a range.

```
for (x in 42) {}
```

### E0342 · map keys must be a supported type

Map keys may be `string`, a scalar (`int`, `float`, `bool`, `char`, `byte`), a
composite object (`List`, `Map`, class/struct instance, enum, interface), an
option, or a tuple. Keys are hashed and compared structurally (deep), so a
`List<int>` key `[1, 2]` matches any equal list, and a `(int, string)` tuple
key matches any equal tuple. Reference, function, and pointer keys are not
supported.

```
let m = { (new cls(), 2): "a" }   // class methods/references are not keys
```

### E0343 · index error

The index expression has the wrong type for the indexed value, or the value
cannot be indexed at all.

```
let xs = [1,2]  let x = xs["a"]
```

### E0351 · undeclared name

An identifier in expression position is not a local, function, const, type,
or member of the surrounding class.

```
fn main() { print(nope) }
```

### E0352 · no such member

A field, property, or method name does not exist on the receiver class,
struct, or List/Map built-ins.

```
class P { var x: int }
let p = P()  p.y
```

### E0353 · enum variant error

A variant name does not exist on the enum, or a bare variant name is used
where a payload constructor is required.

```
enum Color { Red  Green }
case Color.Purple -> {}
```

### E0354 · cannot construct this type

Interfaces and enums are not directly constructible; classes and structs
are.

```
let i = SomeInterface()
```

### E0355 · member assignment on non-class value

Assigning through `.member` requires the receiver to be a class or struct
value.

```
let n = 5  n.x = 3
```

### E0360 · interface conformance

A type declaring `implements I` must provide every member `I` declares.

```
class P implements Runnable { }  // no run()
```

### E0361 · this / super misuse

`this`/`super` are only valid inside a class body; `this(...)` only as a
named constructor's single delegation.

```
fn f() { this }
```

### E0362 · invalid assignment

The assignment target is not assignable: it is immutable, undeclared, a
wrong receiver kind, a missing setter, or write-through an `&T`.

```
fn f() { let x = 1  x = 2 }
```

### E0363 · invalid cast or is

The cast/`is` target is unrelated to the source type (no numeric
conversion, option relationship, or inheritance). `char`/`int` casts are
allowed (scalar reinterpreting; see the `chartoi`/`itochar` notes in
`06-compiler.md`) but `is` between a `char` and an integer still rejects —
a char value is neither an int nor a byte.

```
let n = 5 as bool
```

## E04xx — operations, attributes, calls

### E0400 · attribute error

An attribute name is unknown, repeated, or given arguments it does not take.

```
#[wat] let x = 1
```

### E0401 · manualAlloc misuse

`#[manualAlloc]` applies to class/struct types only: its binding needs an
allocation initializer, fields need an initializer and, when untyped, an
annotation; it is not allowed on statics or constants.

```
#[manualAlloc] let x = 5
```

### E0410 · call argument error

The call passes too many or too few arguments, uses named arguments where
the callee takes positional ones, or a generic function name is unknown.

```
fn add(a: int) {}  add(1, 2)
```

### E0411 · call of non-function

Only functions, type constructors, and enum variant constructors can be
called.

```
let n = 5  n()
```

### E0420 · member access on non-class value

`.member` requires a class, struct, enum, or interface receiver; an option
(`T?`) must be unwrapped or accessed with `?.` first, and scalars have no
members.

```
let n = 5  n.name
```

### E0430 · operator operand error

An operator is applied to operands of a type it does not define
(non-numeric arithmetic, non-int bitwise and shifts, non-bool `!`,
unpacking a non-option, dereferencing a non-pointer).

```
let s = "a"  let n = s * 2
```

### E0440 · alloc / free misuse

The raw-buffer builtins have a fixed shape: `alloc(T, count)` with a scalar
element type and an `int` count, `free(p)` with a pointer argument, zero
extra `.free()`/`alloc` arguments when used on bindings.

```
unsafe { alloc(Widget, 4) }  // scalar element types only
```

## E05xx — ownership and memory

### E0501 · use after free or move

Reading or calling with a `#[manualAlloc]` value after `.free()` or after
its ownership moved elsewhere.

```
x.free()  println(x)
```

### E0502 · already freed

A `#[manualAlloc]` binding is released twice.

```
x.free()  x.free()
```

### E0503 · free on a non-manual value

`x.free()` is only available on a `#[manualAlloc]` binding, which owns its
allocation.

```
let x = User()  x.free()
```

### E0504 · owned position

A `#[manualAlloc]` value can only reach an owning position by moving a
`#[manualAlloc]` binding or a fresh allocation; it cannot be stored in a
managed binding, field, or collection.

```
#[manualAlloc] let y = x  // x must be manual or fresh
```

### E0505 · overwrite or non-owning return of a manual value

A `#[manualAlloc]` binding cannot be overwritten, and a manual value cannot
be returned from a function that does not declare `#[manualAlloc]`.

```
x = make()  // x is manual
```

### E0506 · leaked owned result

The result of a `#[manualAlloc]` function is owned by the caller and must be
bound, passed to an owned parameter, or returned; discarding it leaks.

```
make()  // result dropped, not consumed
```

### E0520 · unsafe required

Raw pointers, `&`/`*`, pointer access, indexing and stores, and the
raw-buffer builtins are only allowed inside `unsafe { }`.

```
let p = &local
*p = 5
```

### E0700 · reference outside parameter

`&T` exists only as a function parameter type; stored or returned `&T` would
dangle once the borrowed local goes away.

```
let r: &int = &x
```

## E09xx — not lowered yet

### E0900 · not lowered yet

The checker accepted the construct, but the code generator does not lower it
yet; the program is rejected instead of miscompiled.

```
let (a, b) = (1, 2)  // tuple values lower; `let` destructuring does not
```

## Uncoded diagnostics

Some diagnostics intentionally render without a code. A message that has no
bucket saying exactly what happened (for example `tuple pattern does not
match a tuple value`) stays `(uncoded)` rather than being pinned to a code
that only roughly fits. Adding a code means adding a catalogue entry and a
`CatalogueEntry` in `compiler/src/error.rs`, and assigning the code at the
diagnostic's creation site.