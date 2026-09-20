# PickleScript — Class System Design

Classes are first-class. The OO model is deliberately shallow: no multiple
inheritance, no checked exceptions, no reflection ceremony, and no rule that
everything must live in a class. Composition is encouraged, interfaces
provide abstraction, and inheritance is single.

## Class shape

```
class Employee {

    name: string                 // field, public by default
    salary: int = 40_000         // field with initializer
    private id: int              // private field

    constructor(name: string) {  // explicit constructor
        this.name = name
    }

    constructor.guest() {        // named constructor (factory flavor)
        this("Guest")
    }

    fn raise(amount: int) {      // method
        salary += amount
    }

    fn senior() -> bool => salary >= 100_000

    static var count = 0         // static field
    static fn create() -> Employee => Employee("Anonymous")

    property label: string {     // property
        get => "Employee {name} ({salary})"
    }

    operator == (other: Employee) -> bool => name == other.name
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
- A named constructor (`constructor.name(...)`) is a factory: its body is
  exactly one `this(args)` delegation to the primary constructor, it is called
  as `Type.name(args)`, and it returns a fully constructed instance.
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
class Manager extends Employee {

    override fn review() { ... }

}
```

Rules:
- Single inheritance, unlimited depth.
- A class implicitly implements all interfaces implemented by its parent.
- `super.m()` calls the parent implementation.
- Abstraction: an abstract method is declared by omitting the body
  (`fn review()`) in a class; often via an `interface` instead.

> **Implemented subset (current compiler).** `extends` is lowered for state,
> methods, `override fn`, `super.m()`, `is`/`as`, and constructor chaining.
> Fields are laid out parent-first (superclass slots occupy the low indices), a
> subclass inherits its parent's methods, and its synthesized constructor takes
> every non-initialized instance field superclass-first. Classes must be
> registered ancestor-first, so a superclass always has a lower runtime id. A
> method that is overridden somewhere in the program dispatches on the
> receiver's runtime class (`after : Animal`, `p : Poodle` both call
> `Poodle.describe`), while `super.m()` stays bound to the superclass
> implementation and never-overridden methods call statically. `x is T` /
> `x as T` test the runtime class chain: an upcast is statically
> true/identity, a downcast is a checked runtime test (`as` panics on a bad
> cast). A subclass constructor may open with `super(args)`: the arguments are
> checked against (and chain into) the parent constructor — an explicit parent
> gets its parameters bound and its body run inlined, a synthesized parent gets
> its uninitialized fields filled — recursing across the whole hierarchy.
> Named constructors in a hierarchy delegate through their class's primary
> constructor, which handles the chain. A class that relies on the synthesized
> constructor still cannot sit below a class that declares an explicit
> constructor. **Not yet lowered** (a loud "not lowered yet" diagnostic, never
> a silent miscompile): interfaces/`implements`. Generic classes
> (`class Box<T>`) are lowered on their own (see 04-types.md), but **not inside
> a hierarchy**: an instantiated generic class using
> `extends`/`implements`/`override` bails.

## Interfaces

```
interface Serializable {
    fn export(path: string)
    fn version() -> int => 0     // default implementation
}

class Config implements Serializable {
    fn export(path: string) { }
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

- `static var` fields: one per class, initialized once at program start (in
  class-registration order) to their declared value or the field type's
  default. Reads and writes use the type name (`Employee.count`); a static
  method may use the bare name.
- `const NAME = value` members: compile-time constants. Reads use the type
  name (`Employee.MAX`), or the bare name inside the class body; one constant
  may reference another. They are inlined at each use, immutable, and cannot
  be reached through an instance.
- `static fn`: callable as `Employee.create()`; has no receiver, cannot see
  instance state.
- Statics are inherited through the type name.

## Object identity, equality, hashing

- By default: reference identity for classes, structural equality for
  structs and enums.
- `operator ==` / `operator hash` customize equality for classes.
- Standard collections use these operators.

## Construction without ceremony

`Point(1.5, 2.0)` requires a matching constructor. When no constructor is
declared, the compiler synthesizes:

- Default constructor `Point()` if all fields have defaults, plus
  `Point(x: float, ...)` — no: synthesized constructor is `Point()`
  using field initializers, and named-argument construction
  `Point(x: 1.5, y: 2.0)` is accepted for any class with no
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
    deinit { ... }      // at GC sweep, once, best-effort
}
```

`init` runs after the fields and the constructor body are initialised, with
`this` live.

`deinit` is the GC finalizer. At most one `deinit` is allowed per class; it
compiles to a private `pkl_<T>_deinit(this)` function whose address is
registered on the class descriptor, and the collector runs it when a **dead**
instance is swept. It runs at most once, off the allocation path, in an
unspecified order relative to other finalizers, and it must **not allocate**
(allocation from a finalizer can alias the collector's borrow). Finalizers are
best-effort: they only run if a collection actually reclaims a dead instance,
so do not rely on timing or on them running at all.

```
class Handle {
    fd: int

    deinit {
        println(this.fd)   // `this` is live; do not allocate here
    }
}
```

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