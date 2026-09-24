//! Tests for the history surface: extraction, diffs, JSON round-trip, and
//! the hash. The shapes here lock the `.pickle/history/*.db` format so old
//! caches keep parsing and new surfaces keep diffing.

use pickle_compiler::ast::{TypeExpr, TypeExprKind};
use pickle_compiler::diag::{DiagnosticSink, FileId, SourceMap, Span};
use pickle_compiler::front::frontend;
use pickle_compiler::history::{
    diff_surfaces, extract_surface, fnv1a64, transit_label, type_expr_to_string, PublicItem,
    Snapshot,
};

fn span() -> Span {
    Span::new(FileId(0), 0, 0)
}

fn ty(kind: TypeExprKind) -> TypeExpr {
    TypeExpr { span: span(), kind }
}

fn parse_program(src: &str) -> pickle_compiler::ast::Program {
    let diags = DiagnosticSink::new();
    let mut map = SourceMap::default();
    let out =
        frontend("test.pk", src, &mut map, &diags).expect("parse should succeed for the fixture");
    out.program
}

fn check_str(src: &str) -> bool {
    let diags = DiagnosticSink::new();
    let mut map = SourceMap::default();
    pickle_compiler::front::frontend_checked("test.pk", src, &mut map, &diags).is_some()
}

const USER_V1: &str = r#"class User {
    name: string
    age: int

    constructor(name: string, age: int) {
        this.name = name
        this.age = age
    }

    fn greet() -> string {
        return this.name
    }
}

fn main() {
    let u = User("ada", 120)
    println(u.greet())
}"#;

const USER_V2: &str = r#"class User {
    name: string?
    age: int
    admin: bool

    constructor(name: string, age: int) {
        this.name = name
        this.age = age
    }

    fn greet() -> string {
        return this.name
    }
}

fn main() {
    let u = User("ada", 120)
    println(u.greet())
}"#;

#[test]
fn type_expr_renders_all_kinds() {
    assert_eq!(
        type_expr_to_string(&ty(TypeExprKind::Path(vec!["a".into(), "b".into()]))),
        "a.b"
    );
    assert_eq!(
        type_expr_to_string(&ty(TypeExprKind::Generic(
            Box::new(ty(TypeExprKind::Path(vec!["Map".into()]))),
            vec![
                ty(TypeExprKind::Path(vec!["string".into()])),
                ty(TypeExprKind::Generic(
                    Box::new(ty(TypeExprKind::Path(vec!["List".into()]))),
                    vec![ty(TypeExprKind::Option(Box::new(ty(TypeExprKind::Path(
                        vec!["int".into()]
                    )))))]
                ))
            ]
        ))),
        "Map<string, List<int?>>"
    );
    assert_eq!(
        type_expr_to_string(&ty(TypeExprKind::Pointer(Box::new(ty(
            TypeExprKind::Path(vec!["T".into()])
        ))))),
        "*T"
    );
    assert_eq!(
        type_expr_to_string(&ty(TypeExprKind::Ref(Box::new(ty(TypeExprKind::Path(
            vec!["T".into()]
        )))))),
        "&T"
    );
    assert_eq!(
        type_expr_to_string(&ty(TypeExprKind::Tuple(vec![
            ty(TypeExprKind::Path(vec!["int".into()])),
            ty(TypeExprKind::Path(vec!["string".into()])),
        ]))),
        "(int, string)"
    );
    assert_eq!(
        type_expr_to_string(&ty(TypeExprKind::Fn {
            params: vec![ty(TypeExprKind::Path(vec!["int".into()]))],
            ret: Some(Box::new(ty(TypeExprKind::Path(vec!["bool".into()])))),
            is_async: true,
        })),
        "async (int) -> bool"
    );
    assert_eq!(type_expr_to_string(&ty(TypeExprKind::Infer)), "_");
}

#[test]
fn surface_extraction_has_flat_membership() {
    let p = parse_program(USER_V1);
    assert!(check_str(USER_V1), "fixture must type-check");
    let items = extract_surface(&p);
    let names: Vec<String> = items
        .iter()
        .map(|i| format!("{}:{}", i.kind, i.name))
        .collect();
    assert!(
        names.contains(&"class:User".to_string()),
        "missing class: {names:?}"
    );
    assert!(
        names.contains(&"field:User.name".to_string()),
        "missing field"
    );
    assert!(
        names.contains(&"field:User.age".to_string()),
        "missing field"
    );
    assert!(
        names.contains(&"ctor:User.<init>".to_string()),
        "missing ctor"
    );
    assert!(
        names.contains(&"method:User.greet".to_string()),
        "missing method"
    );
    assert!(names.contains(&"fn:main".to_string()), "missing fn");
    let name = items
        .iter()
        .find(|i| i.kind == "field" && i.name == "User.name")
        .unwrap();
    assert_eq!(name.sig, "name: string");
    // Sorted by (kind, name).
    for w in items.windows(2) {
        assert!(
            (w[0].kind, &w[0].name) <= (w[1].kind, &w[1].name),
            "surface must be sorted: {:?} after {:?}",
            w[1],
            w[0]
        );
    }
}

#[test]
fn surface_covers_enums_interfaces_structs_consts() {
    let src = r#"enum Color {
        case Red
        case Rgb(r: int, g: int, b: int)
    }

    interface Runnable {
        fn run() -> string
    }

    struct Point {
        var x: int
        var y: int
    }

    const VERSION: int = 1
"#;
    let p = parse_program(src);
    let items = extract_surface(&p);
    let names: Vec<String> = items
        .iter()
        .map(|i| format!("{}:{}", i.kind, i.name))
        .collect();
    for want in [
        "enum:Color",
        "variant:Color.Red",
        "variant:Color.Rgb",
        "interface:Runnable",
        "imethod:Runnable.run",
        "struct:Point",
        "field:Point.x",
        "field:Point.y",
        "const:VERSION",
    ] {
        assert!(
            names.contains(&want.to_string()),
            "missing {want}: {names:?}"
        );
    }
    let rgb = items
        .iter()
        .find(|i| i.kind == "variant" && i.name == "Color.Rgb")
        .unwrap();
    assert_eq!(rgb.sig, "(r: int, g: int, b: int)");
    let cst = items
        .iter()
        .find(|i| i.kind == "const" && i.name == "VERSION")
        .unwrap();
    assert_eq!(cst.sig, "VERSION: int");
}

#[test]
fn diff_reports_add_remove_change() {
    let before = extract_surface(&parse_program(USER_V1));
    let after = extract_surface(&parse_program(USER_V2));
    let changes = diff_surfaces(&before, &after);
    let changed: Vec<String> = changes
        .iter()
        .filter(|c| c.status == "~")
        .map(|c| c.name.clone())
        .collect();
    assert_eq!(
        changed,
        vec!["User.name".to_string()],
        "only name became nullable: {changes:?}"
    );
    let change = changes.iter().find(|c| c.status == "~").unwrap();
    assert_eq!(change.before.as_deref(), Some("name: string"));
    assert_eq!(change.after.as_deref(), Some("name: string?"));
    assert!(changes
        .iter()
        .any(|c| c.status == "+" && c.name == "User.admin"));
    assert!(
        !changes.iter().any(|c| c.status == "-"),
        "nothing removed: {changes:?}"
    );
}

#[test]
fn transit_labels_are_honest() {
    assert_eq!(
        transit_label("name: string", "name: string?"),
        Some("made nullable")
    );
    assert_eq!(
        transit_label("name: string?", "name: string"),
        Some("no longer optional")
    );
    assert_eq!(transit_label("a: int", "a: float"), None);
    // Wrapping in `?` only: still honestly nullable, whatever the payload.
    assert_eq!(
        transit_label("xs: List<int>", "xs: List<int>?"),
        Some("made nullable")
    );
    // A `?` that moves around (payload also changed) is not named.
    assert_eq!(transit_label("xs: List<int?>", "xs: List<int>?"), None);
    // A rename that also flips nullability is not "made nullable".
    assert_eq!(transit_label("a: int", "b: int?"), None);
}

#[test]
fn snapshot_json_round_trips() {
    let items = vec![
        PublicItem {
            kind: "field",
            name: "User.name".into(),
            vis: "def",
            sig: "name: string?".into(),
        },
        PublicItem {
            kind: "fn",
            name: "say\"hi\"_\u{2603}".into(),
            vis: "pub",
            sig: "say\"hi\"(s: string) -> string".into(),
        },
    ];
    let snap = Snapshot {
        commit: "0123456789abcdef0123456789abcdef01234567".into(),
        short: "0123456".into(),
        date: "2026-09-20 10:00:00 +0000".into(),
        message: "make User.name optional\n(with \"escapes\" and unicode \u{2603})".into(),
        file: "src/User.pkl".into(),
        hash: fnv1a64(b"contents of file"),
        ok: true,
        codes: vec![],
        deps: vec!["import a.b as c".into(), "use x.y.z".into()],
        items: items.clone(),
    };
    let json = snap.to_json();
    let back = Snapshot::from_json(&json).expect("round-trip must parse");
    assert_eq!(back, snap);

    // A malformed line must fail closed, not fabricate a snapshot.
    assert!(Snapshot::from_json("not json").is_none());
    assert!(Snapshot::from_json(r#"{"commit":1}"#).is_none());
}

#[test]
fn snapshot_json_with_errors_and_unknown_kind() {
    let snap = Snapshot {
        commit: "abc".into(),
        short: "abc".into(),
        date: "d".into(),
        message: "m".into(),
        file: "f.pkl".into(),
        hash: fnv1a64(b"x"),
        ok: false,
        codes: vec!["E0308".into(), "E0351".into()],
        deps: vec![],
        items: vec![PublicItem {
            kind: "field",
            name: "A.b".into(),
            vis: "priv",
            sig: "b: int".into(),
        }],
    };
    let json = snap.to_json();
    let back = Snapshot::from_json(&json).unwrap();
    assert!(!back.ok);
    assert_eq!(back.codes, vec!["E0308".to_string(), "E0351".to_string()]);
    assert_eq!(back.items[0].vis, "priv");

    // A cache line with an unknown kind decodes by skipping that item.
    let tampered = json.replace("\"kind\":\"field\"", "\"kind\":\"wat\"");
    let back = Snapshot::from_json(&tampered).unwrap();
    assert!(back.items.is_empty());
}

#[test]
fn fnv1a64_is_deterministic_and_differs() {
    assert_eq!(fnv1a64(b"hello"), fnv1a64(b"hello"));
    assert_ne!(fnv1a64(b"hello"), fnv1a64(b"hellp"));
    assert_eq!(fnv1a64(b""), format!("{:016x}", 0xcbf29ce484222325u64));
}
