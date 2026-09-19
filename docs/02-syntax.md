# PickleScript — Syntax Specification

Version 1. Files use the `.pkl` extension.

## Lexical conventions

- Identifiers: `[A-Za-z_][A-Za-z0-9_]*`. Type names are conventionally
  `UpperCamelCase`; the compiler warns on style violations but doesn't
  require them.
- Keywords are reserved and lowercase.
- Comments: `// line`, `/// doc line`, `/* block */`, `/** doc block */`.
  Doc comments attach to the following declaration.
- Numeric literals:
  - `42`, `1_000_000`, `0xFF`, `0b1011`, `0o17`
  - `0.5`, `3.14e2`, suffix types: `7u8`, `-2i32`, `2.5f32` (default `int`,
    `float`)
- String literals:
  - `"hello"` — interpolated; `{expr}` embeds `expr.toString()`.
    `"Score: {score}"`. Escapes: `\n \t \\ \" \{ \u{1F600}`.
  - `r"raw text {not inserted}"` — no interpolation.
  - `"""multi\nline\nstring"""` — interpolation allowed.
- Character literal: `'a'` — a Unicode scalar, type `char`.

## Statements

No semicolons. A statement ends at a newline or a closing brace, unless the
line is *visibly incomplete* (ends with an operator, a `,`, or an open
delimiter `( [ {`, or continues with `.` / `?` / `->`), in which case the
expression continues onto the next line.

```
let x = f(          // incomplete: open paren
    1, 2,
)
let y = 1 +
        2           // incomplete: trailing operator
let z = 3           // complete
```

### Declarations

```
let name = expr            // immutable binding
let name: Type = expr      // annotated
var count = 0              // mutable
const MAX = 100            // compile-time constant
const NAME: string = "pkl"
```

`let` bindings may be pattern based: `let (a, b) = pair`.

### Control flow

```
if (cond) { } else if (cond) { } else { }

if (let user = findById(7)) { }   // conditional binding (none => else)

while (cond) { }

for (let i = 0; i < n; i++) { }

for (item in items) { }           // iteration over any Iterable

for (i in 0..10) { }              // half-open range
for (i in 0..=10) { }             // inclusive range

break
continue
return [expr] | return          // bare return == return none, if option
```

### Expression statements

```
player.damage(20)
print("done")
```

### Declarations & expressions together

```
// Single-expression bodies keep small functions compact.
fn double(x: int) -> int => x * 2
```

## Functions

```
fn greet(name: string, punctuation: string = ".") -> string {
    return "Hello, {name}{punctuation}"
}

fn add(a: int, b: int) -> int { a + b }    // tail expression = value

fn describe() -> (string, int) {           // tuple return
    return ("pickle", 7)
}

fn join(...items: string) -> string { }    // rest parameter

fn apply(fn f: (int) -> int, n: int) -> int { f(n) }
```

- Calls: positional and named arguments:
  `greet("Sam")`, `greet(name: "Sam")`, `moveTo(x: 10, y: 20)`.
- Overloads resolved by argument types (arity + types). Default arguments
  fill the tail.
- Lambda: `(x) -> x * 2`, `(x, y) -> x + y`, and block form
  `fn (x: int) -> int { x * x }`.

## Operators

| Precedence | Operators                    |
|-----------|------------------------------|
| high      | `.` `?` `::` `(` `[` `->`    |
|           | `!` `-` (unary) `~`          |
|           | `**`                         |
|           | `*` `/` `%`                  |
|           | `+` `-`                      |
|           | `<<` `>>` `&`                |
|           | `==` `!=` `<` `<=` `>` `>=` `is` `in` |
|           | `&&` / `and`                 |
|           | `||` / `or`                  |
|           | `??` (default) `?:`          |
|           | `=` `:=` `+=` `-=` `*=` `/=` `%=` `<<=` `>>=` `&=` `|=` `^=` `++=` |

- `==` is structural for structs and enums, identity for classes unless
  overridden (`operator == (other) -> bool`).
- `is` type test; `in` membership / map key test.
- `?` postfix on `T?`: `let x = maybe?.field` (short-circuit `none`).
- `a ?? b`: `a` if present, else `b`.
- `&expr` address-of (inside `unsafe`), `*T` pointer dereference,
  `ptr->field` pointer field access, `<-` channel send: `ch <- value`.

## Modules

```
module game.player          // path. Identifies the file.

import graphics             // qualified: graphics.render(...)
import graphics as g        // aliased: g.render(...)
use graphics.render         // unqualified: render(...)
use graphics.*              // unqualified: everything public
```

Import paths resolve against `src/` and package dependencies. A file's module
path is derived from `module` declaration or its path under `src/`.

## Full keyword list

```
module import use class struct enum interface
fn constructor property get set operator
let var const static override public private protected
if else while for in break continue return
match case
true false none
init deinit
async await task channel spawn
unsafe
extends implements
is as
this super
```

## Example walking

```
module game

import graphics

class Player {

    name: string
    health: int = 100
    private secret: int = 0

    constructor(name: string) {
        this.name = name
    }

    fn damage(amount: int) {
        health -= amount
    }

    fn alive() -> bool => health > 0

    property level: int {
        get => health / 10
    }
}

enum State { Idle Moving Dead }

fn main() {
    let p = Player("Hero")
    p.damage(20)
    print("Health: {p.health}")
    var state = State.Idle
    match (state) {
        case State.Dead -> print("dead")
        case -> print("alive")
    }
}
```