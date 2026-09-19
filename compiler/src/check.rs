use std::collections::HashMap;

use crate::ast::*;
use crate::diag::{Diagnostic, DiagnosticSink, Span};
use crate::resolve::{
    CallableInfo, ClassTable, ResolvedProgram, TypeTableEntry,
};
use crate::ty::Ty;

/// A bound local variable inside a function body.
#[derive(Debug, Clone)]
struct Local {
    ty: Ty,
    mutable: bool,
}

pub fn check_program(
    prog: &Program,
    resolved: &ResolvedProgram,
    diags: &DiagnosticSink,
) {
    let mut ck = Checker::new(prog, resolved, diags);
    ck.check();
}

/// Type-check and return the inferred type of every expression, keyed by its
/// `Span`. Used by codegen (`emit`) so IR emission reuses the checker's
/// inference instead of re-deriving types.
pub fn collect_expr_types(
    prog: &Program,
    resolved: &ResolvedProgram,
    diags: &DiagnosticSink,
) -> HashMap<Span, Ty> {
    let mut ck = Checker::new(prog, resolved, diags);
    ck.check();
    std::mem::take(&mut ck.types)
}

struct Checker<'a> {
    prog: &'a Program,
    resolved: &'a ResolvedProgram,
    diags: &'a DiagnosticSink,
    /// Lexical scopes; bottom (last) is innermost.
    scopes: Vec<HashMap<String, Local>>,
    /// Inferred type of each checked expression (codegen side table).
    types: HashMap<Span, Ty>,
    /// Current enclosing class name and its instantiated type args.
    self_ty: Option<Ty>,
    /// Type args for the current class's generic params (name -> Ty).
    self_args: HashMap<String, Ty>,
    /// Current function/method return type.
    ret_ty: Ty,
    /// Current function's generic params (name only), for Var resolution.
    fn_generics: Vec<String>,
    /// Loop depth for `break`/`continue` validation.
    loop_depth: usize,
}

impl<'a> Checker<'a> {
    fn new(
        prog: &'a Program,
        resolved: &'a ResolvedProgram,
        diags: &'a DiagnosticSink,
    ) -> Checker<'a> {
        let scopes: Vec<HashMap<String, Local>> = vec![HashMap::new()];
        Checker {
            prog,
            resolved,
            diags,
            scopes,
            types: HashMap::new(),
            self_ty: None,
            self_args: HashMap::new(),
            ret_ty: Ty::Empty,
            fn_generics: Vec::new(),
            loop_depth: 0,
        }
    }

    fn check(&mut self) {
        // Item bodies: fn / test / const values.
        for item in &self.prog.items {
            match &item.kind {
                ItemKind::Fn(f) => self.check_fn_signature_bodies(f),
                ItemKind::Test(f) => self.check_fn_signature_bodies(f),
                ItemKind::Const(c) => {
                    let ty = self
                        .resolved
                        .consts
                        .iter()
                        .find(|f| f.name == c.name)
                        .map(|f| f.ty.clone())
                        .unwrap_or(Ty::Unknown);
                    let got = self.check_expr(&c.value);
                    self.check_assignable(&ty, &got, c.span, "const initializer");
                }
                ItemKind::Class(c) => self.check_class_bodies(c),
                ItemKind::Struct(s) => self.check_struct_bodies(s),
                ItemKind::Enum(_) | ItemKind::Interface(_) => {}
            }
        }
        self.check_interface_conformance();
    }

    // ---- scope helpers -----------------------------------------------------

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &str, ty: Ty, mutable: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), Local { ty, mutable });
        }
    }

    fn lookup(&self, name: &str) -> Option<&Local> {
        for scope in self.scopes.iter().rev() {
            if let Some(l) = scope.get(name) {
                return Some(l);
            }
        }
        None
    }

    fn err(&self, span: Span, msg: impl Into<String>) {
        self.diags.emit(Diagnostic::error_at(span, msg));
    }

    fn err_note(&self, span: Span, msg: impl Into<String>, note: impl Into<String>) {
        self.diags
            .emit(Diagnostic::error_at(span, msg).note(note));
    }

    // ---- assignability -----------------------------------------------------

    /// Is `got` acceptable where `want` is declared? Handles option/`none`,
    /// `Var` params, interface conformance, `Unknown` recovery.
    fn ok_types(&mut self, want: &Ty, got: &Ty) -> bool {
        if *got == Ty::Unknown || *want == Ty::Unknown {
            return true;
        }
        if let Ty::Var(n) = want {
            // A generic param accepts any single type (unified later).
            let _ = n;
            return true;
        }
        if got == want {
            return true;
        }
        // none -> T?
        if got.is_none() && want.is_option() {
            return true;
        }
        // T? expression can satisfy T?; T satisfies T? via implicit option.
        if want.is_option() {
            if got.is_option() {
                let (wi, gi) = (
                    want.inner_option().unwrap(),
                    got.inner_option().unwrap(),
                );
                return self.ok_types(&wi, &gi);
            }
            let inner = want.inner_option().unwrap();
            return self.ok_types(&inner, got);
        }
        // class implements interface?
        if let (Some(gotn), Some(wantn)) = (got.named(), want.named()) {
            if gotn != wantn {
                if self.implements_interface(gotn, wantn) {
                    return true;
                }
                if self.conforms_to(got, want) {
                    return true;
                }
            }
            // A subclass/descendant `got` is assignable to an ancestor `want`.
            if matches!(want, Ty::Class(..) | Ty::Struct(..))
                && self.is_ancestor(wantn, gotn)
            {
                return true;
            }
        }
        // Function types: parameters are contravariant, result covariant, and
        // unknown (`?`) types on either side widen to accept.
        if let (Ty::Fn(gp, gr), Ty::Fn(wp, wr)) = (got, want) {
            if gp.len() != wp.len() {
                return false;
            }
            for (g, w) in gp.iter().zip(wp.iter()) {
                if *g == Ty::Unknown || *w == Ty::Unknown {
                    continue;
                }
                // contravariance: callee params may accept a wider type
                if !self.ok_types(g, w) {
                    return false;
                }
            }
            if **gr == Ty::Unknown || **wr == Ty::Unknown {
                return true;
            }
            return self.ok_types(gr, wr);
        }
        // Empty (void) is never assignable to a value type; but a `return;` in
        // a void fn and statement-position block tails are handled elsewhere.
        false
    }

    fn check_assignable(&mut self, want: &Ty, got: &Ty, span: Span, what: &str) {
        if !self.ok_types(want, got) {
            self.err_note(
                span,
                format!(
                    "type mismatch in {what}: expected `{want}`, found `{got}`"
                ),
                format!(
                    "`{}` does not satisfy `{}`; use `as` for an explicit conversion",
                    got, want
                ),
            );
        }
    }

    // ---- type table lookups ------------------------------------------------

    fn class_table(&self, name: &str) -> Option<ClassTable> {
        match self.resolved.types.get(name) {
            Some(TypeTableEntry::Class(t)) | Some(TypeTableEntry::Struct(t)) => Some(t.clone()),
            _ => None,
        }
    }

    /// Substitute concrete args for a type's generic `Var`s.
    fn subst(&self, ty: &Ty, args: &HashMap<String, Ty>) -> Ty {
        match ty {
            Ty::Var(n) => args.get(n).cloned().unwrap_or_else(|| Ty::Var(n.clone())),
            Ty::Option(inner) => self.subst(inner, args).opt_of(),
            Ty::Class(n, a) => Ty::Class(n.clone(), a.iter().map(|t| self.subst(t, args)).collect()),
            Ty::Struct(n, a) => Ty::Struct(n.clone(), a.iter().map(|t| self.subst(t, args)).collect()),
            Ty::Enum(n, a) => Ty::Enum(n.clone(), a.iter().map(|t| self.subst(t, args)).collect()),
            Ty::Interface(n, a) => {
                Ty::Interface(n.clone(), a.iter().map(|t| self.subst(t, args)).collect())
            }
            Ty::List(inner) => Ty::List(Box::new(self.subst(inner, args))),
            Ty::Map(k, v) => Ty::Map(
                Box::new(self.subst(k, args)),
                Box::new(self.subst(v, args)),
            ),
            Ty::Tuple(items) => Ty::Tuple(items.iter().map(|t| self.subst(t, args)).collect()),
            Ty::Fn(ps, ret) => Ty::Fn(
                ps.iter().map(|t| self.subst(t, args)).collect(),
                Box::new(self.subst(ret, args)),
            ),
            Ty::Range(inner) => Ty::Range(Box::new(self.subst(inner, args))),
            other => other.clone(),
        }
    }

    /// The `(name, generic-args)` pair that backs this type for member lookup.
    fn type_key(&self, ty: &Ty) -> Option<(String, HashMap<String, Ty>)> {
        match ty {
            Ty::Class(n, a) | Ty::Struct(n, a) | Ty::Enum(n, a) => {
                let params = self
                    .resolved
                    .types
                    .get(n)
                    .map(|e| e.generics().to_vec())
                    .unwrap_or_default();
                let mut map = HashMap::new();
                for (i, p) in params.iter().enumerate() {
                    map.insert(p.clone(), a.get(i).cloned().unwrap_or(Ty::Unknown));
                }
                Some((n.clone(), map))
            }
            Ty::Interface(n, _) => Some((n.clone(), HashMap::new())),
            _ => None,
        }
    }

    fn implements_interface(&self, class: &str, iface: &str) -> bool {
        let mut pending: Vec<String> = vec![class.to_string()];
        let mut seen: Vec<String> = Vec::new();
        while let Some(name) = pending.pop() {
            if seen.contains(&name) {
                continue;
            }
            seen.push(name.clone());
            let table = match self.resolved.types.get(&name) {
                Some(TypeTableEntry::Class(t)) | Some(TypeTableEntry::Struct(t)) => t.clone(),
                _ => continue,
            };
            if table.implements.iter().any(|t| t.named() == Some(iface)) {
                return true;
            }
            if let Some(p) = table.extends.as_ref().and_then(|t| t.named().map(str::to_string)) {
                pending.push(p);
            }
        }
        false
    }

    /// True if `desc` extends/structurally inherits `anc` through the class
    /// chain (used for upcast/downcast checks in casts).
    fn is_ancestor(&self, anc: &str, desc: &str) -> bool {
        let mut pending: Vec<String> = vec![desc.to_string()];
        let mut seen: Vec<String> = Vec::new();
        while let Some(name) = pending.pop() {
            if name == anc {
                return true;
            }
            if seen.contains(&name) {
                continue;
            }
            seen.push(name.clone());
            let table = match self.resolved.types.get(&name) {
                Some(TypeTableEntry::Class(t)) | Some(TypeTableEntry::Struct(t)) => t.clone(),
                _ => continue,
            };
            if let Some(p) = table.extends.as_ref().and_then(|t| t.named().map(str::to_string)) {
                pending.push(p);
            }
        }
        false
    }

    /// True if `got`'s declared interfaces include `want` (or an ancestor).
    fn conforms_to(&self, got: &Ty, want: &Ty) -> bool {
        let (Some(gotn), Some(wantn)) = (got.named(), want.named()) else {
            return false;
        };
        if wantn == gotn {
            return true;
        }
        if !matches!(want, Ty::Interface(..)) {
            return false;
        }
        self.implements_interface(gotn, wantn)
    }

    // ---- member lookup -----------------------------------------------------

    fn find_field(
        &self,
        class: &str,
        name: &str,
        args: &HashMap<String, Ty>,
    ) -> Option<(crate::resolve::FieldInfo, bool)> {
        let mut chain: Vec<String> = vec![class.to_string()];
        let mut seen: Vec<String> = Vec::new();
        while let Some(cname) = chain.pop() {
            if seen.contains(&cname) {
                continue;
            }
            seen.push(cname.clone());
            let table = match self.resolved.types.get(&cname) {
                Some(TypeTableEntry::Class(t)) | Some(TypeTableEntry::Struct(t)) => t.clone(),
                _ => continue,
            };
            if let Some(f) = table.fields.iter().find(|f| f.name == name) {
                return Some((f.clone(), true));
            }
            if let Some(f) = table.consts.iter().find(|f| f.name == name) {
                return Some((f.clone(), true));
            }
            if let Some(p) = table.extends.as_ref().and_then(|t| t.named().map(str::to_string)) {
                chain.push(p);
            }
        }
        let _ = args;
        None
    }

    fn find_method(
        &self,
        class: &str,
        name: &str,
    ) -> Option<CallableInfo> {
        let mut chain: Vec<String> = vec![class.to_string()];
        let mut seen: Vec<String> = Vec::new();
        while let Some(cname) = chain.pop() {
            if seen.contains(&cname) {
                continue;
            }
            seen.push(cname.clone());
            let table = match self.resolved.types.get(&cname) {
                Some(TypeTableEntry::Class(t)) | Some(TypeTableEntry::Struct(t)) => t.clone(),
                _ => continue,
            };
            if let Some(m) = table.methods.iter().find(|m| m.name == name) {
                return Some(m.clone());
            }
            if let Some(p) = table.extends.as_ref().and_then(|t| t.named().map(str::to_string)) {
                chain.push(p);
            }
        }
        None
    }

    fn find_property(&self, class: &str, name: &str) -> Option<crate::resolve::PropertyInfo> {
        let table = self.class_table(class)?;
        table.properties.iter().find(|p| p.name == name).cloned()
    }

    // ---- bodies ------------------------------------------------------------

    fn check_fn_signature_bodies(&mut self, f: &FnDecl) {
        self.push_scope();
        self.fn_generics = f.generics.iter().map(|g| g.name.clone()).collect();
        self.ret_ty = self
            .resolved
            .fns
            .get(&f.name)
            .and_then(|fns| fns.first())
            .map(|c| c.ret.clone())
            .unwrap_or(Ty::Empty);
        for p in &f.params {
            let ty = self.resolve_param_ty(p);
            self.declare(&p.name, ty, true);
            if let Some(d) = &p.default {
                let dt = self.check_expr(d);
                self.check_assignable(
                    &self.resolve_param_ty(p),
                    &dt,
                    p.span,
                    "default argument",
                );
            }
        }
        if let Some(body) = &f.body {
            self.check_fn_body(body);
        }
        self.ret_ty = Ty::Empty;
        self.fn_generics = Vec::new();
        self.pop_scope();
    }

    fn resolve_param_ty(&self, p: &Param) -> Ty {
        p.ty
            .as_ref()
            .map(|t| {
                let mut generics = self.fn_generics.clone();
                if let Some(gt) = self.self_ty.as_ref().and_then(|t| self.type_key(t)) {
                    generics.extend(gt.1.keys().cloned());
                }
                self.resolved_fn_ty(t, &generics)
            })
            .unwrap_or(Ty::Unknown)
    }

    fn resolved_fn_ty(&self, te: &TypeExpr, generics: &[String]) -> Ty {
        // Resolve against the final module tables, so user-defined types and
        // this module's generics are visible.
        let mut g = generics.to_vec();
        if let Some(gt) = self.self_ty.as_ref().and_then(|t| self.type_key(t)) {
            for k in gt.1.keys() {
                if !g.contains(k) {
                    g.push(k.clone());
                }
            }
        }
        self.resolved.resolve_ty(te, self.instantiated_generics(&g), self.diags)
    }

    /// Generics with `self_args` substituted for the current class, so method
    /// param types that reference class generics resolve to the instantiated
    /// type (or stay `Var` where possible).
    fn instantiated_generics<'b>(&self, generics: &'b [String]) -> &'b [String] {
        let _ = self.self_args.is_empty();
        generics
    }

    fn check_fn_body(&mut self, body: &FnBody) {
        match body {
            FnBody::Block(b) => {
                let expr_ty = self.check_block(b);
                self.check_return_expr(&expr_ty, b.span);
            }
            FnBody::Expr(e) => {
                let t = self.check_expr(e);
                self.check_return_expr(&t, e.span);
            }
        }
    }

    fn check_return_expr(&mut self, got: &Ty, span: Span) {
        if *got == Ty::Empty {
            // Tail expr was a statement block; nothing to check for void fns.
        }
        if self.ret_ty != Ty::Empty && *got != Ty::Empty {
            let want = self.ret_ty.clone();
            self.check_assignable(&want, got, span, "return value");
        }
    }

    fn check_block(&mut self, b: &Block) -> Ty {
        self.push_scope();
        let mut last = Ty::Empty;
        for s in &b.stmts {
            last = self.check_stmt(s);
        }
        if let Some(e) = &b.expr {
            last = self.check_expr(e);
        }
        self.pop_scope();
        last
    }

    fn check_stmt(&mut self, s: &Stmt) -> Ty {
        match s {
            Stmt::Let {
                pattern,
                ty,
                init,
                mutable,
                span,
            } => {
                let annotated = ty
                    .as_ref()
                    .map(|t| self.resolved_fn_ty(t, &self.fn_generics));
                let inferred = init.as_ref().map(|e| self.check_expr(e));
                let final_ty = match (&annotated, &inferred) {
                    (Some(a), Some(i)) => {
                        self.check_assignable(a, i, *span, "initializer");
                        a.clone()
                    }
                    (Some(a), None) => a.clone(),
                    (None, Some(i)) => i.clone(),
                    (None, None) => Ty::Unknown,
                };
                self.bind_pattern(pattern, &final_ty, *mutable);
                final_ty
            }
            Stmt::Const { name, ty, value, span } => {
                let vt = self.check_expr(value);
                if let Some(t) = ty {
                    let tt = self.resolved_fn_ty(t, &self.fn_generics);
                    self.check_assignable(&tt, &vt, *span, "const initializer");
                    self.declare(name, tt, false);
                } else {
                    self.declare(name, vt.clone(), false);
                }
                vt
            }
            Stmt::Return { value, span } => {
                let vt = match value {
                    Some(e) => self.check_expr(e),
                    None => Ty::Empty,
                };
                if self.ret_ty == Ty::Empty {
                    if value.is_some() {
                        self.err_note(
                            *span,
                            "cannot return a value from a function with no return type",
                            "declare a return type with `-> T`, or remove the value",
                        );
                    }
                } else {
                    self.check_assignable(&self.ret_ty.clone(), &vt, *span, "return value");
                }
                Ty::Empty
            }
            Stmt::Break { span } | Stmt::Continue { span } => {
                if self.loop_depth == 0 {
                    self.err(*span, "`break`/`continue` used outside of a loop");
                }
                Ty::Empty
            }
            Stmt::While { cond, body, .. } => {
                self.check_bool_cond(cond);
                self.loop_depth += 1;
                self.check_block(body);
                self.loop_depth -= 1;
                Ty::Empty
            }
            Stmt::For { header, body, .. } => {
                self.check_for_header(header);
                self.loop_depth += 1;
                self.check_block(body);
                self.loop_depth -= 1;
                Ty::Empty
            }
            Stmt::Expr(e) => {
                let t = self.check_expr(e);
                let _ = t;
                Ty::Empty
            }
            Stmt::Empty(_) => Ty::Empty,
        }
    }

    fn check_bool_cond(&mut self, e: &Expr) {
        let t = self.check_expr(e);
        if t != Ty::Unknown && t != Ty::Bool {
            self.err_note(e.span, "condition must be a `bool`", format!("found `{t}`"));
        }
    }

    fn check_for_header(&mut self, header: &ForHeader) {
        match header {
            ForHeader::In { pattern, sequence } => {
                let st = self.check_expr(sequence);
                let elem = match &st {
                    Ty::List(_) | Ty::Map(_, _) | Ty::Range(_) => {
                        st.elem().unwrap_or(Ty::Unknown)
                    }
                    Ty::String => Ty::Char,
                    Ty::Unknown => Ty::Unknown,
                    other => {
                        self.err_note(
                            sequence.span,
                            format!(
                                "`for (x in ...)` requires a sequence (List, string, Map, or range), found `{other}`"
                            ),
                            format!("found `{other}`"),
                        );
                        Ty::Unknown
                    }
                };
                self.bind_pattern(pattern, &elem, true);
            }
            ForHeader::Range { init, cond, step } => {
                let _ = self.check_stmt(init);
                self.check_bool_cond(cond);
                let _ = self.check_expr(step);
            }
        }
    }

    fn bind_pattern(&mut self, p: &Pattern, ty: &Ty, mutable: bool) {
        match p {
            Pattern::Wildcard => {}
            Pattern::Binding { name, ty: ann, .. } => {
                // A pattern annotation like `let m: int?` is parsed into the
                // pattern; it wins over the inferred value type and is checked
                // against it.
                let final_ty = match ann {
                    Some(te) => {
                        let at = self.resolved_fn_ty(te, &self.fn_generics);
                        if *ty != Ty::Unknown {
                            self.check_assignable(&at, ty, p.span(), "binding");
                        }
                        at
                    }
                    None => ty.clone(),
                };
                self.declare(name, final_ty, mutable);
            }
            Pattern::Literal(_) => {}
            Pattern::Tuple(parts) => {
                if let Ty::Tuple(tys) = ty {
                    for (i, part) in parts.iter().enumerate() {
                        let t = tys.get(i).cloned().unwrap_or(Ty::Unknown);
                        self.bind_pattern(part, &t, mutable);
                    }
                } else if *ty != Ty::Unknown {
                    self.err(
                        p.span(),
                        "tuple pattern does not match a tuple value",
                    );
                }
            }
            Pattern::Variant { path, payloads } => {
                // `Enum.Variant(p1, p2)` binding. If only the variant name is
                // given, the enum type is inferred from the value.
                let (enum_name, variant_name) = match path.len() {
                    2 => (path[0].clone(), path[1].clone()),
                    1 => match ty {
                        Ty::Enum(en, _) => (en.clone(), path[0].clone()),
                        _ => {
                            if *ty != Ty::Unknown {
                                self.err(
                                    p.span(),
                                    "enum variant pattern requires a value of enum type",
                                );
                            }
                            return;
                        }
                    },
                    _ => return,
                };
                if *ty == Ty::Unknown {
                    return;
                }
                if let Some(TypeTableEntry::Enum(t)) = self.resolved.types.get(&enum_name) {
                    if let Some((_, ftypes, _)) =
                        t.variants.iter().find(|(n, ..)| n == &variant_name)
                    {
                        for (i, part) in payloads.iter().enumerate() {
                            let ft = ftypes.get(i).cloned().unwrap_or(Ty::Unknown);
                            self.bind_pattern(part, &ft, mutable);
                        }
                    } else {
                        self.err(p.span(), format!("enum `{enum_name}` has no variant `{variant_name}`"));
                    }
                }
            }
            Pattern::Or(parts) => {
                for part in parts {
                    self.bind_pattern(part, ty, mutable);
                }
            }
        }
    }

    fn check_class_bodies(&mut self, c: &ClassDecl) {
        let table = self.class_table(&c.name).unwrap_or_else(|| ClassTable {
            name: c.name.clone(),
            span: c.span,
            visibility: c.visibility,
            generics: c.generics.iter().map(|g| g.name.clone()).collect(),
            extends: None,
            implements: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            properties: Vec::new(),
            ctor: None,
            consts: Vec::new(),
        });
        self.self_ty = Some(Ty::Class(c.name.clone(), table.generics.iter().cloned().map(Ty::Var).collect()));
        self.self_args = table
            .generics
            .iter()
            .map(|g| (g.clone(), Ty::Var(g.clone())))
            .collect();

        for m in &c.members {
            match m {
                ClassMember::Field { init: Some(e), ty, span, .. } => {
                    let ft = self.check_expr(e);
                    let w = ty
                        .as_ref()
                        .map(|t| self.resolved_fn_ty(t, &table.generics))
                        .or_else(|| {
                            self.resolved
                                .types
                                .get(&c.name)
                                .and_then(|e| match e {
                                    TypeTableEntry::Class(t) | TypeTableEntry::Struct(t) => t
                                        .fields
                                        .iter()
                                        .find(|f| f.name == m.name())
                                        .map(|f| {
                                            self.subst(&f.ty, &self.self_args)
                                        }),
                                    _ => None,
                                })
                        });
                    if let Some(w) = w {
                        self.check_assignable(&w, &ft, *span, "field initializer");
                    }
                }
                ClassMember::Field { init: None, .. } => {}
                ClassMember::Const { value, ty, span, .. } => {
                    let vt = self.check_expr(value);
                    if let Some(t) = ty {
                        let tt = self.resolved_fn_ty(t, &table.generics);
                        self.check_assignable(&tt, &vt, *span, "const initializer");
                    }
                }
                ClassMember::Method(md) => self.check_method_body(md, &table),
                ClassMember::Constructor(cd) => {
                    self.push_scope();
                    self.ret_ty = Ty::Empty;
                    self.fn_generics = Vec::new();
                    for p in &cd.params {
                        let pt = self.resolve_param_ty(p);
                        self.declare(&p.name, pt, true);
                    }
                    self.check_block(&cd.body);
                    self.pop_scope();
                }
                ClassMember::Property(p) => {
                    for acc in [&p.get, &p.set].into_iter().flatten() {
                        self.push_scope();
                        self.ret_ty = p
                            .ty
                            .as_ref()
                            .map(|t| self.resolved_fn_ty(t, &table.generics))
                            .unwrap_or(Ty::Unknown);
                        self.fn_generics = Vec::new();
                        let t = match acc {
                            PropertyAccessor::Expr(e) => self.check_expr(e),
                            PropertyAccessor::Block(b) => self.check_block(b),
                        };
                        let want = self.ret_ty.clone();
                        self.check_assignable(&want, &t, p.span, "property accessor");
                        self.pop_scope();
                    }
                }
                _ => {}
            }
        }
        self.self_ty = None;
        self.self_args = HashMap::new();
    }

    fn check_struct_bodies(&mut self, s: &StructDecl) {
        let table = self.class_table(&s.name).unwrap_or_else(|| ClassTable {
            name: s.name.clone(),
            span: s.span,
            visibility: s.visibility,
            generics: s.generics.iter().map(|g| g.name.clone()).collect(),
            extends: None,
            implements: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            properties: Vec::new(),
            ctor: None,
            consts: Vec::new(),
        });
        self.self_ty = Some(Ty::Struct(s.name.clone(), table.generics.iter().cloned().map(Ty::Var).collect()));
        self.self_args = table
            .generics
            .iter()
            .map(|g| (g.clone(), Ty::Var(g.clone())))
            .collect();
        for m in &s.members {
            match m {
                ClassMember::Field { init: Some(e), ty, span, .. } => {
                    let ft = self.check_expr(e);
                    let w = ty
                        .as_ref()
                        .map(|t| self.resolved_fn_ty(t, &table.generics))
                        .or_else(|| {
                            self.resolved
                                .types
                                .get(&s.name)
                                .and_then(|e| match e {
                                    TypeTableEntry::Class(t) | TypeTableEntry::Struct(t) => t
                                        .fields
                                        .iter()
                                        .find(|f| f.name == m.name())
                                        .map(|f| self.subst(&f.ty, &self.self_args)),
                                    _ => None,
                                })
                        });
                    if let Some(w) = w {
                        self.check_assignable(&w, &ft, *span, "field initializer");
                    }
                }
                ClassMember::Field { init: None, .. } => {}
                ClassMember::Method(md) => self.check_method_body(md, &table),
                _ => {}
            }
        }
        self.self_ty = None;
        self.self_args = HashMap::new();
    }

    fn check_method_body(&mut self, md: &MethodDecl, table: &ClassTable) {
        self.push_scope();
        self.fn_generics = md.generics.iter().map(|g| g.name.clone()).collect();
        self.ret_ty = self
            .resolved
            .types
            .get(&table.name)
            .and_then(|e| match e {
                TypeTableEntry::Class(t) | TypeTableEntry::Struct(t) => t
                    .methods
                    .iter()
                    .find(|m| m.name == md.name)
                    .map(|m| m.ret.clone()),
                _ => None,
            })
            .unwrap_or(Ty::Empty);
        // Declare `this` in scope.
        if let Some(st) = &self.self_ty {
            self.declare("this", st.clone(), false);
        }
        for p in &md.params {
            let pt = self.resolve_param_ty(p);
            self.declare(&p.name, pt.clone(), true);
            if let Some(d) = &p.default {
                let dt = self.check_expr(d);
                self.check_assignable(&pt, &dt, p.span, "default argument");
            }
        }
        if let Some(body) = &md.body {
            self.check_fn_body(body);
        }
        self.ret_ty = Ty::Empty;
        self.fn_generics = Vec::new();
        self.pop_scope();
    }

    fn check_interface_conformance(&mut self) {
        for entry in self.resolved.types.values() {
            let (name, implements) = match entry {
                TypeTableEntry::Class(t) | TypeTableEntry::Struct(t) => {
                    (t.name.clone(), t.implements.clone())
                }
                TypeTableEntry::Enum(t) => (t.name.clone(), t.implements.clone()),
                TypeTableEntry::Interface(_) => continue,
            };
            for i in implements {
                let Some(iface_name) = i.named().map(str::to_string) else {
                    continue;
                };
                let Some(TypeTableEntry::Interface(iface)) =
                    self.resolved.types.get(&iface_name)
                else {
                    continue;
                };
                for member in &iface.members {
                    if member.is_property {
                        let has = self
                            .find_property(&name, &member.name)
                            .or_else(|| self.find_field(&name, &member.name, &HashMap::new()).map(|(f, _)| crate::resolve::PropertyInfo {
                                name: f.name,
                                visibility: f.visibility,
                                is_static: f.is_static,
                                ty: f.ty,
                                has_get: true,
                                has_set: f.mutable,
                                span: f.span,
                            }));
                        if has.is_none() {
                            self.err(
                                member.span,
                                format!(
                                    "`{name}` declares `implements {iface_name}` but has no member `{}`",
                                    member.name
                                ),
                            );
                        }
                    } else if self.find_method(&name, &member.name).is_none() {
                        self.err(
                            member.span,
                            format!(
                                "`{name}` declares `implements {iface_name}` but has no method `{}`",
                                member.name
                            ),
                        );
                    }
                }
            }
        }
    }

    // ---- expressions ------------------------------------------------------

    fn check_expr(&mut self, e: &Expr) -> Ty {
        let t = self.check_expr_impl(e);
        self.types.insert(e.span, t.clone());
        t
    }

    fn check_expr_impl(&mut self, e: &Expr) -> Ty {
        match &e.kind {
            ExprKind::Lit(Lit::String(parts)) => self.string_ty(parts),
            ExprKind::Lit(l) => self.lit_ty(l),
            ExprKind::Ident(name) => self.ident_ty(e, name),
            ExprKind::This => self
                .self_ty
                .clone()
                .unwrap_or_else(|| {
                    self.err(e.span, "`this` used outside of a class body");
                    Ty::Unknown
                }),
            ExprKind::Super => self.super_ty(e),
            ExprKind::Call { callee, args } => self.check_call(e, callee, args),
            ExprKind::Member { object, name } => self.check_member(e, object, name),
            ExprKind::OptAccess { object, name } => self.check_opt_access(e, object, name),
            ExprKind::OptUnwrap(inner) => self.check_opt_unwrap(e, inner),
            ExprKind::Index { object, index } => self.check_index(e, object, index),
            ExprKind::Binary { op, lhs, rhs } => self.check_binary(e, *op, lhs, rhs),
            ExprKind::Unary { op, operand } => self.check_unary(e, *op, operand),
            ExprKind::Assign { target, op, value } => self.check_assign(e, target, *op, value),
            ExprKind::Lambda {
                params,
                return_ty,
                body,
                ..
            } => self.check_lambda(e, params, return_ty.as_ref(), body),
            ExprKind::If {
                cond,
                then,
                else_else,
            } => self.check_if(e, cond, then, else_else.as_deref()),
            ExprKind::Match { scrutinee, arms } => self.check_match(e, scrutinee, arms),
            ExprKind::Await(inner) => {
                let _ = self.check_expr(inner);
                Ty::Unknown
            }
            ExprKind::GenericCall { name, type_args } => {
                let _ = type_args;
                self.generic_fn_ty(e, name)
            }
            ExprKind::Cast { expr, ty, kind } => self.check_cast(e, expr, ty, kind.clone()),
            ExprKind::Unsafe(b) => self.check_block(b),
            ExprKind::Block(b) => self.check_block(b),
            ExprKind::Tuple(items) => {
                Ty::Tuple(items.iter().map(|i| self.check_expr(i)).collect())
            }
            ExprKind::Array(items) => {
                if items.is_empty() {
                    Ty::List(Box::new(Ty::Unknown))
                } else {
                    let elem = self.check_expr(&items[0]);
                    for i in items.iter().skip(1) {
                        let t = self.check_expr(i);
                        self.check_assignable(&elem, &t, i.span, "array element");
                    }
                    Ty::List(Box::new(elem))
                }
            }
            ExprKind::Map(pairs) => {
                if pairs.is_empty() {
                    Ty::Map(Box::new(Ty::Unknown), Box::new(Ty::Unknown))
                } else {
                    let k = self.check_expr(&pairs[0].0);
                    if k != Ty::Unknown && k != Ty::String {
                        self.err(
                            pairs[0].0.span,
                            "map keys must be `string` values",
                        );
                    }
                    let vt = self.check_expr(&pairs[0].1);
                    for (kk, vv) in pairs.iter().skip(1) {
                        let tk = self.check_expr(kk);
                        let tv = self.check_expr(vv);
                        self.check_assignable(&k, &tk, kk.span, "map key");
                        self.check_assignable(&vt, &tv, vv.span, "map value");
                    }
                    Ty::Map(Box::new(k), Box::new(vt))
                }
            }
            ExprKind::Range { start, end, incl } => {
                let st = self.check_expr(start);
                let _ = self.check_expr(end);
                let _ = incl;
                Ty::Range(Box::new(st))
            }
        }
    }

    fn lit_ty(&self, lit: &Lit) -> Ty {
        match lit {
            Lit::Int { .. } => Ty::Int,
            Lit::Float { .. } => Ty::Float,
            Lit::String(_) => Ty::String,
            Lit::Char(_) => Ty::Char,
            Lit::Bool(_) => Ty::Bool,
            Lit::None => Ty::None,
        }
    }

    /// Type an interpolated string: check every embedded expression (so the
    /// codegen side table has its type and unresolved names are reported), and
    /// the literal itself is a `string`.
    fn string_ty(&mut self, parts: &[StrPart]) -> Ty {
        for part in parts {
            if let StrPart::Expr(e) = part {
                let _ = self.check_expr(e);
            }
        }
        Ty::String
    }

    fn ident_ty(&mut self, e: &Expr, name: &str) -> Ty {
        if let Some(l) = self.lookup(name) {
            return l.ty.clone();
        }
        if let Some(fns) = self.resolved.fns.get(name) {
            let t = self.fn_sig(fns);
            let _ = e;
            return t;
        }
        if let Some(c) = self.resolved.consts.iter().find(|c| c.name == name) {
            return c.ty.clone();
        }
        // Implicit receiver: a bare field/property/method name inside a
        // class or struct body refers to `this.name`.
        if let Some(st) = &self.self_ty {
            if let Some((cn, args_map)) = self.type_key(st) {
                if let Some(f) = self.find_field(&cn, name, &args_map) {
                    return self.subst(&f.0.ty, &args_map);
                }
                if let Some(p) = self.find_property(&cn, name) {
                    return p.ty.clone();
                }
                if let Some(m) = self.find_method(&cn, name) {
                    return Ty::Fn(
                        m.params.iter().map(|p| p.ty.clone()).collect(),
                        Box::new(m.ret.clone()),
                    );
                }
            }
        }
        // Type name used as a constructor/static receiver.
        if let Some(entry) = self.resolved.types.get(name) {
            return match entry {
                TypeTableEntry::Class(t) => {
                    Ty::Class(t.name.clone(), t.generics.iter().cloned().map(Ty::Var).collect())
                }
                TypeTableEntry::Struct(t) => Ty::Struct(
                    t.name.clone(),
                    t.generics.iter().cloned().map(Ty::Var).collect(),
                ),
                TypeTableEntry::Enum(t) => {
                    Ty::Enum(t.name.clone(), t.generics.iter().cloned().map(Ty::Var).collect())
                }
                TypeTableEntry::Interface(t) => Ty::Interface(
                    t.name.clone(),
                    t.generics.iter().cloned().map(Ty::Var).collect(),
                ),
            };
        }
        self.err_note(
            e.span,
            format!("use of undeclared name `{name}`"),
            "names must be declared before use",
        );
        Ty::Unknown
    }

    fn fn_sig(&self, fns: &[CallableInfo]) -> Ty {
        let Some(f) = fns.first() else {
            return Ty::Unknown;
        };
        Ty::Fn(
            f.params.iter().map(|p| p.ty.clone()).collect(),
            Box::new(f.ret.clone()),
        )
    }

    fn super_ty(&mut self, e: &Expr) -> Ty {
        let Some(st) = &self.self_ty else {
            self.err(e.span, "`super` used outside of a class body");
            return Ty::Unknown;
        };
        let Some((name, _)) = self.type_key(st) else {
            return Ty::Unknown;
        };
        let Some(table) = self.class_table(&name) else {
            return Ty::Unknown;
        };
        table.extends.unwrap_or_else(|| {
            self.err(e.span, "this class has no parent to call `super` on");
            Ty::Unknown
        })
    }

    fn check_call(&mut self, e: &Expr, callee: &Expr, args: &[CallArg]) -> Ty {
        // `Enum.Variant(...)` constructor call.
        if let ExprKind::Member { object, name } = &callee.kind {
            if let ExprKind::Ident(enum_name) = &object.kind {
                if let Some(TypeTableEntry::Enum(t)) = self.resolved.types.get(enum_name) {
                    let ftypes = t
                        .variants
                        .iter()
                        .find(|(n, ..)| n == name)
                        .map(|(_, ft, _)| ft.clone());
                    let ftypes = match ftypes {
                        Some(ft) => ft,
                        None => {
                            self.err(
                                callee.span,
                                format!("enum `{enum_name}` has no variant `{name}`"),
                            );
                            return Ty::Unknown;
                        }
                    };
                    self.check_args(e, &ftypes, args);
                    return Ty::Enum(t.name.clone(), Vec::new());
                }
            }
        }

        // Constructor call: `TypeName(args)` where callee is a type name.
        if let ExprKind::Ident(cname) = &callee.kind {
            if let Some(entry) = self.resolved.types.get(cname) {
                let ct = match entry {
                    TypeTableEntry::Class(c) => {
                        Ty::Class(c.name.clone(), c.generics.iter().cloned().map(Ty::Var).collect())
                    }
                    TypeTableEntry::Struct(s) => Ty::Struct(
                        s.name.clone(),
                        s.generics.iter().cloned().map(Ty::Var).collect(),
                    ),
                    TypeTableEntry::Interface(_) | TypeTableEntry::Enum(_) => {
                        self.err(e.span, format!("`{cname}` cannot be constructed directly"));
                        return Ty::Unknown;
                    }
                };
                if let Some((cn, _)) = self.type_key(&ct) {
                    if let Some(m) = self.find_method(&cn, cname) {
                        let params: Vec<Ty> =
                            m.params.iter().map(|p| p.ty.clone()).collect();
                        self.check_args(e, &params, args);
                        return m.ret.clone();
                    }
                }
                if let Some(c) = self.class_table(cname) {
                    let params: Vec<Ty> = c
                        .fields
                        .iter()
                        .map(|f| f.ty.clone())
                        .collect();
                    self.check_args(e, &params, args);
                }
                return ct;
            }
        }

        // Regular function call by name (user or builtin).
        if let ExprKind::Ident(cname) = &callee.kind {
            if let Some(fns) = self.resolved.fns.get(cname) {
                let first = fns
                    .iter()
                    // Prefer the signature whose fixed-param count best fits
                    // the positional arguments; variadic rest stays loose.
                    .min_by_key(|f| {
                        let fixed = f.params.iter().filter(|p| !p.rest).count();
                        fixed.abs_diff(args.iter().filter(|a| !a.spread).count())
                    })
                    .cloned()
                    .unwrap_or_else(|| fns[0].clone());
                self.check_args_info(e, &first.params, args);
                return first.ret.clone();
            }
        }

        let ct = self.check_expr(callee);
        match &ct {
            Ty::Fn(params, ret) => {
                self.check_args(e, params, args);
                *ret.clone()
            }
            Ty::Unknown => Ty::Unknown,
            _ => {
                self.err_note(
                    e.span,
                    format!("attempt to call a non-function value of type `{ct}`"),
                    "only functions and type constructors can be called",
                );
                Ty::Unknown
            }
        }
    }

    fn generic_fn_ty(&mut self, e: &Expr, name: &str) -> Ty {
        let Some(fns) = self.resolved.fns.get(name) else {
            self.err(e.span, format!("unknown generic function `{name}`"));
            return Ty::Unknown;
        };
        self.fn_sig(fns)
    }

    fn check_args(&mut self, e: &Expr, params: &[Ty], args: &[CallArg]) {
        // Smallest overload wins is not implemented; we check against the
        // declared signature literally (positional, then named/rest).
        let mut position = 0usize;
        for a in args {
            if let Some(name) = &a.name {
                self.err_note(
                    a.span,
                    format!("named argument `{name}` is not supported for this call"),
                    "positional arguments are expected here",
                );
                let _ = self.check_expr(&a.value);
                position += 1;
                continue;
            }
            if a.spread {
                let _ = self.check_expr(&a.value);
                position += 1;
                continue;
            }
            let want = match params.get(position) {
                Some(p) => p.clone(),
                None => {
                    let _ = self.check_expr(&a.value);
                    self.err(e.span, "too many arguments in call");
                    position += 1;
                    continue;
                }
            };
            let got = self.check_expr(&a.value);
            self.check_assignable(&want, &got, a.span, "argument");
            position += 1;
        }
    }

    /// Variadic-aware argument checking against `ParamInfo`s. Params flagged
    /// `rest` absorb any remaining positional arguments without arity errors.
    fn check_args_info(&mut self, e: &Expr, params: &[crate::resolve::ParamInfo], args: &[CallArg]) {
        let rest_ty = params
            .iter()
            .rev()
            .find(|p| p.rest)
            .map(|p| p.ty.clone());
        let fixed = params.iter().filter(|p| !p.rest).count();
        let mut position = 0usize;
        for a in args {
            if let Some(name) = &a.name {
                self.err_note(
                    a.span,
                    format!("named argument `{name}` is not supported for this call"),
                    "positional arguments are expected here",
                );
                let _ = self.check_expr(&a.value);
                position += 1;
                continue;
            }
            if a.spread {
                let _ = self.check_expr(&a.value);
                position += 1;
                continue;
            }
            let want = if position < fixed {
                params[position].ty.clone()
            } else {
                match &rest_ty {
                    Some(rt) => rt.clone(),
                    None => {
                        let _ = self.check_expr(&a.value);
                        self.err(e.span, "too many arguments in call");
                        position += 1;
                        continue;
                    }
                }
            };
            let got = self.check_expr(&a.value);
            self.check_assignable(&want, &got, a.span, "argument");
            position += 1;
        }
        if position < fixed && args.iter().all(|a| a.name.is_none()) {
            // Missing required arguments.
            let missing = &params[position..fixed];
            if !missing.is_empty() {
                self.err_note(
                    e.span,
                    format!(
                        "missing argument{} for parameter{} `{}`",
                        if missing.len() == 1 { "" } else { "s" },
                        if missing.len() == 1 { "" } else { "s" },
                        missing
                            .iter()
                            .map(|p| p.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    "all required parameters must be supplied",
                );
            }
        }
    }

    fn check_member(&mut self, e: &Expr, object: &Expr, name: &str) -> Ty {
        // Type-qualified static access: `Foo.bar`.
        if let ExprKind::Ident(tname) = &object.kind {
            if let Some(entry) = self.resolved.types.get(tname) {
                let gname = entry.name().to_string();
                if let Some(f) = self.find_field(&gname, name, &HashMap::new()) {
                    return self.subst(&f.0.ty, &HashMap::new());
                }
                if let Some(p) = self.find_property(&gname, name) {
                    return p.ty.clone();
                }
                if let Some(m) = self.find_method(&gname, name) {
                    return Ty::Fn(
                        m.params.iter().map(|p| p.ty.clone()).collect(),
                        Box::new(m.ret.clone()),
                    );
                }
                self.err(
                    e.span,
                    format!("no member `{name}` on type `{tname}`"),
                );
                return Ty::Unknown;
            }
        }

        let ot = self.check_expr(object);
        if let Ty::List(inner) = &ot {
            // Builtin list methods (slice subset).
            return match name {
                "push" => Ty::Fn(vec![inner.as_ref().clone()], Box::new(Ty::Empty)),
                "pop" => Ty::Fn(vec![], Box::new(inner.as_ref().clone())),
                other => {
                    self.err(
                        e.span,
                        format!("no member `{other}` on `List<{}>`", inner.bare_name()),
                    );
                    Ty::Unknown
                }
            };
        }
        if let Ty::Map(k, v) = &ot {
            // Builtin map methods (slice subset).
            return match name {
                "has" => Ty::Fn(vec![k.as_ref().clone()], Box::new(Ty::Bool)),
                "keys" => Ty::Fn(vec![], Box::new(Ty::List(k.clone()))),
                "values" => Ty::Fn(vec![], Box::new(Ty::List(v.clone()))),
                other => {
                    self.err(
                        e.span,
                        format!(
                            "no member `{other}` on `Map<{}, {}>`",
                            k.bare_name(),
                            v.bare_name()
                        ),
                    );
                    Ty::Unknown
                }
            };
        }
        let Some((class, args_map)) = self.type_key(&ot) else {
            self.err_note(
                e.span,
                format!("cannot access member `{name}` on value of type `{ot}`"),
                "member access requires a class, struct, enum, or interface value",
            );
            return Ty::Unknown;
        };
        if let Some(f) = self.find_field(&class, name, &args_map) {
            return self.subst(&f.0.ty, &args_map);
        }
        if let Some(p) = self.find_property(&class, name) {
            return self.subst(&p.ty, &args_map);
        }
        if let Some(m) = self.find_method(&class, name) {
            return Ty::Fn(
                m.params.iter().map(|p| self.subst(&p.ty, &args_map)).collect(),
                Box::new(self.subst(&m.ret, &args_map)),
            );
        }
        self.err_note(
            e.span,
            format!("no member `{name}` on `{class}`"),
            format!("`{class}` defines fields, properties, and methods; check the spelling"),
        );
        Ty::Unknown
    }

    fn check_opt_access(&mut self, e: &Expr, object: &Expr, name: &str) -> Ty {
        let ot = self.check_expr(object);
        let Some(inner) = ot.inner_option() else {
            self.err_note(
                e.span,
                format!("`?.` on a non-option value of type `{ot}`"),
                "optional chaining requires an `Option` receiver",
            );
            return Ty::Unknown;
        };
        let Some((class, args_map)) = self.type_key(&inner) else {
            return Ty::Unknown;
        };
        let member_ty = if let Some(f) = self.find_field(&class, name, &args_map) {
            self.subst(&f.0.ty, &args_map)
        } else if let Some(p) = self.find_property(&class, name) {
            self.subst(&p.ty, &args_map)
        } else if let Some(m) = self.find_method(&class, name) {
            Ty::Fn(
                m.params.iter().map(|p| self.subst(&p.ty, &args_map)).collect(),
                Box::new(self.subst(&m.ret, &args_map)),
            )
        } else {
            self.err(e.span, format!("no member `{name}` on `{class}`"));
            Ty::Unknown
        };
        member_ty.opt_of()
    }

    fn check_opt_unwrap(&mut self, e: &Expr, inner: &Expr) -> Ty {
        let t = self.check_expr(inner);
        match t.inner_option() {
            Some(inner_t) => inner_t,
            None => {
                self.err_note(
                    e.span,
                    format!("`!` cannot unwrap non-option value of type `{t}`"),
                    "only `Option` values may be unwrapped",
                );
                t
            }
        }
    }

    fn check_index(&mut self, e: &Expr, object: &Expr, index: &Expr) -> Ty {
        let ot = self.check_expr(object);
        let it = self.check_expr(index);
        match &ot {
            Ty::List(_) | Ty::Range(_) => {
                if it != Ty::Unknown && it != Ty::Int {
                    self.err(e.span, "index must be an `int`");
                }
                ot.elem().unwrap_or(Ty::Unknown)
            }
            Ty::Map(_, val_ty) => {
                if it != Ty::Unknown && it != Ty::String {
                    self.err(e.span, "map index must be a `string`");
                }
                val_ty.as_ref().clone()
            }
            Ty::String => {
                if it != Ty::Unknown && it != Ty::Int {
                    self.err(e.span, "string index must be an `int`");
                }
                Ty::Char
            }
            Ty::Tuple(items) => Ty::Tuple(items.clone()),
            Ty::Unknown => Ty::Unknown,
            other => {
                self.err(
                    e.span,
                    format!("cannot index a value of type `{other}`"),
                );
                Ty::Unknown
            }
        }
    }

    fn check_binary(&mut self, e: &Expr, op: BinOp, lhs: &Expr, rhs: &Expr) -> Ty {
        let lt = self.check_expr(lhs);
        let rt = self.check_expr(rhs);
        use BinOp::*;
        match op {
            Add | Sub | Mul | Div | Mod | Pow => {
                if lt != Ty::Unknown && rt != Ty::Unknown {
                    let numeric_ok = lt.is_numeric() && rt.is_numeric();
                    let string_ok = matches!(op, Add) && lt == Ty::String && rt == Ty::String;
                    if !numeric_ok && !string_ok {
                        self.err_note(
                            e.span,
                            format!("operator `{op:?}` requires numeric or string operands, found `{lt}` and `{rt}`"),
                            "`+` concatenates strings; arithmetic operators need numbers",
                        );
                    }
                }
                if lt == Ty::Float || rt == Ty::Float {
                    Ty::Float
                } else if matches!(op, Add) && (lt == Ty::String || rt == Ty::String) {
                    Ty::String
                } else {
                    Ty::Int
                }
            }
            Shl | Shr | BitAnd | BitOr | BitXor => {
                if lt != Ty::Unknown && lt != Ty::Int {
                    self.err(e.span, "bitwise operators require `int` operands");
                }
                let _ = rt;
                Ty::Int
            }
            Eq | Ne | Lt | Le | Gt | Ge => Ty::Bool,
            And | Or => {
                self.check_bool_cond(lhs);
                self.check_bool_cond(rhs);
                Ty::Bool
            }
            Range | RangeIncl => Ty::Range(Box::new(lt)),
            NullCoalesce => {
                let inner = lt.inner_option();
                match inner {
                    Some(inner_t) => {
                        if rt != Ty::Unknown {
                            self.check_assignable(&inner_t, &rt, rhs.span, "`??` default");
                        }
                        lt
                    }
                    None => {
                        self.err_note(
                            e.span,
                            "left side of `??` is not an `Option`",
                            "`a ?? b` yields `a` when present, else `b`",
                        );
                        lt
                    }
                }
            }
            Send => Ty::Unknown,
            Is => {
                let _ = lt;
                let _ = rt;
                Ty::Bool
            }
            In => {
                let ot = self.check_expr(lhs);
                let _ = self.check_expr(rhs);
                if !matches!(ot, Ty::List(_) | Ty::Map(_, _) | Ty::String) {
                    let _ = ot;
                }
                Ty::Bool
            }
        }
    }

    fn check_unary(&mut self, e: &Expr, op: UnOp, operand: &Expr) -> Ty {
        let t = self.check_expr(operand);
        match op {
            UnOp::Neg => {
                if t != Ty::Unknown && !t.is_numeric() {
                    self.err(e.span, format!("unary `-` requires a numeric operand, found `{t}`"));
                }
                t
            }
            UnOp::Not => {
                if t != Ty::Unknown && t != Ty::Bool {
                    self.err(e.span, format!("unary `!` requires a `bool` operand, found `{t}`"));
                }
                Ty::Bool
            }
            UnOp::BitNot => {
                if t != Ty::Unknown && t != Ty::Int {
                    self.err(e.span, format!("unary `~` requires an `int` operand, found `{t}`"));
                }
                Ty::Int
            }
            UnOp::AddrOf | UnOp::Deref => Ty::Unknown,
        }
    }

    fn check_assign(&mut self, e: &Expr, target: &Expr, op: AssignOp, value: &Expr) -> Ty {
        let _ = op;
        let vt = self.check_expr(value);
        match &target.kind {
            ExprKind::Ident(name) => {
                match self.lookup(name).cloned() {
                    Some(l) => {
                        if !l.mutable {
                            self.err(
                                target.span,
                                format!("cannot assign to immutable binding `{name}`"),
                            );
                        }
                        self.check_assignable(&l.ty, &vt, target.span, "assignment");
                    }
                    None => {
                        // Implicit receiver field assignment: `count = x`
                        // inside a class body writes `this.count`.
                        if let Some(st) = &self.self_ty {
                            if let Some((cn, args_map)) = self.type_key(st) {
                                if let Some(f) = self.find_field(&cn, name, &args_map) {
                                    if !f.0.mutable {
                                        self.err(
                                            target.span,
                                            format!("cannot assign to immutable field `{name}`"),
                                        );
                                    }
                                    self.check_assignable(&f.0.ty, &vt, target.span, "assignment");
                                    return Ty::Empty;
                                }
                            }
                        }
                        self.err(
                            target.span,
                            format!("cannot assign to undeclared name `{name}`"),
                        );
                    }
                }
            }
            ExprKind::Member { object, name } => {
                let ot = self.check_expr(object);
                if let Some((class, args_map)) = self.type_key(&ot) {
                    if let Some(f) = self.find_field(&class, name, &args_map) {
                        if !f.0.mutable {
                            self.err(
                                target.span,
                                format!("cannot assign to immutable field `{name}`"),
                            );
                        }
                        self.check_assignable(&f.0.ty, &vt, target.span, "assignment");
                    } else if let Some(p) = self.find_property(&class, name) {
                        if !p.has_set {
                            self.err(
                                target.span,
                                format!("property `{name}` has no setter"),
                            );
                        }
                    } else {
                        self.err(
                            target.span,
                            format!("no assignable member `{name}` on `{class}`"),
                        );
                    }
                } else {
                    self.err(target.span, "cannot assign to a member of a non-class value");
                }
            }
            ExprKind::Index { object, index } => {
                let ot = self.check_expr(object);
                let it = self.check_expr(index);
                match &ot {
                    Ty::List(_) | Ty::Range(_) if it != Ty::Unknown && it != Ty::Int => {
                        self.err(target.span, "index must be an `int`");
                    }
                    Ty::Map(_, _) if it != Ty::Unknown && it != Ty::String => {
                        self.err(target.span, "map index must be a `string`");
                    }
                    _ => {}
                }
                let elem_ty = match &ot {
                    Ty::List(inner) => inner.as_ref().clone(),
                    Ty::Map(_, v) => v.as_ref().clone(),
                    Ty::String => Ty::Char,
                    _ => {
                        self.err(
                            target.span,
                            "index assignment target must be a List, Map, or string",
                        );
                        Ty::Unknown
                    }
                };
                self.types.insert(target.span, elem_ty.clone());
                self.check_assignable(&elem_ty, &vt, target.span, "assignment");
            }
            _ => {
                self.err(target.span, "assignment target must be a variable, member, or index");
            }
        }
        let _ = e.span;
        vt
    }

    fn check_lambda(
        &mut self,
        e: &Expr,
        params: &[Param],
        return_ty: Option<&TypeExpr>,
        body: &FnBody,
    ) -> Ty {
        self.push_scope();
        let mut param_tys = Vec::new();
        for p in params {
            let pt = self.resolve_param_ty(p);
            param_tys.push(pt.clone());
            self.declare(&p.name, pt, true);
        }
        let saved_ret = self.ret_ty.clone();
        if let Some(ty) = return_ty {
            self.ret_ty = self.resolved_fn_ty(ty, &self.fn_generics);
        } else {
            self.ret_ty = Ty::Unknown;
        }
        self.check_fn_body(body);
        let ret = if let Some(ty) = return_ty {
            self.resolved_fn_ty(ty, &self.fn_generics)
        } else {
            Ty::Unknown
        };
        self.ret_ty = saved_ret;
        self.pop_scope();
        let _ = e.span;
        Ty::Fn(param_tys, Box::new(ret))
    }

    fn check_if(
        &mut self,
        e: &Expr,
        cond: &IfCond,
        then: &Block,
        else_else: Option<&Expr>,
    ) -> Ty {
        match cond {
            IfCond::Cond(c) => self.check_bool_cond(c),
            IfCond::Binding { pattern, value } => {
                let vt = self.check_expr(value);
                self.push_scope();
                self.bind_pattern(pattern, &vt, false);
                self.pop_scope();
            }
        }
        let tt = self.check_block(then);
        let _ = e.span;
        if let Some(else_e) = else_else {
            let et = self.check_expr(else_e);
            if tt != Ty::Empty && et != Ty::Empty && tt != et {
                self.err_note(
                    else_e.span,
                    format!("`if` branches have mismatched types: `{tt}` and `{et}`"),
                    "both branches of an if-expression must produce the same type",
                );
            }
            tt
        } else {
            Ty::Empty
        }
    }

    fn check_match(&mut self, e: &Expr, scrutinee: &Expr, arms: &[MatchArm]) -> Ty {
        let st = self.check_expr(scrutinee);
        let mut result = Ty::Empty;
        for arm in arms {
            self.push_scope();
            self.bind_pattern(&arm.pattern, &st, false);
            if let Some(g) = &arm.guard {
                self.check_bool_cond(g);
            }
            let bt = self.check_expr(&arm.body);
            if result == Ty::Empty {
                result = bt;
            } else if bt != Ty::Empty && bt != result {
                self.err_note(
                    arm.span,
                    format!("match arms produce inconsistent types: `{result}` and `{bt}`"),
                    "all arms of a match expression must produce the same type",
                );
            }
            self.pop_scope();
        }
        let _ = e.span;
        result
    }

    fn check_cast(&mut self, e: &Expr, expr: &Expr, ty: &TypeExpr, kind: CastKind) -> Ty {
        let src = self.check_expr(expr);
        let target = self.resolved_fn_ty(ty, self.instantiated_generics(&self.fn_generics));
        match kind {
            CastKind::Is => {
                // `is` must check against a type the source can actually be.
                // Unknown types are not yet resolved; allow them through.
                if src != Ty::Unknown && target != Ty::Unknown && !self.cast_related(&src, &target)
                {
                    self.err_note(
                        e.span,
                        format!("`{src} is {target}` can never succeed"),
                        "cast target must be assignable from, or an ancestor/descendant of, the source",
                    );
                }
                Ty::Bool
            }
            CastKind::As | CastKind::TryAs => {
                if src != Ty::Unknown && target != Ty::Unknown && !self.cast_related(&src, &target)
                {
                    self.err_note(
                        e.span,
                        format!("cannot cast `{src}` to `{target}`"),
                        "cast target must be assignable from, or an ancestor/descendant of, the source",
                    );
                }
                target
            }
        }
    }

    /// Whether `from` can be safely cast to `to`: numeric conversions,
    /// option<->inner, class inheritance (either direction), and
    /// conforming interfaces.
    fn cast_related(&self, from: &Ty, to: &Ty) -> bool {
        if from == to {
            return true;
        }
        if from.is_numeric() && to.is_numeric() {
            return true;
        }
        // Unwrap option layers on either side.
        if let Some(inner) = from.inner_option() {
            if self.cast_related(&inner, to) {
                return true;
            }
        }
        if let Some(inner) = to.inner_option() {
            if self.cast_related(from, &inner) {
                return true;
            }
        }
        // Same-kind structural/type relations.
        match (from, to) {
            (Ty::Class(a, _), Ty::Class(b, _)) | (Ty::Struct(a, _), Ty::Struct(b, _)) => {
                a == b
                    || self.is_ancestor(a, b)
                    || self.is_ancestor(b, a)
                    || self.conforms_to(from, to)
                    || self.conforms_to(to, from)
            }
            (Ty::Interface(a, _), Ty::Interface(b, _)) => {
                a == b || self.conforms_to(from, to) || self.conforms_to(to, from)
            }
            (Ty::Enum(a, _), Ty::Enum(b, _)) => a == b,
            (Ty::List(a), Ty::List(b)) | (Ty::Range(a), Ty::Range(b)) => self.cast_related(a, b),
            (Ty::Map(ka, va), Ty::Map(kb, vb)) => {
                self.cast_related(ka, kb) && self.cast_related(va, vb)
            }
            (Ty::Fn(pa, ra), Ty::Fn(pb, rb)) => {
                pa.len() == pb.len() && {
                    let mut ok = true;
                    for (x, y) in pa.iter().zip(pb.iter()) {
                        if !self.cast_related(x, y) {
                            ok = false;
                        }
                    }
                    ok && self.cast_related(ra, rb)
                }
            }
            _ => false,
        }
    }
}

impl ClassMember {
    fn name(&self) -> String {
        match self {
            ClassMember::Field { name, .. }
            | ClassMember::Const { name, .. }
            | ClassMember::Property(PropertyDecl { name, .. })
            | ClassMember::Method(MethodDecl { name, .. }) => name.clone(),
            ClassMember::Constructor(c) => format!("constructor {}", c.span),
            ClassMember::Init(_) => "init".into(),
            ClassMember::Deinit(_) => "deinit".into(),
        }
    }
}

impl Pattern {
    /// Patterns do not carry their own span in the AST yet; error reporting
    /// for pattern mismatches uses the enclosing span passed by the caller.
    fn span(&self) -> Span {
        Span::new(crate::diag::FileId(0), 0, 0)
    }
}