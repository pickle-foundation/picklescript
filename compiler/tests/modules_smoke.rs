//! The module system: `import`/`use` loading, qualified references, aliases,
//! and the ambiguity diagnostic for cross-module name collisions.

use pickle_compiler::diag::{DiagnosticSink, SourceMap};
use pickle_compiler::error::ErrorCode;
use pickle_compiler::front::frontend_checked;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/modules/{name}", env!("CARGO_MANIFEST_DIR"))
}

fn check_file(path: &str) -> (Option<bool>, DiagnosticSink, SourceMap) {
    let source = std::fs::read_to_string(path).expect("fixture exists");
    let diags = DiagnosticSink::new();
    let mut map = SourceMap::default();
    let ok = frontend_checked(path, &source, &mut map, &diags);
    (ok.map(|_| true), diags, map)
}

fn codes(diags: &DiagnosticSink) -> Vec<ErrorCode> {
    diags
        .diagnostics
        .borrow()
        .iter()
        .filter_map(|d| d.code)
        .collect()
}

fn has_code(diags: &DiagnosticSink, code: ErrorCode) -> bool {
    codes(diags).contains(&code)
}

// ---- the three import forms + the `use ... as` alias ----------------------

#[test]
fn imports_and_aliases_compile_clean() {
    let (ok, diags, _map) = check_file(&fixture("main.pkl"));
    assert!(ok.is_some(), "main.pkl should frontend-clean");
    assert_eq!(codes(&diags), Vec::new(), "no diagnostics expected");
}

// ---- module path mismatch -------------------------------------------------

#[test]
fn module_path_mismatch_is_reported() {
    let (ok, diags, _map) = check_file(&fixture("mismatch_main.pkl"));
    assert!(ok.is_some(), "loader continues after module errors");
    assert!(
        has_code(&diags, ErrorCode::ModuleMismatch),
        "expected a ModuleMismatch diagnostic, got {:?}",
        codes(&diags)
    );
}

// ---- a bare collided name must be disambiguated with an alias -------------

#[test]
fn ambiguous_import_names_both_providers() {
    let (ok, diags, map) = check_file(&fixture("ambiguous_main.pkl"));
    assert!(ok.is_some(), "loader continues after collision errors");
    assert!(
        has_code(&diags, ErrorCode::AmbiguousImport),
        "expected an AmbiguousImport diagnostic, got {:?}",
        codes(&diags)
    );
    let rendered = diags.render_all(&map, false);
    assert!(
        rendered.contains("provided by module `math.ops` and module `text.format`"),
        "provider note missing in:\n{rendered}"
    );
    assert!(
        rendered.contains("use `math.ops.helper as helperA`"),
        "alias suggestion missing in:\n{rendered}"
    );
}

// ---- aliasing resolves the same collision --------------------------------

#[test]
fn aliased_collision_compiles_clean() {
    let (ok, diags, _map) = check_file(&fixture("aliased_main.pkl"));
    assert!(ok.is_some(), "aliases should disambiguate providers");
    assert_eq!(codes(&diags), Vec::new(), "no diagnostics expected");
}

// ---- a missing module path is reported under ModuleNotFound --------------

#[test]
fn missing_module_is_reported() {
    let path = fixture("missing_main.pkl");
    std::fs::write(
        &path,
        "import does.not.exist\n\nfn main() {}\n",
    )
    .expect("write fixture");
    let (ok, diags, _map) = check_file(&path);
    let _ = std::fs::remove_file(&path);
    assert!(ok.is_some(), "loader continues after module errors");
    assert!(
        has_code(&diags, ErrorCode::ModuleNotFound),
        "expected a ModuleNotFound diagnostic, got {:?}",
        codes(&diags)
    );
}

// ---- a `use` of a name the module does not export -------------------------

#[test]
fn unexported_item_is_reported() {
    let path = fixture("unexported_main.pkl");
    std::fs::write(
        &path,
        "use math.ops.nope\n\nfn main() {}\n",
    )
    .expect("write fixture");
    let (ok, diags, _map) = check_file(&path);
    let _ = std::fs::remove_file(&path);
    assert!(ok.is_some(), "loader continues after module errors");
    assert!(
        has_code(&diags, ErrorCode::NotExported),
        "expected a NotExported diagnostic, got {:?}",
        codes(&diags)
    );
}

// ---- module decls without a matching file still work as plain sources -----

#[test]
fn module_decl_single_file_still_compiles() {
    let source = "module hello.world\n\nfn greet() -> int { return 5 }\n";
    let diags = DiagnosticSink::new();
    let mut map = SourceMap::default();
    let out = frontend_checked("hello_world.pkl", source, &mut map, &diags);
    assert!(out.is_some());
    assert_eq!(codes(&diags), Vec::new());
}