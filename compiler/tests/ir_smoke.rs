//! Smoke tests for PickleIR lowering: the emitter must produce a sane module
//! (functions, blocks, externs, string pool) for slice-1 programs.
//!
//! These programs must pass resolve + type-check, since `emit_ir` aborts on
//! diagnostics; a panic here means front-end bugs as much as emitter bugs.

use pickle_compiler::diag::{DiagnosticSink, SourceMap};
use pickle_compiler::emit::emit_ir;
use pickle_compiler::front::frontend;
use pickle_compiler::ir::{BinOp, Callee, IrConst, IrInstr, IrModule, IrTerm, IrTy};

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

// ---- lists (slice 1.5) ----

fn externs(m: &IrModule) -> Vec<String> {
    m.externs.iter().map(|e| e.symbol.clone()).collect()
}

#[test]
fn emits_array_literal_materializes_list() {
    let m = emit_str(
        r#"fn main() {
            let xs = [1, 2, 3]
            println(len(xs))
        }"#,
    );
    let syms = externs(&m);
    assert!(syms.iter().any(|s| s == "pickle_list_new"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_list_push"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_box_i64"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_list_len"), "externs: {syms:?}");
}

#[test]
fn emits_index_read_unboxes() {
    let m = emit_str(
        r#"fn head(xs: List<int>) -> int {
            xs[0]
        }"#,
    );
    let syms = externs(&m);
    assert!(syms.iter().any(|s| s == "pickle_list_get"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_unbox_i64"), "externs: {syms:?}");
}

#[test]
fn emits_index_assign_boxes() {
    let m = emit_str(
        r#"fn put(xs: List<int>, v: int) {
            xs[0] = v
        }"#,
    );
    let syms = externs(&m);
    assert!(syms.iter().any(|s| s == "pickle_list_set"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_box_i64"), "externs: {syms:?}");
}

#[test]
fn emits_list_methods() {
    let m = emit_str(
        r#"fn main() {
            var xs = [1]
            xs.push(2)
            let last = xs.pop()
            println(last)
        }"#,
    );
    let syms = externs(&m);
    assert!(syms.iter().any(|s| s == "pickle_list_push"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_list_pop"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_box_i64"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_unbox_i64"), "externs: {syms:?}");
}

#[test]
fn emits_for_in_list_with_get_and_unbox() {
    let m = emit_str(
        r#"fn sum(xs: List<int>) -> int {
            var total = 0
            for (x in xs) {
                total = total + x
            }
            return total
        }"#,
    );
    let syms = externs(&m);
    assert!(syms.iter().any(|s| s == "pickle_list_get"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_unbox_i64"), "externs: {syms:?}");
    let sum = m.funcs.iter().find(|f| f.name == "sum").expect("sum");
    let cond = sum
        .blocks
        .iter()
        .find(|b| matches!(b.term, IrTerm::BranchIf { .. }))
        .expect("loop condition block");
    assert!(
        sum.blocks.iter().any(|b| {
            if let IrTerm::Branch { target } = &b.term {
                *target == cond.id
                    && b.instrs.iter().any(|i| {
                        if let IrInstr::Call { callee: Callee::Extern(ex), args, .. } = i {
                            m.externs.get(ex.0).map(|e| e.symbol.as_str()) == Some("pickle_list_len")
                                && args.len() == 1
                        } else {
                            false
                        }
                    })
            } else {
                false
            }
        }),
        "the for-loop must bound iterations by `pickle_list_len`, dump:\n{sum}"
    );
}

#[test]
fn emits_char_lists_in_codegen() {
    // `char` elements box through `pickle_box_char`/`pickle_unbox_char`,
    // same as the other scalars, so a char list lowers to real IR.
    let m = emit_str(
        r#"fn main() {
            let cs = ['a', 'b']
            println(cs[1])
        }"#,
    );
    let externs: Vec<&str> = m.externs.iter().map(|e| e.symbol.as_str()).collect();
    assert!(externs.contains(&"pickle_box_char"), "externs: {externs:?}");
    assert!(externs.contains(&"pickle_unbox_char"), "externs: {externs:?}");
}

#[test]
fn emits_nested_generic_lists() {
    // Nesting is spelled with a single `>>` token in the source; the parser
    // splits it so `List<List<int>>` resolves, and the emitter passes the
    // outer element (the inner list pointer) through without boxing.
    let m = emit_str(
        r#"fn probe(grid: List<List<int>>) -> int {
            let first = grid[0]
            let second = grid[1]
            let a = first[0]
            let b = second[1]
            return a + b
        }

        fn main() {
            let grid = [[1, 2], [3, 4]]
            println(probe(grid))
        }"#,
    );
    let syms = externs(&m);
    assert!(syms.iter().any(|s| s == "pickle_list_new"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_list_push"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_list_get"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_unbox_i64"), "externs: {syms:?}");
    let probe = m.funcs.iter().find(|f| f.name == "probe").expect("probe");
    assert!(
        probe
            .blocks
            .iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| matches!(i, IrInstr::Call { callee: Callee::Extern(ex), .. }
                if m.externs.get(ex.0).map(|e| e.symbol.as_str()) == Some("pickle_list_get")))
            .count()
            >= 2,
        "a nested read needs two `pickle_list_get` calls, dump:\n{probe}"
    );
}

#[test]
fn emits_enum_construct_and_match() {
    // `Enum.Variant(...)` lowers to `pickle_enum_new` + `pickle_enum_set_field`
    // (boxing scalar payloads); `match` lowers to `pickle_enum_tag` checks with
    // `pickle_enum_field` payload extraction. A chain that ends without a
    // catch-all arm calls `pickle_panic_no_match`.
    let m = emit_str(
        r#"enum Color {
            Red
            Blue
            Rgb(r: int, g: int, b: int)
        }

        fn describe(c: Color) -> int {
            match (c) {
                case Color.Red -> 0
                case Color.Blue -> 1
                case Color.Rgb(r, g, b) -> r + g + b
            }
        }

        fn other(c: Color) -> int {
            match (c) {
                case Color.Red -> 10
                case c2 -> 20
            }
        }

        fn main() {
            let c = Color.Rgb(1, 2, 3)
            println(describe(c), other(c))
        }"#,
    );
    let syms = externs(&m);
    for need in [
        "pickle_enum_new",
        "pickle_enum_set_field",
        "pickle_enum_tag",
        "pickle_enum_field",
        "pickle_panic_no_match",
    ] {
        assert!(syms.iter().any(|s| s == need), "missing {need}, externs: {syms:?}");
    }
    assert!(syms.iter().any(|s| s == "pickle_box_i64"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_unbox_i64"), "externs: {syms:?}");

    let describe = m.funcs.iter().find(|f| f.name == "describe").expect("describe");
    let checks = describe
        .blocks
        .iter()
        .filter(|b| matches!(b.term, IrTerm::BranchIf { .. }))
        .count();
    assert!(checks >= 3, "three variant arms need three tag checks, dump:\n{describe}");
    let field_reads = describe
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter(|i| matches!(i, IrInstr::Call { callee: Callee::Extern(ex), .. }
            if m.externs.get(ex.0).map(|e| e.symbol.as_str()) == Some("pickle_enum_field")))
        .count();
    assert!(
        field_reads >= 3,
        "binding r, g, b needs three payload reads, dump:\n{describe}"
    );

    let other = m.funcs.iter().find(|f| f.name == "other").expect("other");
    let other_checks = other
        .blocks
        .iter()
        .filter(|b| matches!(b.term, IrTerm::BranchIf { .. }))
        .count();
    assert_eq!(
        other_checks, 1,
        "only the variant arm is tested; the binding arm is a catch-all, dump:\n{other}"
    );
    assert!(
        other
            .blocks
            .iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| matches!(i, IrInstr::StoreSlot { .. })),
        "the binding arm stores the scrutinee into a slot, dump:\n{other}"
    );
}

#[test]
fn emits_map_operations() {
    // Map literals lower to `pickle_map_new` + `pickle_map_set`; reads go
    // through `pickle_map_get_boxed`; methods/len/iteration use their externs.
    let m = emit_str(
        r#"fn lookup(m: Map<string, int>) -> int {
            let a = m["x"]
            return a + len(m)
        }

        fn main() {
            var m = {"alpha": 10, "beta": 20}
            m["beta"] = 5
            m["gamma"] = m["gamma"] + 1
            let yes = m.has("alpha")
            let ks = m.keys()
            let vs = m.values()
            print(yes, ks, vs, lookup(m))
            for (x in m) {
                print(x)
            }
        }"#,
    );
    let syms = externs(&m);
    for need in [
        "pickle_map_new",
        "pickle_map_set",
        "pickle_map_get_boxed",
        "pickle_map_has",
        "pickle_map_len",
        "pickle_map_keys",
        "pickle_map_values",
    ] {
        assert!(syms.iter().any(|s| s == need), "missing {need}, externs: {syms:?}");
    }
    assert!(syms.iter().any(|s| s == "pickle_box_i64"), "externs: {syms:?}");
    assert!(syms.iter().any(|s| s == "pickle_unbox_i64"), "externs: {syms:?}");
    let lookup = m.funcs.iter().find(|f| f.name == "lookup").expect("lookup");
    assert!(
        lookup
            .blocks
            .iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| matches!(i, IrInstr::Call { callee: Callee::Extern(ex), .. }
                if m.externs.get(ex.0).map(|e| e.symbol.as_str()) == Some("pickle_map_get_boxed")))
            .count()
            >= 1,
        "a map read needs `pickle_map_get_boxed`, dump:\n{lookup}"
    );
}

#[test]
fn map_read_defaults_to_zero() {
    // An absent key must not unbox a null pointer: the emitter boxes the
    // value type's zero (`0`) and passes it to `pickle_map_get_boxed`.
    let m = emit_str(
        r#"fn peek(m: Map<string, int>) -> int {
            m["missing"]
        }"#,
    );
    let get_boxed = m
        .externs
        .iter()
        .position(|e| e.symbol == "pickle_map_get_boxed")
        .expect("pickle_map_get_boxed extern");
    let box_i64 = m
        .externs
        .iter()
        .position(|e| e.symbol == "pickle_box_i64")
        .expect("pickle_box_i64 extern");
    let peek = m.funcs.iter().find(|f| f.name == "peek").expect("peek");
    let calls: Vec<(usize, bool)> = peek
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter_map(|i| {
            if let IrInstr::Call { callee: Callee::Extern(id), .. } = i {
                Some((id.0, false))
            } else if let IrInstr::Const { c: IrConst::Int(0), .. } = i {
                Some((usize::MAX, true))
            } else {
                None
            }
        })
        .collect();
    let box_pos = calls
        .iter()
        .position(|(id, _)| *id == box_i64)
        .expect("boxing the zero before the read");
    let get_pos = calls
        .iter()
        .position(|(id, _)| *id == get_boxed)
        .expect("the boxed get");
    assert!(
        box_pos < get_pos,
        "the zero default must be boxed before `pickle_map_get_boxed`"
    );
    assert!(calls.iter().any(|(id, is_zero)| *is_zero || *id == box_i64));
}

#[test]
fn emits_class_ctor_registration_and_fields() {
    let m = emit_str(
        r#"class Point {
            x: int
            y: int
        }

        fn main() {
            let p = Point(1, 2)
            println(p.y)
            p.x = 7
        }"#,
    );
    let syms = externs(&m);
    for s in ["pickle_class_register", "pickle_class_new", "pickle_obj_slot_get", "pickle_obj_slot_set"] {
        assert!(syms.iter().any(|x| x == s), "externs: {syms:?}");
    }
    let main = m.funcs.iter().find(|f| f.name == "main").expect("main");
    let reg_calls = main
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter(|i| {
            if let IrInstr::Call { callee: Callee::Extern(id), .. } = i {
                m.externs.get(id.0).map(|e| e.symbol.as_str()) == Some("pickle_class_register")
            } else {
                false
            }
        })
        .count();
    assert!(reg_calls >= 1, "main must register every class up front");
}

#[test]
fn emits_class_methods_statics_and_this() {
    let m = emit_str(
        r#"class Counter {
            value: int

            fn add(by: int) {
                this.value = this.value + by
            }

            fn total() -> int {
                return this.value
            }

            static fn zero() -> string {
                return "z"
            }
        }

        fn main() {
            let c = Counter(1)
            c.add(2)
            println(c.total())
            println(Counter.zero())
        }"#,
    );
    let syms: Vec<String> = m.funcs.iter().map(|f| f.symbol.clone()).collect();
    for sym in ["pkl_Counter_new", "pkl_Counter_add", "pkl_Counter_total", "pkl_Counter_sm_zero"] {
        assert!(syms.iter().any(|s| s == sym), "symbols: {syms:?}");
    }
    let add = m.funcs.iter().find(|f| f.symbol == "pkl_Counter_add").expect("add");
    assert_eq!(add.params.first().map(|p| p.ty), Some(IrTy::Ptr), "receiver first");
    let zero = m.funcs.iter().find(|f| f.symbol == "pkl_Counter_sm_zero").expect("zero");
    assert!(zero.params.is_empty(), "static method has no receiver");
}

#[test]
fn emits_explicit_ctor_field_inits_and_init_block() {
    let m = emit_str(
        r#"class Counter {
            var count: int = 0

            constructor(start: int) {
                this.count = this.count + start
            }

            init {
                this.count = this.count + 1
            }

            fn total() -> int {
                return this.count
            }
        }

        fn main() {
            let c = Counter(1)
            println(c.total())
        }"#,
    );
    let syms: Vec<String> = m.funcs.iter().map(|f| f.symbol.clone()).collect();
    assert!(
        syms.iter().any(|s| s == "pkl_Counter_new"),
        "symbols: {syms:?}"
    );
    let new = m.funcs.iter().find(|f| f.symbol == "pkl_Counter_new").expect("new");
    assert_eq!(new.params.len(), 1, "explicit ctor params win over fields");
    assert_eq!(new.params[0].name, "start");
    let allocs = new
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter(|i| {
            if let IrInstr::Call { callee: Callee::Extern(id), .. } = i {
                m.externs.get(id.0).map(|e| e.symbol.as_str()) == Some("pickle_class_new")
            } else {
                false
            }
        })
        .count();
    assert!(allocs >= 1, "ctor allocates the object");
}

#[test]
fn synthesized_ctor_skips_initialized_fields() {
    let m = emit_str(
        r#"class Point {
            var x: int = 0
            y: int
        }

        class Label {
            text: string
            var times: int = 3
        }

        fn main() {
            let p = Point(5)
            let l = Label("hi")
            println(p.y)
            println(l.text)
        }"#,
    );
    let point = m.funcs.iter().find(|f| f.symbol == "pkl_Point_new").expect("Point.new");
    let names: Vec<&str> = point.params.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, vec!["y"], "initialized field `x` is not a parameter");
    let label = m.funcs.iter().find(|f| f.symbol == "pkl_Label_new").expect("Label.new");
    let names: Vec<&str> = label.params.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, vec!["text"], "`times` has an initializer so it is skipped");
}

#[test]
fn emits_property_accessors_and_dispatches() {
    let m = emit_str(
        r#"class Counter {
            var count: int

            property doubled: int {
                get => this.count * 2
            }

            property mirrored: int {
                get => this.count
                set { this.count = value }
            }

            property label: string {
                get { return "c" }
            }
        }

        fn main() {
            let c = Counter(1)
            println(c.doubled)
            c.mirrored = 9
            println(c.mirrored)
            println(c.label)
        }"#,
    );
    let syms: Vec<String> = m.funcs.iter().map(|f| f.symbol.clone()).collect();
    for sym in [
        "pkl_Counter_doubled_get",
        "pkl_Counter_mirrored_get",
        "pkl_Counter_mirrored_set",
        "pkl_Counter_label_get",
    ] {
        assert!(syms.iter().any(|s| s == sym), "symbols: {syms:?}");
    }
    let get = m.funcs.iter().find(|f| f.symbol == "pkl_Counter_doubled_get").expect("get");
    assert_eq!(get.params.first().map(|p| p.ty), Some(IrTy::Ptr), "getter receiver first");
    assert_eq!(get.ret, IrTy::Int, "getter returns the property type");
    let set = m.funcs.iter().find(|f| f.symbol == "pkl_Counter_mirrored_set").expect("set");
    assert_eq!(set.params.len(), 2, "setter takes `this` and `value`");
    assert_eq!(set.params[1].name, "value");
    assert_eq!(set.ret, IrTy::Unit, "setter returns unit");
    let main = m.funcs.iter().find(|f| f.name == "main").expect("main");
    let used: Vec<String> = main
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter_map(|i| {
            if let IrInstr::Call { callee: Callee::Func(fid), .. } = i {
                Some(m.funcs.get(fid.0).map(|f| f.symbol.clone()))
            } else {
                None
            }
        })
        .flatten()
        .collect();
    for want in ["pkl_Counter_doubled_get", "pkl_Counter_mirrored_get", "pkl_Counter_mirrored_set"] {
        assert!(used.iter().any(|s| s == want), "main calls: {used:?}");
    }
}

#[test]
fn property_getter_reads_bare_field_and_this() {
    // Bare field names and bare property names inside accessors resolve
    // through `this`; the getter/setter bodies must lower without unset `this`.
    let m = emit_str(
        r#"class Meter {
            var value: int

            property squared: int {
                get => value * value
            }

            property plus: int {
                get => this.value + this.squared
            }

            init {
                this.value = this.value + 1
            }
        }

        fn main() {
            let x = Meter(3)
            println(x.squared)
            println(x.plus)
        }"#,
    );
    let get = m.funcs.iter().find(|f| f.symbol == "pkl_Meter_squared_get").expect("get");
    let reads: Vec<&str> = get
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter_map(|i| {
            if let IrInstr::Call { callee: Callee::Extern(id), .. } = i {
                m.externs.get(id.0).map(|e| e.symbol.as_str())
            } else {
                None
            }
        })
        .filter(|s| *s == "pickle_obj_slot_get")
        .collect();
    assert!(reads.len() >= 2, "bare `value` reads dispatch through `this`, dump:\n{get}");
}

#[test]
fn emits_char_fields_and_collections_boxed() {
    // `char` fields, `List<char>`, and `Map<string, char>` values lower
    // through `pickle_box_char`/`pickle_unbox_char`; the class no longer bails.
    let m = emit_str(
        r#"class Tile {
            var glyph: char = 'x'
            var tag: char
        }

        fn main() {
            let t = Tile('?')
            t.tag = '!'
            println(t.glyph)
            let cs: List<char> = ['a', t.tag]
            println(cs[1])
            let by: Map<string, char> = { "k": 'z' }
            println(by["k"])
        }"#,
    );
    let externs: Vec<&str> = m.externs.iter().map(|e| e.symbol.as_str()).collect();
    for want in ["pickle_box_char", "pickle_unbox_char"] {
        assert!(externs.contains(&want), "externs: {externs:?}");
    }
    // RHS-first form: `Tile('?')` records `glyph = 'x'` via a field init, and
    // `tag` becomes the ctor parameter.
    let new = m.funcs.iter().find(|f| f.name == "Tile.new").expect("Tile.new");
    let names: Vec<&str> = new.params.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, vec!["tag"], "`glyph` carries an initializer so it is skipped");
    let tile = m.funcs.iter().find(|f| f.name == "Tile.new").expect("new");
    let stores: Vec<&str> = tile
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter_map(|i| {
            if let IrInstr::Call { callee: Callee::Extern(id), .. } = i {
                m.externs.get(id.0).map(|e| e.symbol.as_str())
            } else {
                None
            }
        })
        .collect();
    assert!(stores.contains(&"pickle_box_char"), "new must box the char field init, stores: {stores:?}");
    // The getter path unboxes: `t.glyph` + `cs[1]` + `by["k"]` all want the
    // char scalar back.
    let main = m.funcs.iter().find(|f| f.name == "main").expect("main");
    let unboxes = main
        .blocks
        .iter()
        .flat_map(|b| b.instrs.iter())
        .filter_map(|i| {
            if let IrInstr::Call { callee: Callee::Extern(id), .. } = i {
                m.externs.get(id.0).map(|e| e.symbol.as_str())
            } else {
                None
            }
        })
        .filter(|s| *s == "pickle_unbox_char")
        .count();
    assert!(unboxes >= 3, "field/get/list/map reads unbox chars, dump:\n{main}");
}