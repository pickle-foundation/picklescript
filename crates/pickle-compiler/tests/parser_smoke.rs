use pickle_compiler::ast::{ClassMember, ItemKind, Program};
use pickle_compiler::diag::DiagnosticSink;
use pickle_compiler::lexer::lex;
use pickle_compiler::parser::parse;

fn parse_str(src: &str) -> Program {
    let diags = DiagnosticSink::new();
    let tokens = lex(pickle_compiler::diag::FileId(0), src, &diags);
    match parse(tokens, &diags) {
        Ok(p) => p,
        Err(_) => panic!("parse failed for:\n{src}"),
    }
}

#[test]
fn parses_fibonacci() {
    let p = parse_str(
        r#"fn fib(n: Int) -> Int {
            if (n < 2) {
                return n
            }
            return fib(n - 1) + fib(n - 2)
        }

        fn main() {
            let result = fib(10)
            print("fib(10) = {result}")
        }"#,
    );
    assert_eq!(p.items.len(), 2);
    assert!(matches!(&p.items[0].kind, ItemKind::Fn(f) if f.name == "fib"));
    assert!(matches!(&p.items[1].kind, ItemKind::Fn(f) if f.name == "main"));
}

#[test]
fn parses_class_with_members() {
    let p = parse_str(
        r#"class Counter {
            var count: Int = 0

            constructor(start: Int) {
                count = start
            }

            fn increment() {
                count = count + 1
            }

            property current: Int {
                get {
                    return count
                }
            }
        }"#,
    );
    match &p.items[0].kind {
        ItemKind::Class(c) => {
            assert_eq!(c.name, "Counter");
            let kinds: Vec<&str> = c
                .members
                .iter()
                .map(|m| match m {
                    ClassMember::Field { .. } => "field",
                    ClassMember::Constructor(_) => "constructor",
                    ClassMember::Method(_) => "method",
                    ClassMember::Property(_) => "property",
                    _ => "other",
                })
                .collect();
            assert_eq!(kinds, vec!["field", "constructor", "method", "property"]);
        }
        other => panic!("expected class, got {other:?}"),
    }
}

#[test]
fn parses_match_with_guards_and_multi_pattern() {
    let p = parse_str(
        r#"fn describe(x: Int?) -> String {
            return match (x) {
                case none -> "nothing"
                case n -> "value {n}"
                case 0, 1 -> "small"
                case y if y > 100 -> "big"
            }
        }"#,
    );
    match &p.items[0].kind {
        ItemKind::Fn(f) => {
            let body = f.body.as_ref().expect("fn body");
            let stmts = match body {
                pickle_compiler::ast::FnBody::Block(b) => &b.stmts,
                pickle_compiler::ast::FnBody::Expr(_) => panic!("expected block body"),
            };
            assert_eq!(stmts.len(), 1);
        }
        other => panic!("expected fn, got {other:?}"),
    }
}

#[test]
fn parses_generics_and_interfaces() {
    let p = parse_str(
        r#"interface Drawable<T> {
            fn draw(ctx: Graphics)
            property width: Int
        }

        class Circle<T> extends Shape implements Drawable<T> {
            constructor(radius: T) {
                super(radius)
            }

            fn draw(ctx: Graphics) {
                ctx.render(self)
            }
        }"#,
    );
    let items = &p.items;
    assert!(matches!(items[0].kind, ItemKind::Interface(_)));
    match &items[0].kind {
        ItemKind::Interface(i) => {
            assert_eq!(i.generics.len(), 1);
            use pickle_compiler::ast::InterfaceMember;
            assert!(matches!(i.members[0], InterfaceMember::Method(_)));
            assert!(matches!(i.members[1], InterfaceMember::Property { .. }));
        }
        _ => {}
    }
    assert!(matches!(items[1].kind, ItemKind::Class(_)));
}

#[test]
fn lexes_unterminated_string_without_hang() {
    let diags = DiagnosticSink::new();
    let tokens = lex(
        pickle_compiler::diag::FileId(0),
        r#"fn main() { print("oops) }"#,
        &diags,
    );
    assert!(!tokens.is_empty());
}

#[test]
fn parses_loops() {
    let p = parse_str(
        r#"fn sum(n: Int) -> Int {
            var total = 0
            var i = 0
            while (i < n) {
                total = total + i
                i = i + 1
            }
            for (let j = 0; j < n; j = j + 1) {
                total = total + j
            }
            for (item in items) {
                total = total + item
            }
            return total
        }"#,
    );
    assert_eq!(p.items.len(), 1);
}

#[test]
fn parses_struct_enum_import() {
    use pickle_compiler::ast::{ImportKind, ItemKind, StructDecl};
    let p = parse_str(
        r#"module game

        import geometry
        import io as iolib

        struct Vec2 {
            var x: Float
            var y: Float
        }

        enum Color {
            case Red
            case Green
            case Blue
            case Rgb(r: Int, g: Int, b: Int)
        }

        fn area(v: Vec2) -> Float {
            return v.x * v.y
        }"#,
    );
    assert_eq!(p.module.as_ref().unwrap().path, vec!["game"]);
    assert_eq!(p.imports.len(), 2);
    match &p.imports[0].kind {
        ImportKind::Module { path, alias } => {
            assert_eq!(path, &vec!["geometry".to_string()]);
            assert!(alias.is_none());
        }
        _ => panic!("expected module import"),
    }
    assert!(matches!(&p.items[0].kind, ItemKind::Struct(StructDecl { .. })));
    match &p.items[1].kind {
        ItemKind::Enum(e) => {
            assert_eq!(e.variants.len(), 4);
            assert_eq!(e.variants[3].name, "Rgb");
            assert_eq!(e.variants[3].fields.len(), 3);
        }
        other => panic!("expected enum, got {other:?}"),
    }
    assert!(matches!(&p.items[2].kind, ItemKind::Fn(f) if f.name == "area"));
}

#[test]
fn parses_generics_fn_and_lambda() {
    use pickle_compiler::ast::ItemKind;
    let p = parse_str(
        r#"fn map<A, B>(xs: List<A>, f: fn (A) -> B) -> List<B> {
            var out = List<B>()
            out.reserve(xs.length)
            for (x in xs) {
                out.push(f(x))
            }
            return out
        }

        fn use_map() {
            let double = fn (x: Int) -> Int {
                return x * 2
            }
            let ys = map([1, 2, 3], double)
        }"#,
    );
    assert_eq!(p.items.len(), 2);
    match &p.items[0].kind {
        ItemKind::Fn(f) => {
            assert_eq!(f.generics.len(), 2);
            assert!(matches!(&f.params[0].ty, Some(t) if t.span.end > 0));
            assert!(f.return_ty.is_some());
        }
        _ => {}
    }
}

#[test]
fn parses_while_and_operators() {
    use pickle_compiler::ast::ItemKind;
    let p = parse_str(
        r#"class Fraction {
            var numerator: Int
            var denominator: Int

            operator + (other: Fraction) -> Fraction {
                return 1
            }

            operator == (other: Fraction) -> Bool {
                return false
            }
        }"#,
    );
    match &p.items[0].kind {
        ItemKind::Class(c) => {
            let methods = c
                .members
                .iter()
                .filter_map(|m| match m {
                    ClassMember::Method(m) => Some(m),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(methods.len(), 2);
            assert_eq!(methods[0].operator.as_deref(), Some("+"));
            assert_eq!(methods[1].operator.as_deref(), Some("=="));
        }
        other => panic!("expected class, got {other:?}"),
    }
}