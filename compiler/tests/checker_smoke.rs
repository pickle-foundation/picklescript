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
