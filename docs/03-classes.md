# PickleScript — Class System Design

Classes are first-class. The OO model is deliberately shallow: no multiple
inheritance, no checked exceptions, no reflection ceremony, and no rule that
everything must live in a class. Composition is encouraged, interfaces
provide abstraction, and inheritance is single.

## Class shape

```
class Player {

    name: string                 // field, public by default
    health: int = 100            // field with initializer
    private secret: int          // private field

    constructor(name: string) {  // explicit constructor
        this.name = name
    }

    constructor.guest() {        // named constructor (factory flavor)
        this("Guest")
    }

    fn damage(amount: int) {     // method
        health -= amount
    }

    fn alive() -> bool => health > 0

    static var count = 0         // static field
    static fn create() -> Player => Player("Anonymous")

    property label: string {     // property
        get => "Player {name} ({health} hp)"
    }

    operator == (other: Player) -> bool => name == other.name
}
```

## Semantics

- Fields are plain data. The primary field list doubles as the data layout.
- A field with an initializer runs in declaration order during construction.
- Research methods: zero-parameter reads and calls may drop empty parens in
  expression position (`p.alive` reads a property the same way `p.alive()`
  calls it — no, see syntax rules below).
- Methods defined with `fn`; the receiver is implicit (`health -= amount`
  modifies `this.health`); use `this` when a name needs to be explicit.
- Constructors run field initializers first, then constructor body.
  `super(args)` may be called first in a subclass constructor.
- `property` computes a value and/or intercepts assignment; no backing field
  is created. `get`/`set` bodies are small blocks or `=>` expressions.

## Method resolution rules

- `override fn` redefines an inherited method. `override` is required when
  redefining a non-abstract parent method; the compiler enforces it.
- `final fn` (or `final class`) blocks further overriding.
- `private` members: class only. `protected`: class + subclasses.
  `public` (default): everywhere.
- Calling a protected/private member through an unrelated base reference is
  a compile error.

## Inheritance

```
class Enemy extends Entity {

    override fn update(delta: float) { ... }

}
```

Rules:
- Single inheritance, unlimited depth.
- A class implicitly implements all interfaces implemented by its parent.
- `super.m()` calls the parent implementation.
- Abstraction: an abstract method is declared by omitting the body
  (`fn update(delta: float)`) in a class; often via an `interface` instead.

## Interfaces

```
interface Drawable {
    fn draw()
    fn width() -> int => 0     // default implementation
}

class Sprite implements Drawable {
    fn draw() { }
}
```

- Interfaces declare method contracts; implementations are required unless
  a default body is present.
- A class may implement any number of interfaces.
- Interfaces cannot hold state (fields/properties with backing state are
  forbidden; computed properties are allowed).
- Nothing is *explicitly declared* as implementing — `implements` only needs
  the methods to match structurally, but declaring it is required to keep the
  contract visible and errors local.

Wait — no. `implements Drawable` is declared explicitly. The compiler then
verifies every required member exists. (Structural implements without
declaring is rejected to keep contracts explicit.)

## Static members

- `static var/let/const` fields: one per class, initialized lazily on first
  touch (thread-safe once).
- `static fn`: callable as `Player.create()`; has no receiver, cannot see
  instance state.
- Statics are inherited through the type name.

## Object identity, equality, hashing

- By default: reference identity for classes, structural equality for
  structs and enums.
- `operator ==` / `operator hash` customize equality for classes.
- Standard collections use these operators.

## Construction without ceremony

`Player("Hero")` requires a matching constructor. When no constructor is
declared, the compiler synthesizes:

- Default constructor `Player()` if all fields have defaults, plus
  `Player(name: string, ...)` — no: synthesized constructor is `Player()`
  using field initializers, and named-argument construction
  `Player(name: "Hero", health: 200)` is accepted for any class with no
  explicit constructor. This keeps little data classes terse:

```
class Point {
    x: float
    y: float
}

let p = Point(x: 1.5, y: 2.0)   // synthesized
```

## Object lifecycle hooks

```
class File {
    init { ... }        // after fields + constructor body, before first use
    deinit { ... }      // before collection (don't rely on timing)
}
```

`deinit` runs during GC for cleanup of native handles; it must not allocate
or allocate can be deferred to `init` alternates. Finalizers are best-effort.

## Composition over inheritance

PickleScript ships generic composition aids (delegation is not a keyword;
use fields + forwarding methods). Prefer:

```
class Keyboard {
    fn key(name: string) -> bool { ... }
}

class Editor {
    var kb = Keyboard()
    fn key(name: string) -> bool => kb.key(name)
}
```