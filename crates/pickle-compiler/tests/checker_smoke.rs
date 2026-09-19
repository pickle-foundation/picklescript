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
