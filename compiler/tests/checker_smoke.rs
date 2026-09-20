use pickle_compiler::diag::{DiagnosticSink, SourceMap};
use pickle_compiler::front::frontend_checked;

fn check_str(src: &str) -> DiagnosticSink {
    let diags = DiagnosticSink::new();
    let mut map = SourceMap::default();
    let _ = frontend_checked("test.pk", src, &mut map, &diags);
    diags
}

fn has_errors(diags: &DiagnosticSink) -> bool {
    diags.any_error()
}

fn error_msgs(diags: &DiagnosticSink) -> String {
    diags
        .diagnostics
        .borrow()
        .iter()
        .map(|d| d.message.clone())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn accepts_well_typed_module() {
    let d = check_str(
        r#"fn double(x: int) -> int {
            return x * 2
        }

        fn main() {
            let a = double(21)
            let b: int = a + 1
            print("b is {b}")
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_mismatched_return() {
    let d = check_str(
        r#"fn bad() -> int {
            return "hello"
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a type mismatch error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn rejects_unknown_name() {
    let d = check_str(
        r#"fn main() {
            print(nope)
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected an undeclared-name error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_class_methods_and_fields() {
    let d = check_str(
        r#"class Counter {
            var count: int = 0

            constructor(start: int) {
                count = start
            }

            fn increment(by: int) {
                count = count + by
            }

            fn value() -> int {
                return count
            }
        }

        fn main() {
            let c = Counter(1)
            c.increment(2)
            print(c.value())
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_assign_to_immutable() {
    let d = check_str(
        r#"fn main() {
            let x = 1
            x = 2
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected an immutable-assignment error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_option_flows() {
    let d = check_str(
        r#"fn maybe() -> int? {
            return none
        }

        fn main() {
            let m: int? = maybe()
            let v = m ?? 0
            print(v)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn coalesce_yields_inner_type() {
    // `a ?? b` is the unwrapped inner type, so its result assigns to `int`
    // (not `int?`).
    let d = check_str(
        r#"fn maybe() -> int? {
            return none
        }

        fn main() {
            let m: int? = maybe()
            let v: int = m ?? 0
            print(v)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_mismatched_list_elements() {
    let d = check_str(
        r#"fn main() {
            let xs = [1, 2, "three"]
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a list-element type error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_generic_function() {
    let d = check_str(
        r#"fn id<T>(x: T) -> T {
            return x
        }

        fn main() {
            let a = id(3)
            let b = id("hi")
            print(a, b)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn accepts_enum_match() {
    let d = check_str(
        r#"enum Color {
            Red
            Blue
            Rgb(r: int, g: int, b: int)
        }

        fn main() {
            let c = Color.Rgb(1, 2, 3)
            match (c) {
                case Color.Red -> print("red")
                case Color.Blue -> print("blue")
                case Color.Rgb(r, g, b) -> print(r, g, b)
            }
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn accepts_enum_match_as_value() {
    let d = check_str(
        r#"enum Shape {
            Circle(r: float)
            Rect(w: float, h: float)
        }

        fn area(s: Shape) -> float {
            match (s) {
                case Shape.Circle(r) -> 3.14 * r * r
                case Shape.Rect(w, h) -> w * h
            }
        }

        fn main() {
            let s = Shape.Circle(1.0)
            println(area(s))
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_inconsistent_match_arms() {
    let d = check_str(
        r#"enum E { A(x: int) B }
        fn pick(v: int) -> E {
            if (v > 0) { E.A(1) } else { E.B }
        }
        fn main() {
            let e = pick(1)
            match (e) {
                case E.A(x) -> x
                case E.B -> "text"
            }
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected inconsistent match arm type error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_lambda_higher_order() {
    let d = check_str(
        r#"fn apply(f: fn (int) -> int, x: int) -> int {
            return f(x)
        }

        fn main() {
            let double = (n) => n * 2
            let r = apply(double, 21)
            print(r)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_bad_lambda_call() {
    let d = check_str(
        r#"fn main() {
            let f = (n: int) => n + 1
            let s = f("hi")
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a lambda argument type error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_is_as_casts() {
    let d = check_str(
        r#"class Shape {
            fn area() -> int {
                return 0
            }
        }

        class Circle extends Shape {
            var radius: int

            constructor(r: int) {
                radius = r
            }
        }

        fn main() {
            var s: Shape = Circle(5)
            let flag = s is Circle
            let c = s as Circle
            let maybe = s as? Circle
            let r = c.radius
            print(flag, r, maybe)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn as_question_yields_option() {
    // `as?` produces an option, so its result assigns to `T?`; plain `as` and
    // numeric conversions assign to the target/number type.
    let d = check_str(
        r#"fn main() {
            let x: int = 5
            let y: int? = x as? int
            let z: int = 3.9 as int
            let f: float = x as float
            let b: bool = x is int
            print(y, z, f, b)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_invalid_is_cast() {
    let d = check_str(
        r#"fn main() {
            let x: int = 42
            let flag = x is string
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a cast error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_control_flow_while_for_if() {
    let d = check_str(
        r#"fn main() {
            var sum = 0
            for (i in [1, 2, 3, 4, 5]) {
                if (i > 3) {
                    sum = sum + i
                }
            }
            var k = 0
            while (k < 3) {
                k = k + 1
                if (k == 2) {
                    continue
                }
            }
            print(sum)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn accepts_string_concatenation() {
    // `"a" + "b"` must be typed `string`, not `int`.
    let d = check_str(
        r#"fn greet(name: string) -> string {
            "hello " + name
        }

        fn main() {
            let s: string = "a" + "b"
            print(s)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn accepts_operators_inference() {
    let d = check_str(
        r#"fn main() {
            let a = 1 + 2 * 3
            let b = 10 / 4
            let c = 7 % 3
            let d = -a
            let e = a < b
            let f = !e
            let g = a == 7
            let h = "x" + "y"
            let neg = -c
            print(a, b, c, d, e, f, g, h, neg)
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_op_type_mismatch() {
    let d = check_str(
        r#"fn main() {
            let x = 1 + "two"
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a binary-op type error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn rejects_option_type_mix() {
    let d = check_str(
        r#"fn main() {
            let m: int? = none
            let n: int = m
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected an option-type error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_map_typing() {
    let d = check_str(
        r#"fn total(m: Map<string, int>) -> int {
            let a = m["x"]
            let ks = m.keys()
            let vs = m.values()
            return a + len(ks) + len(vs)
        }

        fn main() {
            var m = {"alpha": 10, "beta": 20}
            m["beta"] = m["beta"] + m["missing"]
            let hasit: bool = m.has("alpha")
            print(hasit, total(m))
            for (x in m) {
                print(x)
            }
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_non_string_map_keys() {
    let d = check_str(
        r#"fn main() {
            var m = {1: "one"}
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a map-key type error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn rejects_non_string_map_index() {
    let d = check_str(
        r#"fn main() {
            var m = {"a": 1}
            let x = m[42]
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a map-index type error, got:\n{}",
        error_msgs(&d)
    );
    let msgs = error_msgs(&d);
    assert!(
        msgs.contains("string"),
        "expected a `string` key diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_static_property_accessors() {
    // Static properties type-qualify through the class name; getters may
    // reference other statics by `Type.prop`; instance properties must not
    // be reached through the type name (or statics through instances).
    let d = check_str(
        r#"class Config {
            static property limit: int {
                get => 10
            }

            static property label: string {
                get => "cfg"
            }

            static property guarded: int {
                get => Config.limit
                set {}
            }

            property instanceOnly: int { get => 1 }
        }

        fn main() {
            println(Config.limit)
            println(Config.label)
            Config.guarded = 11
            println(Config.limit)
        }"#,
    );
    assert!(!has_errors(&d), "expected clean program, got:\n{}", error_msgs(&d));
}

#[test]
fn rejects_static_instance_misuse() {
    // Static properties cannot be reached through an instance, and instance
    // properties cannot be reached through the type name.
    let d = check_str(
        r#"class Config {
            static property limit: int {
                get => 10
            }

            property doubled: int { get => limit * 2 }
        }

        fn main() {
            let c = Config()
            println(c.limit)
            println(Config.doubled)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(
        msgs.contains("static property `limit` must be accessed on the type `Config`"),
        "missing instance-to-static diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains("instance property `doubled` must be accessed on an instance of `Config`"),
        "missing type-to-instance diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_non_bool_match_guard() {
    // A guard must be a bool expression; an `int` guard types through.
    let d = check_str(
        r#"enum Color {
            Red
            Blue
            Rgb(r: int, g: int, b: int)
        }

        fn pick(c: Color) -> int {
            match (c) {
                case Color.Blue if 3 -> 1
                case other -> 0
            }
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a guard typing error, got:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn accepts_guarded_match_arms() {
    // Payload bindings are live in guards, so `if r > 0` sees each binder.
    let d = check_str(
        r#"enum Color {
            Red
            Blue
            Rgb(r: int, g: int, b: int)
        }

        fn pick(c: Color) -> int {
            match (c) {
                case Color.Rgb(r, g, b) if r > 0 && g > 0 && b > 0 -> r + g + b
                case Color.Rgb(_, _, _) -> -1
                case Color.Blue if c != Color.Red -> 2
                case other if true -> 3
            }
        }"#,
    );
    assert!(!has_errors(&d), "expected clean program, got:\n{}", error_msgs(&d));
}

#[test]
fn rejects_too_few_constructor_args() {
    // A missing synthesized-ctor parameter used to reach codegen and turn into
    // a JIT verifier crash; the checker must flag the arity mismatch instead.
    let d = check_str(
        r#"class Pair {
            var a: int
            var b: int
        }

        fn main() {
            let p = Pair(1)
            println(p.a)
        }"#,
    );
    assert!(
        has_errors(&d),
        "expected a constructor arity error, got:\n{}",
        error_msgs(&d)
    );
    let msgs = error_msgs(&d);
    assert!(
        msgs.contains("2 argument(s), found 1"),
        "expected the arity diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_static_field_access() {
    // Static fields type-qualify through the class name (read, write, compound)
    // and are reachable by bare name inside the class's own static methods.
    let d = check_str(
        r#"class Counter {
            static var total: int = 10
            static var flag: bool

            static fn bump(by: int) -> int {
                total = total + by
                return total
            }
        }

        fn main() {
            println(Counter.total)
            Counter.total = Counter.total + 5
            Counter.total += 7
            println(Counter.flag)
            println(Counter.bump(3))
        }"#,
    );
    assert!(!has_errors(&d), "expected clean program, got:\n{}", error_msgs(&d));
}

#[test]
fn rejects_static_field_instance_misuse() {
    // Static fields cannot be reached through an instance, and instance fields
    // cannot be reached through the type name (reads or writes).
    let d = check_str(
        r#"class Counter {
            static var total: int = 10
            var local: int = 1
        }

        fn main() {
            let c = Counter()
            println(c.total)
            c.total = 3
            println(Counter.local)
            Counter.local = 4
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(
        msgs.contains(
            "static field `total` must be accessed on the type `Counter`, not on an instance"
        ),
        "missing instance-to-static read diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains(
            "static field `total` must be assigned on the type `Counter`, not on an instance"
        ),
        "missing instance-to-static write diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains("instance field `local` must be accessed on an instance of `Counter`"),
        "missing type-to-instance read diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains("instance field `local` must be assigned on an instance of `Counter`"),
        "missing type-to-instance write diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_class_const_access() {
    // Class constants type-qualify through the class name and are reachable by
    // bare name inside the class body (static and instance methods); one
    // constant may reference another.
    let d = check_str(
        r#"class Config {
            const BASE: int = 2
            const DOUBLE: int = BASE * 2
            const NAME: string = "cfg"

            static fn label() -> string {
                return NAME
            }

            fn limit() -> int {
                return DOUBLE + 1
            }
        }

        fn main() {
            println(Config.BASE)
            println(Config.DOUBLE)
            println(Config.NAME)
            println(Config.label())
            let c = Config()
            println(c.limit())
            let x: int = Config.BASE * 10
        }"#,
    );
    assert!(!has_errors(&d), "expected clean program, got:\n{}", error_msgs(&d));
}

#[test]
fn rejects_const_misuse() {
    // Constants are immutable and static: assigning one, reading one through an
    // instance, and a value of the wrong type must all be rejected.
    let d = check_str(
        r#"class Config {
            const BASE: int = 2
            const BAD: int = "nope"
        }

        fn main() {
            Config.BASE = 5
            let c = Config()
            println(c.BASE)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(
        msgs.contains("cannot assign to immutable static field `BASE`"),
        "missing const-assignment diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains(
            "static field `BASE` must be accessed on the type `Config`, not on an instance"
        ),
        "missing instance-to-const read diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains("const initializer"),
        "missing const initializer type diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_named_constructor() {
    // A named constructor is a static factory whose body is a single
    // `this(...)` delegation to the primary constructor, including when the
    // primary constructor is synthesized from the fields.
    let d = check_str(
        r#"class Point {
            x: int
            y: int = 10

            constructor(x: int) {
                this.x = x
            }

            constructor.origin() {
                this(0)
            }

            constructor.diagonal(n: int) {
                this(n * 2)
            }
        }

        class Box {
            w: int

            constructor.square(s: int) {
                this(s)
            }
        }

        fn main() {
            let a = Point.origin()
            let b = Point.diagonal(3)
            let c = Point(7)
            let d = Box.square(5)
            println(a.x)
            println(a.y)
            println(b.x)
            println(c.x)
            println(d.w)
        }"#,
    );
    assert!(!has_errors(&d), "expected clean program, got:\n{}", error_msgs(&d));
}

#[test]
fn rejects_named_constructor_misuse() {
    // The body must be exactly one `this(...)` delegation, its arguments must
    // match the primary constructor, names must be unique, and `this(...)` is
    // not valid outside a named constructor.
    let d = check_str(
        r#"class A {
            x: int
            constructor(x: int) { this.x = x }
            constructor.extra() {
                this(1)
                println("side")
            }
            constructor.wrong() { this("nope") }
        }

        class B {
            constructor.dup() { this() }
            constructor.dup() { this() }
        }

        fn main() { this(1) }"#,
    );
    let msgs = error_msgs(&d);
    assert!(
        msgs.contains("named constructor body must be a single `this(...)` delegation"),
        "missing delegation-shape diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains("expected `int`, found `string`"),
        "missing delegation-argument diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains("duplicate member `dup`"),
        "missing duplicate named-constructor diagnostic, got:\n{msgs}"
    );
    assert!(
        msgs.contains("`this(...)` can only be used as a named constructor's delegation"),
        "missing stray-delegation diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_deinit_using_this() {
    // A `deinit` finalizer runs with `this` live and returns unit.
    let d = check_str(
        r#"class Widget {
            value: int

            deinit {
                println(this.value)
            }
        }

        fn main() {
            let w = Widget(7)
            println(w.value)
        }"#,
    );
    assert!(!has_errors(&d), "expected a clean program, got:\n{}", error_msgs(&d));
}

#[test]
fn rejects_deinit_unknown_member() {
    let d = check_str(
        r#"class Widget {
            value: int

            deinit {
                this.missing = 1
            }
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(msgs.contains("missing"), "expected a member diagnostic, got:\n{msgs}");
}

#[test]
fn rejects_duplicate_deinit() {
    let d = check_str(
        r#"class Widget {
            value: int

            deinit { println(1) }
            deinit { println(2) }
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(
        msgs.contains("already has a `deinit` block"),
        "missing duplicate-deinit diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_inheritance_and_hierarchy_casts() {
    // Inherited fields/methods, `super.m()`, subclass-to-superclass
    // assignability, and `is`/`as` across the hierarchy all type-check.
    let d = check_str(
        r#"class Animal {
            name: string
            legs: int = 4

            fn label() -> string {
                return this.name
            }
        }

        class Dog extends Animal {
            breed: string

            fn tag() -> string {
                return super.label()
            }
        }

        fn describe(a: Animal) -> string {
            return a.label()
        }

        fn main() {
            let d = Dog("Rex", "lab")
            println(d.legs)
            println(d.tag())
            println(describe(d))
            if (d is Dog) {
                println("dog")
            }
            if (d is Animal) {
                println("animal")
            }
            let a: Animal = d
            let back: Dog = a as Dog
            println(back.breed)
        }"#,
    );
    assert!(!has_errors(&d), "expected a clean program, got:\n{}", error_msgs(&d));
}

#[test]
fn accepts_manual_alloc_and_free() {
    let d = check_str(
        r#"class Widget {
            value: int
            constructor(v: int) {
                value = v
            }
        }

        fn main() {
            #[manualAlloc] let w = Widget(1)
            println(w.value)
            w.free()
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_free_on_managed_binding() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            let w = Widget(1)
            w.free()
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("manualAlloc"),
        "expected a manualAlloc hint, got:\n{msgs}"
    );
}

#[test]
fn rejects_double_free() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[manualAlloc] let w = Widget(1)
            w.free()
            w.free()
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("already freed"),
        "expected a double-free diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_use_after_free() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[manualAlloc] let w = Widget(1)
            w.free()
            println(w.value)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("after `free()`"),
        "expected a use-after-free diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_manual_alloc_of_non_class() {
    let d = check_str(
        r#"fn main() {
            #[manualAlloc] let n = 5
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("class or struct"),
        "expected a manual-alloc type diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_unknown_attribute() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[bogus] let w = Widget(1)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("unknown attribute"),
        "expected an unknown-attribute diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_manual_alloc_with_arguments() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[manualAlloc(8)] let w = Widget(1)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("takes no arguments"),
        "expected an argument diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_item_level_attributes() {
    let d = check_str(
        r#"#[manualAlloc]
        fn main() {
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("not lowered yet"),
        "expected an item-attribute diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_manual_move_between_bindings() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[manualAlloc] let a = Widget(1)
            #[manualAlloc] let b = a
            println(b.value)
            b.free()
        }"#,
    );
    assert!(!has_errors(&d), "unexpected errors:\n{}", error_msgs(&d));
}

#[test]
fn rejects_use_after_move() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[manualAlloc] let a = Widget(1)
            #[manualAlloc] let b = a
            println(a.value)
            b.free()
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("after it was moved"),
        "expected a use-after-move diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_manual_value_into_managed_binding() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[manualAlloc] let a = Widget(1)
            let b = a
            a.free()
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("managed binding"),
        "expected a managed-binding diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_storing_manual_value_in_managed_field() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        class Holder {
            var widget: Widget?
        }

        fn main() {
            #[manualAlloc] let a = Widget(1)
            let h = Holder(none)
            h.widget = a
            a.free()
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("managed field"),
        "expected a managed-field diagnostic, got:\n{msgs}"
    );
}

#[test]
fn accepts_free_in_both_if_branches() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main(flag: bool) {
            #[manualAlloc] let w = Widget(1)
            if (flag) {
                w.free()
            } else {
                w.free()
            }
        }"#,
    );
    assert!(
        !has_errors(&d),
        "freeing on both branches must not be a double free:\n{}",
        error_msgs(&d)
    );
}

#[test]
fn rejects_use_after_free_on_only_one_branch() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main(flag: bool) {
            #[manualAlloc] let w = Widget(1)
            if (flag) {
                w.free()
            }
            println(w.value)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("after `free()`") || msgs.contains("after it was moved"),
        "expected a consumption diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_returning_manual_from_managed_fn() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn leak() -> Widget {
            #[manualAlloc] let w = Widget(1)
            return w
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("does not own its result"),
        "expected an ownership diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_overwriting_live_manual_binding() {
    let d = check_str(
        r#"class Widget {
            value: int
        }

        fn main() {
            #[manualAlloc] var w = Widget(1)
            w = Widget(2)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(
        msgs.contains("overwrite") || msgs.contains("unknown attribute"),
        "expected an overwrite diagnostic, got:\n{msgs}"
    );
}

#[test]
fn rejects_unknown_member_in_subclass() {
    let d = check_str(
        r#"class Animal {
            name: string
        }

        class Dog extends Animal {
            breed: string
        }

        fn main() {
            let d = Dog("Rex", "lab")
            println(d.missing)
        }"#,
    );
    let msgs = error_msgs(&d);
    assert!(has_errors(&d), "expected an error, got none");
    assert!(msgs.contains("missing"), "expected a member diagnostic, got:\n{msgs}");
}
