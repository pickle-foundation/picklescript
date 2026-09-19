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
