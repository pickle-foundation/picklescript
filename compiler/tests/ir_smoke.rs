//! Smoke tests for PickleIR lowering: the emitter must produce a sane module
//! (functions, blocks, externs, string pool) for slice-1 programs.
//!
//! These programs must pass resolve + type-check, since `emit_ir` aborts on
//! diagnostics; a panic here means front-end bugs as much as emitter bugs.

use pickle_compiler::diag::{DiagnosticSink, SourceMap};
use pickle_compiler::emit::emit_ir;
use pickle_compiler::front::frontend;
use pickle_compiler::ir::{BinOp, Callee, IrInstr, IrModule, IrTerm};

fn emit_str(src: &str) -> IrModule {
    let mut map = SourceMap::default();
    let diags = DiagnosticSink::new();
    let out = frontend("test.pkl", src, &mut map, &diags).expect("frontend failed");
    emit_ir(&out.program, &out.resolved, &diags).unwrap_or_else(|| {
        panic!("emit failed:\n{}", diags.render_all(&map, false))
    })
}

#[test]
fn emits_arithmetic_main() {
    let m = emit_str(
        r#"fn main() {
            let x = 2 + 3
            println("hello")
        }"#,
    );
    assert_eq!(m.funcs.len(), 1);
    let main = &m.funcs[0];
    assert!(main.is_main);
    assert_eq!(main.symbol, "pickle_main");
    // The tail is a discarded statement block, so the entry block ends in a
    // plain `return`.
    assert!(main
        .blocks
        .iter()
        .any(|b| matches!(b.term, IrTerm::Return { v: None })));
    let dump = format!("{m}");
    assert!(dump.contains("binop.add"), "dump:\n{dump}");
    assert!(m.strings.iter().any(|s| s == b"hello"), "string pool: {:?}\ndump:\n{m}", m.strings);
    let symbols: Vec<&str> = m.externs.iter().map(|e| e.symbol.as_str()).collect();
    assert!(symbols.contains(&"pickle_print_obj"));
    assert!(symbols.contains(&"pickle_print_newline"));
}

#[test]
fn emits_user_function_and_calls() {
    let m = emit_str(
        r#"fn add(a: int, b: int) -> int {
            a + b
        }

        fn main() {
            let v = add(1, 2)
        }"#,
    );
    assert_eq!(m.funcs.len(), 2);
    let add = m.funcs.iter().find(|f| f.name == "add").expect("add");
    assert_eq!(add.symbol, "pkl_add");
    assert_eq!(add.params.len(), 2);
    assert_eq!(add.ret.to_string(), "int64");
    let main = m.funcs.iter().find(|f| f.name == "main").expect("main");
    let call_in_main = main.blocks.iter().any(|b| {
        b.instrs.iter().any(|i| {
            matches!(
                i,
                IrInstr::Call { callee: Callee::Func(fid), .. } if fid.0 == add_id(&m)
            )
        })
    });
    assert!(call_in_main, "main must call `add`, dump:\n{m}");
}

fn add_id(m: &IrModule) -> usize {
    m.funcs.iter().position(|f| f.name == "add").unwrap()
}

#[test]
fn emits_if_expression() {
    let m = emit_str(
        r#"fn pick(c: bool) -> int {
            if (c) { 1 } else { 2 }
        }"#,
    );
    let pick = m.funcs.iter().find(|f| f.name == "pick").expect("pick");
    // entry + then + else + join
    assert_eq!(pick.blocks.len(), 4, "dump:\n{m}");
    assert_eq!(pick.slots.len(), 2, "param c plus the if result slot");
    let dump = format!("{m}");
    assert!(dump.contains("branchif"), "dump:\n{dump}");
    // The result flows through a slot: both branches store, join loads.
    let stores = pick
        .blocks
        .iter()
        .flat_map(|b| &b.instrs)
        .filter(|i| matches!(i, IrInstr::StoreSlot { .. }))
        .count();
    assert_eq!(stores, 2, "dump:\n{dump}");
}

#[test]
fn emits_while_loop() {
    let m = emit_str(
        r#"fn count(n: int) -> int {
            var i = 0
            var total = 0
            while (i < n) {
                total = total + i
                i = i + 1
            }
            return total
        }"#,
    );
    let count = m.funcs.iter().find(|f| f.name == "count").expect("count");
    let branches = count
        .blocks
        .iter()
        .filter(|b| matches!(b.term, IrTerm::BranchIf { .. }))
        .count();
    assert!(branches >= 1, "loop needs a branchif, dump:\n{count}");
    // The last block returns the accumulated total.
    assert!(
        count
            .blocks
            .iter()
            .any(|b| matches!(b.term, IrTerm::Return { v: Some(_) })),
        "expected an explicit return, dump:\n{count}"
    );
}

#[test]
fn emits_for_in_range() {
    let m = emit_str(
        r#"fn sum(n: int) -> int {
            var total = 0
            for (x in 0..n) {
                total = total + x
            }
            return total
        }"#,
    );
    let sum = m.funcs.iter().find(|f| f.name == "sum").expect("sum");
    // The loop var is bound to the idx slot; the loop has cond/body/end blocks.
    assert!(sum.blocks.len() >= 3, "dump:\n{sum}");
    let dump = format!("{sum}");
    assert!(dump.contains("binop.lt"), "dump:\n{dump}");
}

#[test]
fn emits_string_concat_and_cmp() {
    let m = emit_str(
        r#"fn hi(name: string) -> string {
            "hi " + name
        }

        fn same(a: string, b: string) -> bool {
            a == b
        }"#,
    );
    let symbols: Vec<&str> = m.externs.iter().map(|e| e.symbol.as_str()).collect();
    assert!(symbols.contains(&"pickle_str_concat"), "{:?}", symbols);
    assert!(symbols.contains(&"pickle_str_cmp"), "{:?}", symbols);
    let dump = format!("{m}");
    assert!(dump.contains("binop.eq"), "dump:\n{dump}");
}

#[test]
fn emits_trailing_expr_after_newline_as_return() {
    // Regression: `expr` followed by a newline before `}` was parsed as a
    // discarded statement instead of the block tail, so a value-returning
    // function ended in `unreachable` (a fatal JIT trap at runtime).
    let m = emit_str(
        r#"fn add(a: int, b: int) -> int {
            a + b
        }

        fn main() {
            println(add(1, 2))
        }"#,
    );
    let add = m.funcs.iter().find(|f| f.name == "add").expect("add");
    assert!(
        add.blocks
            .iter()
            .any(|b| matches!(b.term, IrTerm::Return { v: Some(_) })),
        "the tail expression must feed the return, dump:\n{m}"
    );
    assert!(
        !add.blocks.iter().any(|b| matches!(b.term, IrTerm::Unreachable)),
        "no dead tail may remain, dump:\n{m}"
    );
}

#[test]
fn emits_for_continue_increments() {
    let m = emit_str(
        r#"fn take_odds(n: int) -> int {
            var total = 0
            for (x in 0..n) {
                if (x == 2) {
                    continue
                }
                total = total + x
            }
            return total
        }"#,
    );
    let f = m.funcs.iter().find(|g| g.name == "take_odds").expect("fn");
    let cond = f
        .blocks
        .iter()
        .find(|b| matches!(b.term, IrTerm::BranchIf { .. }))
        .expect("loop condition block");
    // `continue` must land in an increment block: it bumps the loop var and
    // then re-enters the condition. Jumping straight to the condition skips
    // the increment and loops forever.
    let inc = f.blocks.iter().find(|b| {
        if let IrTerm::Branch { target } = &b.term {
            *target == cond.id
                && b.instrs
                    .iter()
                    .any(|i| matches!(i, IrInstr::BinOp { op: BinOp::Add, .. }))
        } else {
            false
        }
    });
    assert!(inc.is_some(), "missing increment block, dump:\n{f}");
}

#[test]
fn emits_empty_string_literal() {
    let m = emit_str(
        r#"fn blank() -> string {
            ""
        }

        fn main() {
            println(blank())
        }"#,
    );
    assert!(
        m.strings.iter().any(|s| s.is_empty()),
        "the empty string must land in the pool, dump:\n{m}"
    );
}

#[test]
fn emits_short_circuit_and_or() {
    let m = emit_str(
        r#"fn both(a: bool, b: bool) -> bool {
            a && b
        }

        fn either(a: bool, b: bool) -> bool {
            a || b
        }"#,
    );
    let both = m.funcs.iter().find(|f| f.name == "both").expect("both");
    // && and || each materialize a const for the short-circuit value.
    let dump = format!("{m}");
    assert!(dump.contains("const false"), "dump:\n{dump}");
    assert!(dump.contains("const true"), "dump:\n{dump}");
    // &&: rhs block + short block + join, beyond the entry.
    assert!(both.blocks.len() >= 4, "dump:\n{m}");
}