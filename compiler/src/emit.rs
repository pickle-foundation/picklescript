//! Lower the checked AST to PickleIR.
//!
//! The checker runs first (collecting `expr -> Ty` into a side table); this
//! module walks the typed AST and emits the flat block/slot/temp IR from
//! `ir.rs`. Constructs outside the slice-1 subset are rejected with a
//! "not lowered yet" diagnostic rather than miscompiled.

use std::collections::HashMap;

use crate::ast::*;
use crate::ast::BinOp as AstBinOp;
use crate::ast::UnOp as AstUnOp;
use crate::diag::{Diagnostic, DiagnosticSink, Span};
use crate::ir::*;
use crate::ir::BinOp as IrBinOp;
use crate::ir::UnOp as IrUnOp;
use crate::resolve::{CallableInfo, ResolvedProgram};
use crate::ty::Ty;

/// Front-end subset that emits IR. The module must already pass the checker.
///
/// Returns `None` when type-checking failed or a construct in an emitted body
/// is outside the current lowering subset (an error diagnostic is also
/// emitted).
pub fn emit_ir(
    prog: &Program,
    resolved: &ResolvedProgram,
    diags: &DiagnosticSink,
) -> Option<IrModule> {
    let types = crate::check::collect_expr_types(prog, resolved, diags);
    if diags.any_error() {
        return None;
    }
    let mut em = Emitter {
        prog,
        resolved,
        diags,
        types,
        module: IrModule::default(),
        consts_inits: HashMap::new(),
        const_inlining: Vec::new(),
        fid_list: Vec::new(),
        fdecl: HashMap::new(),
        finfo: HashMap::new(),
        fname: String::new(),
        symbol: String::new(),
        fparams: Vec::new(),
        fret: IrTy::Unit,
        fslots: Vec::new(),
        env: Vec::new(),
        blocks: Vec::new(),
        cur: BlockId(0),
        next_block: 0,
        next_temp: 0,
        loops: Vec::new(),
        failed: false,
    };
    em.emit();
    if em.failed {
        None
    } else {
        Some(em.module)
    }
}

/// A `break`/`continue` target pair.
struct LoopCtx {
    continue_target: BlockId,
    break_target: BlockId,
}

struct Emitter<'a> {
    prog: &'a Program,
    resolved: &'a ResolvedProgram,
    diags: &'a DiagnosticSink,
    types: HashMap<Span, Ty>,
    module: IrModule,
    /// Top-level `const` name -> its initializer (inlined at use sites).
    consts_inits: HashMap<String, &'a Expr>,
    const_inlining: Vec<String>,
    /// Registered functions in emission order (mirrors `module.funcs`).
    fid_list: Vec<FuncId>,
    fdecl: HashMap<FuncId, &'a FnDecl>,
    finfo: HashMap<FuncId, &'a CallableInfo>,

    // ---- per-function state ----
    fname: String,
    symbol: String,
    fparams: Vec<IrParam>,
    fret: IrTy,
    fslots: Vec<IrTy>,
    /// Lexical scopes; bottom is outermost (params live there).
    env: Vec<HashMap<String, Slot>>,
    blocks: Vec<IrBlock>,
    cur: BlockId,
    next_block: u32,
    next_temp: u32,
    loops: Vec<LoopCtx>,
    failed: bool,
}

impl<'a> Emitter<'a> {
    fn emit(&mut self) {
        for item in &self.prog.items {
            if let ItemKind::Const(c) = &item.kind {
                self.consts_inits.insert(c.name.clone(), &c.value);
            }
        }
        let prog = self.prog;
        for item in &prog.items {
            match &item.kind {
                ItemKind::Fn(f) => self.register_fn(f, false),
                ItemKind::Test(f) => self.register_fn(f, true),
                _ => {}
            }
        }
        let fids = self.fid_list.clone();
        for fid in fids {
            self.build_func(fid);
        }
    }

    // ---- registration -----------------------------------------------------

    fn register_fn(&mut self, f: &'a FnDecl, is_test: bool) {
        if f.is_async || !f.generics.is_empty() {
            return;
        }
        if f.params.iter().any(|p| p.default.is_some() || p.rest) {
            return;
        }
        let Some(info) = self.user_callable(&f.name) else {
            return;
        };
        let ok_params = info
            .params
            .iter()
            .map(|p| self.map_ty(&p.ty, f.span))
            .collect::<Result<Vec<_>, _>>()
            .is_ok();
        let ok_ret = self.map_ty(&info.ret, f.span).is_ok();
        if !ok_params || !ok_ret {
            return;
        }
        let fid = FuncId(self.module.funcs.len());
        let symbol = self.symbol_for(&f.name, is_test);
        self.module.funcs_by_name.insert(f.name.clone(), fid);
        self.module.funcs.push(IrFunc {
            name: f.name.clone(),
            symbol,
            params: Vec::new(),
            ret: IrTy::Unit,
            slots: Vec::new(),
            entry: BlockId(0),
            blocks: Vec::new(),
            is_main: !is_test && f.name == "main",
            is_test,
        });
        self.fid_list.push(fid);
        self.fdecl.insert(fid, f);
        self.finfo.insert(fid, info);
    }

    fn symbol_for(&self, name: &str, is_test: bool) -> String {
        if is_test {
            format!("pickle_test_{name}")
        } else if name == "main" {
            "pickle_main".to_string()
        } else {
            format!("pkl_{name}")
        }
    }

    /// The single user (non-builtin, non-overloaded) callable for `name`.
    fn user_callable(&self, name: &str) -> Option<&'a CallableInfo> {
        let infos = self.resolved.fns.get(name)?;
        let mut users = infos
            .iter()
            .filter(|c| !(c.span.file.0 == 0 && c.span.end == 0));
        let first = users.next()?;
        if users.next().is_some() {
            return None;
        }
        Some(first)
    }

    // ---- per-function build ----------------------------------------------

    fn build_func(&mut self, fid: FuncId) {
        self.fname = self.module.funcs[fid.0].name.clone();
        self.symbol = self.module.funcs[fid.0].symbol.clone();
        self.fparams = Vec::new();
        self.fslots = Vec::new();
        self.env = vec![HashMap::new()];
        self.blocks = vec![IrBlock {
            id: BlockId(0),
            instrs: Vec::new(),
            term: IrTerm::Unreachable,
        }];
        self.next_block = 1;
        self.next_temp = 0;
        self.loops = Vec::new();
        self.cur = BlockId(0);

        let Some(info) = self.finfo.get(&fid).copied() else {
            return;
        };
        let f = self.fdecl[&fid];

        // Params occupy slots 0..n.
        for (i, p) in info.params.iter().enumerate() {
            let ir = self.map_ty(&p.ty, f.span).unwrap_or(IrTy::Ptr);
            let slot = Slot(i as u32);
            self.fslots.push(ir);
            self.fparams.push(IrParam {
                name: p.name.clone(),
                ty: ir,
            });
            self.declare(&p.name, slot);
        }
        self.fret = self.map_ty(&info.ret, f.span).unwrap_or(IrTy::Unit);

        self.emit_body(f);
        if self.failed {
            return;
        }

        self.module.funcs[fid.0] = IrFunc {
            name: self.fname.clone(),
            symbol: self.symbol.clone(),
            params: std::mem::take(&mut self.fparams),
            ret: self.fret,
            slots: std::mem::take(&mut self.fslots),
            entry: BlockId(0),
            blocks: std::mem::take(&mut self.blocks),
            is_main: self.module.funcs[fid.0].is_main,
            is_test: self.module.funcs[fid.0].is_test,
        };
    }

    /// Emit the function body; the current block is left terminator-clean.
    fn emit_body(&mut self, f: &'a FnDecl) -> bool {
        let body = match &f.body {
            Some(b) => b,
            None => return true,
        };
        match body {
            FnBody::Block(b) => {
                self.push_scope();
                self.block_tail_as_return(b);
                self.pop_scope();
            }
            FnBody::Expr(e) => {
                if self.fret.is_unit() {
                    let _ = self.expr(e);
                } else if let Ok(t) = self.expr(e) {
                    self.term(IrTerm::Return { v: Some(t) });
                }
            }
        }
        let _ = self.tail_cleanup();
        true
    }

    /// After a block body, close the current block: emit the body's statements,
    /// then if the function returns a value the tail expression *is* the return
    /// value; otherwise it is discarded.
    fn block_tail_as_return(&mut self, b: &Block) {
        for s in &b.stmts {
            if self.stmt(s).is_err() {
                return;
            }
        }
        match &b.expr {
            Some(e) if !self.fret.is_unit() => {
                if let Ok(t) = self.expr(e) {
                    self.term(IrTerm::Return { v: Some(t) });
                }
            }
            Some(e) => {
                let _ = self.expr(e);
                self.term(IrTerm::Return { v: None });
            }
            None if self.fret.is_unit() => {
                self.term(IrTerm::Return { v: None });
            }
            None => {
                // A value-returning function reaches here only when the last
                // statement already terminated this block (`return`), so the
                // current block is dead. Closing it with an empty `return`
                // would not type-check against the value signature, so leave
                // it `Unreachable`; a live block with no tail value is a
                // missing-return program and is rejected upstream by the
                // checker.
            }
        }
    }

    /// Ensure the final block is closed. Returns false iff the function's last
    /// block is dead (shorter than a missing return). Only unit-returning
    /// functions get the trailing empty `return`; a value-returning function
    /// whose last statement already returned leaves its dead block
    /// `Unreachable` (an empty `return` would fail Cranelift's verifier).
    fn tail_cleanup(&mut self) -> bool {
        let block = &mut self.blocks[self.cur.0 as usize];
        if matches!(block.term, IrTerm::Unreachable) && self.fret.is_unit() {
            block.term = IrTerm::Return { v: None };
        }
        true
    }

    // ---- statements ----

    fn stmt(&mut self, s: &Stmt) -> Result<(), ()> {
        match s {
            Stmt::Let {
                pattern,
                ty,
                init,
                mutable: _,
                span,
            } => {
                match pattern {
                    Pattern::Binding { .. } => {
                        let slot_ty = match init {
                            Some(e) => self.irty(e.span)?,
                            None => self.annot_ty(ty, *span)?,
                        };
                        let slot = self.new_slot(slot_ty);
                        if let Some(e) = init {
                            let t = self.expr(e)?;
                            self.instr(IrInstr::StoreSlot { slot, v: t });
                        }
                        if let Pattern::Binding { name, .. } = pattern {
                            self.declare(name, slot);
                        }
                        Ok(())
                    }
                    Pattern::Wildcard => {
                        if let Some(e) = init {
                            let _ = self.expr(e)?;
                        }
                        Ok(())
                    }
                    _ => self.bad(*span, "destructuring patterns are not lowered yet"),
                }
            }
            Stmt::Const { name, value, .. } => {
                let t = self.expr(value)?;
                let ty = self.irty(value.span)?;
                let slot = self.new_slot(ty);
                self.instr(IrInstr::StoreSlot { slot, v: t });
                self.declare(name, slot);
                Ok(())
            }
            Stmt::Return { value, .. } => {
                let v = match value {
                    Some(e) => Some(self.expr(e)?),
                    None => None,
                };
                self.term(IrTerm::Return { v });
                self.cur = self.new_block();
                Ok(())
            }
            Stmt::Break { span: _ } => {
                let Some(target) = self.loops.last().map(|l| l.break_target) else {
                    return self.bad_span_note("unbalanced `break`");
                };
                self.term(IrTerm::Branch { target });
                self.cur = self.new_block();
                Ok(())
            }
            Stmt::Continue { span: _ } => {
                let Some(target) = self.loops.last().map(|l| l.continue_target) else {
                    return self.bad_span_note("unbalanced `continue`");
                };
                self.term(IrTerm::Branch { target });
                self.cur = self.new_block();
                Ok(())
            }
            Stmt::While { cond, body, span } => self.while_stmt(cond, body, *span),
            Stmt::For { header, body, span } => self.for_stmt(header, body, *span),
            Stmt::Expr(e) => {
                let _ = self.expr(e)?;
                Ok(())
            }
            Stmt::Empty(_) => Ok(()),
        }
    }

    fn while_stmt(&mut self, cond: &Expr, body: &Block, span: Span) -> Result<(), ()> {
        let _ = span;
        let cond_id = self.new_block();
        let body_id = self.new_block();
        let end_id = self.new_block();
        self.term(IrTerm::Branch { target: cond_id });
        self.cur = cond_id;
        let c = self.expr(cond)?;
        self.term(IrTerm::BranchIf {
            cond: c,
            then: body_id,
            else_: end_id,
        });
        self.cur = body_id;
        self.loops.push(LoopCtx {
            continue_target: cond_id,
            break_target: end_id,
        });
        self.push_scope();
        self.block_body_only(body)?;
        self.pop_scope();
        self.loops.pop();
        self.term(IrTerm::Branch { target: cond_id });
        self.cur = end_id;
        Ok(())
    }

    fn for_stmt(&mut self, header: &ForHeader, body: &Block, span: Span) -> Result<(), ()> {
        match header {
            ForHeader::Range { init, cond, step } => {
                self.stmt(init)?;
                let cond_id = self.new_block();
                let body_id = self.new_block();
                let end_id = self.new_block();
                self.term(IrTerm::Branch { target: cond_id });
                self.cur = cond_id;
                let c = self.expr(cond)?;
                self.term(IrTerm::BranchIf {
                    cond: c,
                    then: body_id,
                    else_: end_id,
                });
                self.cur = body_id;
                self.loops.push(LoopCtx {
                    continue_target: cond_id,
                    break_target: end_id,
                });
                self.push_scope();
                self.block_body_only(body)?;
                let _ = self.expr(step);
                self.pop_scope();
                self.loops.pop();
                self.term(IrTerm::Branch { target: cond_id });
                self.cur = end_id;
                Ok(())
            }
            ForHeader::In { pattern, sequence } => self.for_in(pattern, sequence, body, span),
        }
    }

    /// `for (x in a..b)`: slice-1 supports integer range sequences only.
    fn for_in(
        &mut self,
        pattern: &Pattern,
        sequence: &Expr,
        body: &Block,
        span: Span,
    ) -> Result<(), ()> {
        let Pattern::Binding { name, .. } = pattern else {
            return self.bad(span, "iteration patterns other than a binding are not lowered yet");
        };
        let (start, end, incl) = match &sequence.kind {
            ExprKind::Binary {
                op: AstBinOp::Range,
                lhs,
                rhs,
            } => (lhs, rhs, false),
            ExprKind::Binary {
                op: AstBinOp::RangeIncl,
                lhs,
                rhs,
            } => (lhs, rhs, true),
            _ => return self.bad(
                sequence.span,
                "`for (x in ...)` over non-range sequences is not lowered yet",
            ),
        };
        self.ensure_int(start)?;
        self.ensure_int(end)?;
        let start_t = self.expr(start)?;
        let end_t = self.expr(end)?;
        let idx = self.new_slot(IrTy::Int);
        let end_slot = self.new_slot(IrTy::Int);
        self.instr(IrInstr::StoreSlot { slot: idx, v: start_t });
        self.instr(IrInstr::StoreSlot { slot: end_slot, v: end_t });

        let cond_id = self.new_block();
        let body_id = self.new_block();
        let end_id = self.new_block();
        self.term(IrTerm::Branch { target: cond_id });
        self.cur = cond_id;
        let cur_t = self.load(idx);
        let end_l = self.load(end_slot);
        let cmp = self.temp();
        self.instr(IrInstr::BinOp {
            dst: cmp,
            op: if incl { IrBinOp::Le } else { IrBinOp::Lt },
            a: cur_t,
            b: end_l,
        });
        self.term(IrTerm::BranchIf {
            cond: cmp,
            then: body_id,
            else_: end_id,
        });
        self.cur = body_id;
        self.loops.push(LoopCtx {
            continue_target: cond_id,
            break_target: end_id,
        });
        self.push_scope();
        self.declare(name, idx);
        self.block_body_only(body)?;
        self.pop_scope();
        self.loops.pop();
        let c = self.load(idx);
        let one = self.temp();
        self.instr(IrInstr::Const {
            dst: one,
            c: IrConst::Int(1),
        });
        let nxt = self.temp();
        self.instr(IrInstr::BinOp {
            dst: nxt,
            op: IrBinOp::Add,
            a: c,
            b: one,
        });
        self.instr(IrInstr::StoreSlot { slot: idx, v: nxt });
        self.term(IrTerm::Branch { target: cond_id });
        self.cur = end_id;
        Ok(())
    }

    fn block_body_only(&mut self, b: &Block) -> Result<(), ()> {
        for s in &b.stmts {
            self.stmt(s)?;
        }
        if let Some(e) = &b.expr {
            let _ = self.expr(e)?;
        }
        Ok(())
    }

    // ---- expressions ----

    fn expr(&mut self, e: &Expr) -> Result<Temp, ()> {
        let _ = self.irty(e.span)?;
        match &e.kind {
            ExprKind::Lit(l) => self.lit(l),
            ExprKind::Ident(name) => self.ident_expr(e, name),
            ExprKind::Call { callee, args } => self.call(e, callee, args),
            ExprKind::Binary { op, lhs, rhs } => self.binary(e, *op, lhs, rhs),
            ExprKind::Unary { op, operand } => self.unary(e, *op, operand),
            ExprKind::Assign { target, op, value } => self.assign(e, target, *op, value),
            ExprKind::If {
                cond,
                then,
                else_else,
            } => self.if_expr(e, cond, then, else_else.as_deref()),
            ExprKind::Block(b) => {
                self.push_scope();
                let r = self.block_value(b);
                self.pop_scope();
                r
            }
            ExprKind::This => self.bad(e.span, "`this` is not lowered yet"),
            ExprKind::Super => self.bad(e.span, "`super` is not lowered yet"),
            ExprKind::Member { .. } => self.bad(e.span, "member access is not lowered yet"),
            ExprKind::Index { .. } => self.bad(e.span, "indexing is not lowered yet"),
            ExprKind::OptAccess { .. } => self.bad(e.span, "optional access is not lowered yet"),
            ExprKind::OptUnwrap(_) => self.bad(e.span, "`!` unwrap is not lowered yet"),
            ExprKind::Lambda { .. } => self.bad(e.span, "lambda values are not lowered yet"),
            ExprKind::Match { .. } => self.bad(e.span, "`match` is not lowered yet"),
            ExprKind::Await(_) => self.bad(e.span, "`await` is not lowered yet"),
            ExprKind::GenericCall { .. } => self.bad(e.span, "generic calls are not lowered yet"),
            ExprKind::Cast { .. } => self.bad(e.span, "casts are not lowered yet"),
            ExprKind::Unsafe(_) => self.bad(e.span, "`unsafe` blocks are not lowered yet"),
            ExprKind::Tuple(_) => self.bad(e.span, "tuple values are not lowered yet"),
            ExprKind::Array(_) => self.bad(e.span, "array values are not lowered yet"),
            ExprKind::Map(_) => self.bad(e.span, "map values are not lowered yet"),
            ExprKind::Range { .. } => self.bad(e.span, "range values are not lowered yet"),
        }
    }

    fn block_value(&mut self, b: &Block) -> Result<Temp, ()> {
        for s in &b.stmts {
            self.stmt(s)?;
        }
        match &b.expr {
            Some(e) => self.expr(e),
            None => Ok(self.unit_temp()),
        }
    }

    fn lit(&mut self, l: &Lit) -> Result<Temp, ()> {
        let dst = self.temp();
        let c = match l {
            Lit::Int { value } => match i64::try_from(*value) {
                Ok(v) => IrConst::Int(v),
                Err(_) => {
                    return self.bad(
                        self.fname_span_fallback(),
                        "integer literal out of range for i64",
                    )
                }
            },
            Lit::Float { value } => IrConst::Float(value.to_bits()),
            Lit::Bool(v) => IrConst::Bool(*v),
            Lit::Char(c) => IrConst::Char(*c as u32),
            Lit::String(parts) => {
                return self.string_literal(parts);
            }
            Lit::None => {
                return self.bad(
                    self.fname_span_fallback(),
                    "`none` literals are not lowered yet",
                )
            }
        };
        self.instr(IrInstr::Const { dst, c });
        Ok(dst)
    }

    fn fname_span_fallback(&self) -> Span {
        // Literal range errors shouldn't happen for checked i128 ints that
        // overflow i64; give a zero span if the expr span is unavailable.
        Span::new(crate::diag::FileId(0), 0, 0)
    }

    /// Lower a string literal. Plain text becomes a `Const` string temp; an
    /// interpolated expression is emitted, stringified via a `pickle_str_from_*`
    /// helper when needed, and the parts are concatenated with
    /// `pickle_str_concat`.
    fn string_literal(&mut self, parts: &[StrPart]) -> Result<Temp, ()> {
        let mut acc: Option<Temp> = None;
        for part in parts {
            let v = match part {
                StrPart::Text(text) => {
                    let sid = StrId(self.intern_string(text));
                    let t = self.temp();
                    self.instr(IrInstr::Const {
                        dst: t,
                        c: IrConst::Str(sid),
                    });
                    t
                }
                StrPart::Expr(e) => {
                    let t = self.expr(e)?;
                    match self.irty(e.span)? {
                        IrTy::Str => t,
                        IrTy::Int => self.extern_call_t1(
                            "pickle_str_from_i64",
                            vec![IrTy::Int],
                            IrTy::Str,
                            vec![t],
                        )?,
                        IrTy::Float => self.extern_call_t1(
                            "pickle_str_from_f64",
                            vec![IrTy::Float],
                            IrTy::Str,
                            vec![t],
                        )?,
                        IrTy::Bool => self.extern_call_t1(
                            "pickle_str_from_bool",
                            vec![IrTy::Bool],
                            IrTy::Str,
                            vec![t],
                        )?,
                        IrTy::Char => self.extern_call_t1(
                            "pickle_str_from_char",
                            vec![IrTy::Char],
                            IrTy::Str,
                            vec![t],
                        )?,
                        other => {
                            return self.bad(
                                e.span,
                                format!("interpolation of `{other:?}` is not lowered yet"),
                            )
                        }
                    }
                }
            };
            acc = Some(match acc {
                None => v,
                Some(a) => self.extern_call_t1(
                    "pickle_str_concat",
                    vec![IrTy::Str, IrTy::Str],
                    IrTy::Str,
                    vec![a, v],
                )?,
            });
        }
        match acc {
            Some(t) => Ok(t),
            None => self.bad(Span::new(crate::diag::FileId(0), 0, 0), "empty string literal"),
        }
    }

    fn ident_expr(&mut self, e: &Expr, name: &str) -> Result<Temp, ()> {
        if let Some(slot) = self.lookup(name) {
            return Ok(self.load(slot));
        }
        // Inline a top-level const initializer.
        if let Some(init) = self.consts_inits.get(name).copied() {
            if self.const_inlining.iter().any(|n| n == name) {
                return self.bad(e.span, format!("cyclic `const` initialization of `{name}`"));
            }
            self.const_inlining.push(name.to_string());
            let t = self.expr(init);
            self.const_inlining.pop();
            return t;
        }
        self.bad(e.span, format!("using `{name}` as a value is not lowered yet"))
    }

    fn unary(&mut self, e: &Expr, op: AstUnOp, operand: &Expr) -> Result<Temp, ()> {
        match op {
            AstUnOp::Neg | AstUnOp::BitNot => {
                let v = self.expr(operand)?;
                let dst = self.temp();
                self.instr(IrInstr::UnOp {
                    dst,
                    op: if matches!(op, AstUnOp::Neg) {
                        IrUnOp::Neg
                    } else {
                        IrUnOp::BitNot
                    },
                    v,
                });
                Ok(dst)
            }
            AstUnOp::Not => {
                let v = self.expr(operand)?;
                let dst = self.temp();
                self.instr(IrInstr::UnOp {
                    dst,
                    op: IrUnOp::Not,
                    v,
                });
                Ok(dst)
            }
            AstUnOp::Deref | AstUnOp::AddrOf => {
                self.bad(e.span, "pointer operations are not lowered yet")
            }
        }
    }

    fn binary(&mut self, e: &Expr, op: AstBinOp, lhs: &Expr, rhs: &Expr) -> Result<Temp, ()> {
        let lt = self.ty_of(&lhs.span);
        let rt = self.ty_of(&rhs.span);
        let both_str = matches!(lt, Some(Ty::String)) && matches!(rt, Some(Ty::String));
        match op {
            AstBinOp::And | AstBinOp::Or => return self.logic(e, op, lhs, rhs),
            AstBinOp::Range | AstBinOp::RangeIncl | AstBinOp::NullCoalesce | AstBinOp::Send
            | AstBinOp::Is | AstBinOp::In | AstBinOp::Pow => {
                return self.bad(e.span, "this operator is not lowered yet")
            }
            AstBinOp::Add if both_str => {
                let a = self.expr(lhs)?;
                let b = self.expr(rhs)?;
                return self.extern_call_t1(
                    "pickle_str_concat",
                    vec![IrTy::Str, IrTy::Str],
                    IrTy::Str,
                    vec![a, b],
                );
            }
            AstBinOp::Eq | AstBinOp::Ne if both_str => {
                let a = self.expr(lhs)?;
                let b = self.expr(rhs)?;
                let cmp = self.extern_call_t1(
                    "pickle_str_cmp",
                    vec![IrTy::Str, IrTy::Str],
                    IrTy::Int,
                    vec![a, b],
                )?;
                let zero = self.temp();
                self.instr(IrInstr::Const {
                    dst: zero,
                    c: IrConst::Int(0),
                });
                let dst = self.temp();
                self.instr(IrInstr::BinOp {
                    dst,
                    op: if op == AstBinOp::Eq { IrBinOp::Eq } else { IrBinOp::Ne },
                    a: cmp,
                    b: zero,
                });
                return Ok(dst);
            }
            _ => {}
        }
        let a = self.expr(lhs)?;
        let b = self.expr(rhs)?;
        let dst = self.temp();
        self.instr(IrInstr::BinOp {
            dst,
            op: binary_opcode(op),
            a,
            b,
        });
        Ok(dst)
    }

    /// Short-circuit `&&` / `||` via branches and a result slot.
    fn logic(&mut self, _e: &Expr, op: AstBinOp, lhs: &Expr, rhs: &Expr) -> Result<Temp, ()> {
        let res_slot = self.new_slot(IrTy::Bool);
        let is_and = op == AstBinOp::And;
        let a = self.expr(lhs)?;
        let rhs_id = self.new_block();
        let short_id = self.new_block();
        let join = self.new_block();
        self.term(IrTerm::BranchIf {
            cond: a,
            then: if is_and { rhs_id } else { short_id },
            else_: if is_and { short_id } else { rhs_id },
        });
        self.cur = rhs_id;
        let b = self.expr(rhs)?;
        self.instr(IrInstr::StoreSlot { slot: res_slot, v: b });
        self.term(IrTerm::Branch { target: join });
        self.cur = short_id;
        let sv = self.temp();
        self.instr(IrInstr::Const {
            dst: sv,
            c: IrConst::Bool(!is_and),
        });
        self.instr(IrInstr::StoreSlot { slot: res_slot, v: sv });
        self.term(IrTerm::Branch { target: join });
        self.cur = join;
        Ok(self.load(res_slot))
    }

    fn assign(&mut self, e: &Expr, target: &Expr, op: AssignOp, value: &Expr) -> Result<Temp, ()> {
        let span = target.span;
        let ExprKind::Ident(name) = &target.kind else {
            return self.bad(span, "assignment targets other than names are not lowered yet");
        };
        let Some(slot) = self.lookup(name) else {
            return self.bad(span, format!("cannot assign to `{name}`"));
        };
        let v = self.expr(value)?;
        if op == AssignOp::Assign {
            self.instr(IrInstr::StoreSlot { slot, v });
            return Ok(v);
        }
        let cur = self.load(slot);
        let dst = self.temp();
        self.instr(IrInstr::BinOp {
            dst,
            op: assign_opcode(op),
            a: cur,
            b: v,
        });
        self.instr(IrInstr::StoreSlot { slot, v: dst });
        let _ = e;
        Ok(dst)
    }

    fn if_expr(
        &mut self,
        e: &Expr,
        cond: &IfCond,
        then: &Block,
        else_else: Option<&Expr>,
    ) -> Result<Temp, ()> {
        if matches!(cond, IfCond::Binding { .. }) {
            return self.bad(then.span, "`if (let ...)` bindings are not lowered yet");
        }
        let IfCond::Cond(c) = cond else { unreachable!() };
        let cond_t = self.expr(c)?;
        let res_ty = self.irty(e.span).ok();
        let need_slot = !matches!(res_ty, Some(IrTy::Unit) | None) && else_else.is_some();
        let res_slot = if need_slot {
            Some(self.new_slot(res_ty.unwrap()))
        } else {
            None
        };
        let then_id = self.new_block();
        let else_id = if else_else.is_some() {
            Some(self.new_block())
        } else {
            None
        };
        let join = self.new_block();
        self.term(IrTerm::BranchIf {
            cond: cond_t,
            then: then_id,
            else_: else_id.unwrap_or(join),
        });

        self.cur = then_id;
        self.push_scope();
        self.block_into_slot(then, res_slot)?;
        self.pop_scope();
        self.term(IrTerm::Branch { target: join });

        if let Some(e) = else_else {
            self.cur = else_id.unwrap();
            match res_slot {
                Some(slot) => {
                    let t = self.expr(e)?;
                    self.instr(IrInstr::StoreSlot { slot, v: t });
                }
                None => {
                    let _ = self.expr(e)?;
                }
            }
            self.term(IrTerm::Branch { target: join });
        }
        self.cur = join;
        match res_slot {
            Some(slot) => Ok(self.load(slot)),
            None => Ok(self.unit_temp()),
        }
    }

    /// Emit a block whose tail value (if any) is stored into `res_slot`.
    fn block_into_slot(&mut self, b: &Block, res_slot: Option<Slot>) -> Result<(), ()> {
        for s in &b.stmts {
            self.stmt(s)?;
        }
        if let Some(e) = &b.expr {
            let t = self.expr(e)?;
            if let Some(slot) = res_slot {
                self.instr(IrInstr::StoreSlot { slot, v: t });
            }
        }
        Ok(())
    }

    fn call(&mut self, e: &Expr, callee: &Expr, args: &[CallArg]) -> Result<Temp, ()> {
        let ExprKind::Ident(name) = &callee.kind else {
            return self.bad(e.span, "only plain function calls are lowered yet");
        };
        // User function?
        if let Some(&fid) = self.module.funcs_by_name.get(name) {
            let mut arg_temps = Vec::new();
            for a in args {
                if a.spread {
                    return self.bad(a.span, "spread arguments are not lowered yet");
                }
                arg_temps.push(self.expr(&a.value)?);
            }
            let dst = self.temp();
            self.instr(IrInstr::Call {
                dst: Some(dst),
                callee: Callee::Func(fid),
                args: arg_temps,
            });
            return Ok(dst);
        }
        // Constructor call?
        if self.resolved.types.contains_key(name) {
            return self.bad(e.span, format!("constructor calls to `{name}` are not lowered yet"));
        }
        // Builtins.
        match name.as_str() {
            "print" | "println" => {
                let newline = name == "println";
                for a in args {
                    let t = self.expr(&a.value)?;
                    let a_ty = self.ty_of(&a.value.span);
                    let sym: &str = match a_ty {
                        Some(Ty::Int) => "pickle_print_i64",
                        Some(Ty::Float) => "pickle_print_f64",
                        Some(Ty::Bool) => "pickle_print_bool",
                        Some(Ty::Char) => "pickle_print_byte",
                        Some(Ty::String) => "pickle_print_obj",
                        _ => {
                            return self.bad(
                                a.value.span,
                                "unsupported `print` argument type",
                            )
                        }
                    };
                    let pty = match a_ty {
                        Some(Ty::Int) => IrTy::Int,
                        Some(Ty::Float) => IrTy::Float,
                        Some(Ty::Bool) => IrTy::Bool,
                        Some(Ty::Char) => IrTy::Char,
                        Some(Ty::String) => IrTy::Ptr,
                        _ => IrTy::Ptr,
                    };
                    self.extern_call_void(sym, vec![pty], vec![t]);
                }
                if newline {
                    self.extern_call_void("pickle_print_newline", vec![], vec![]);
                }
                Ok(self.unit_temp())
            }
            "len" => {
                if args.len() != 1 {
                    return self.bad(e.span, "`len` takes one argument");
                }
                let t = self.expr(&args[0].value)?;
                let a_ty = self.ty_of(&args[0].value.span);
                match a_ty {
                    Some(Ty::String) => {
                        self.extern_call_t1("pickle_str_len", vec![IrTy::Str], IrTy::Int, vec![t])
                    }
                    _ => self.bad(e.span, "`len` over this type is not lowered yet"),
                }
            }
            _ => self.bad(e.span, format!("`{name}` is not lowered yet")),
        }
    }

    // ---- primitive ops ----

    fn extern_call_void(&mut self, symbol: &str, params: Vec<IrTy>, args: Vec<Temp>) {
        let ex = self.module.extern_id(IrExtern {
            symbol: symbol.to_string(),
            params,
            ret: IrTy::Unit,
        });
        self.instr(IrInstr::Call {
            dst: None,
            callee: Callee::Extern(ex),
            args,
        });
    }

    fn extern_call_t1(
        &mut self,
        symbol: &str,
        params: Vec<IrTy>,
        ret: IrTy,
        args: Vec<Temp>,
    ) -> Result<Temp, ()> {
        let ex = self.module.extern_id(IrExtern {
            symbol: symbol.to_string(),
            params,
            ret,
        });
        let dst = self.temp();
        self.instr(IrInstr::Call {
            dst: Some(dst),
            callee: Callee::Extern(ex),
            args,
        });
        Ok(dst)
    }

    // ---- types & helpers ----

    fn ty_of(&self, span: &Span) -> Option<Ty> {
        self.types.get(span).cloned()
    }

    fn irty(&mut self, span: Span) -> Result<IrTy, ()> {
        let Some(t) = self.ty_of(&span) else {
            return self.bad(span, "no inferred type available for this expression");
        };
        self.map_ty(&t, span)
    }

    fn map_ty(&mut self, t: &Ty, span: Span) -> Result<IrTy, ()> {
        match t {
            Ty::Bool => Ok(IrTy::Bool),
            Ty::Char => Ok(IrTy::Char),
            Ty::Int => Ok(IrTy::Int),
            Ty::Float => Ok(IrTy::Float),
            Ty::String => Ok(IrTy::Str),
            Ty::None | Ty::Empty => Ok(IrTy::Unit),
            Ty::Option(..)
            | Ty::Class(..)
            | Ty::Struct(..)
            | Ty::Enum(..)
            | Ty::Interface(..)
            | Ty::List(..)
            | Ty::Map(..)
            | Ty::Tuple(..)
            | Ty::Range(..) => {
                let _ = span;
                Ok(IrTy::Ptr)
            }
            Ty::Fn(..) => self.bad(span, "function values are not lowered yet"),
            Ty::Unknown => self.bad(span, "untyped expression cannot be lowered"),
            Ty::Var(_) => self.bad(span, "generic functions are not lowered yet"),
        }
    }

    fn annot_ty(&mut self, ty: &Option<TypeExpr>, span: Span) -> Result<IrTy, ()> {
        let Some(te) = ty else {
            return self.bad(span, "`let` requires an initializer or a type annotation");
        };
        match &te.kind {
            TypeExprKind::Path(parts) if parts.len() == 1 => match parts[0].as_str() {
                "Int" => Ok(IrTy::Int),
                "Float" => Ok(IrTy::Float),
                "Bool" => Ok(IrTy::Bool),
                "Char" => Ok(IrTy::Char),
                "String" => Ok(IrTy::Str),
                _ => Ok(IrTy::Ptr),
            },
            _ => self.bad(span, "this type annotation is not lowered yet"),
        }
    }

    fn ensure_int(&mut self, e: &Expr) -> Result<(), ()> {
        match self.ty_of(&e.span) {
            Some(Ty::Int) => Ok(()),
            _ => self.bad(e.span, "range bounds must be `int`"),
        }
    }

    fn bad<T>(&mut self, span: Span, msg: impl Into<String>) -> Result<T, ()> {
        self.failed = true;
        self.diags
            .emit(Diagnostic::error_at(span, format!("codegen: {}", msg.into())));
    Err(())
    }

    fn bad_span_note<T>(&mut self, msg: &str) -> Result<T, ()> {
        self.failed = true;
        self.diags
            .emit(Diagnostic::error_at(Span::new(crate::diag::FileId(0), 0, 0), format!("codegen: {msg}")));
    Err(())
    }

    // ---- allocation helpers ----

    fn temp(&mut self) -> Temp {
        let t = Temp(self.next_temp);
        self.next_temp += 1;
        t
    }

    fn unit_temp(&mut self) -> Temp {
        let t = self.temp();
        self.instr(IrInstr::Const {
            dst: t,
            c: IrConst::Int(0),
        });
        t
    }

    fn new_slot(&mut self, ty: IrTy) -> Slot {
        let s = Slot(self.fslots.len() as u32);
        self.fslots.push(ty);
        s
    }

    fn load(&mut self, slot: Slot) -> Temp {
        let d = self.temp();
        self.instr(IrInstr::LoadSlot { dst: d, slot });
        d
    }

    fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block);
        self.next_block += 1;
        self.blocks.push(IrBlock {
            id,
            instrs: Vec::new(),
            term: IrTerm::Unreachable,
        });
        id
    }

    fn instr(&mut self, i: IrInstr) {
        let block = &mut self.blocks[self.cur.0 as usize];
        block.instrs.push(i);
    }

    fn term(&mut self, t: IrTerm) {
        let block = &mut self.blocks[self.cur.0 as usize];
        block.term = t;
    }

    fn push_scope(&mut self) {
        self.env.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.env.pop();
    }

    fn declare(&mut self, name: &str, slot: Slot) {
        if let Some(scope) = self.env.last_mut() {
            scope.insert(name.to_string(), slot);
        }
    }

    fn lookup(&self, name: &str) -> Option<Slot> {
        for scope in self.env.iter().rev() {
            if let Some(slot) = scope.get(name) {
                return Some(*slot);
            }
        }
        None
    }

    fn intern_string(&mut self, s: &str) -> usize {
        for (i, existing) in self.module.strings.iter().enumerate() {
            if existing == s.as_bytes() {
                return i;
            }
        }
        self.module.strings.push(s.as_bytes().to_vec());
        self.module.strings.len() - 1
    }
}

fn binary_opcode(op: AstBinOp) -> IrBinOp {
    match op {
        AstBinOp::Add => IrBinOp::Add,
        AstBinOp::Sub => IrBinOp::Sub,
        AstBinOp::Mul => IrBinOp::Mul,
        AstBinOp::Div => IrBinOp::Div,
        AstBinOp::Mod => IrBinOp::Mod,
        AstBinOp::Shl => IrBinOp::Shl,
        AstBinOp::Shr => IrBinOp::Shr,
        AstBinOp::BitAnd => IrBinOp::BitAnd,
        AstBinOp::BitOr => IrBinOp::BitOr,
        AstBinOp::BitXor => IrBinOp::BitXor,
        AstBinOp::Lt => IrBinOp::Lt,
        AstBinOp::Le => IrBinOp::Le,
        AstBinOp::Gt => IrBinOp::Gt,
        AstBinOp::Ge => IrBinOp::Ge,
        AstBinOp::Eq => IrBinOp::Eq,
        AstBinOp::Ne => IrBinOp::Ne,
        _ => IrBinOp::Add,
    }
}

fn assign_opcode(op: AssignOp) -> IrBinOp {
    match op {
        AssignOp::Add => IrBinOp::Add,
        AssignOp::Sub => IrBinOp::Sub,
        AssignOp::Mul => IrBinOp::Mul,
        AssignOp::Div => IrBinOp::Div,
        AssignOp::Mod => IrBinOp::Mod,
        AssignOp::Shl => IrBinOp::Shl,
        AssignOp::Shr => IrBinOp::Shr,
        AssignOp::BitAnd => IrBinOp::BitAnd,
        AssignOp::BitOr => IrBinOp::BitOr,
        AssignOp::BitXor => IrBinOp::BitXor,
        AssignOp::Assign => IrBinOp::Add,
    }
}