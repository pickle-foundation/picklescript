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
use crate::resolve::{CallableInfo, ClassTable, EnumTable, FieldInfo, ParamInfo, PropertyInfo, ResolvedProgram, TypeTableEntry};
use crate::ty::Ty;

/// Runtime class ids for user classes start at this id: the runtime reserves
/// 0..=6 for the builtin boxed types (string/list/map/int/float/bool/char)
/// and 7 for `PEnum`, so the first user class registered at runtime gets id
/// 8. The compiler assigns `8 + index` in declaration order, matching the
/// runtime's allocation order.
const PICKLE_CLASS_USER_BASE: i64 = 8;

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
        class_consts: HashMap::new(),
        const_inlining: Vec::new(),
        fid_list: Vec::new(),
        fsource: HashMap::new(),
        finfo: HashMap::new(),
        classes: Vec::new(),
        class_by_name: HashMap::new(),
        class_decls: HashMap::new(),
        class_attempted: Vec::new(),
        ctor_ids: HashMap::new(),
        named_ctor_ids: HashMap::new(),
        deinit_ids: HashMap::new(),
        method_ids: HashMap::new(),
        property_ids: HashMap::new(),
        static_property_ids: HashMap::new(),
        static_inits: Vec::new(),
        static_init_id: None,
        owner: None,
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

/// Where a registered function's body and signature come from.
#[derive(Clone)]
enum FnSource<'a> {
    /// Top-level `fn` (or `test`) declaration.
    TopLevel(&'a FnDecl),
    /// The constructor of a class/struct: synthesized from the fields unless an
    /// explicit `constructor(...)` body is present.
    Ctor {
        table: ClassTable,
        /// (slot, initializer) for each instance field with an initializer,
        /// in declaration order.
        inits: Vec<(usize, &'a Expr)>,
        /// `init { ... }` blocks, in declaration order.
        init_blocks: Vec<&'a Block>,
        /// The explicit `constructor(...) { ... }` declaration, if any.
        ctor: Option<&'a ConstructorDecl>,
    },
    /// A named constructor: a static factory that delegates to the primary
    /// constructor via `this(...)`.
    NamedCtor {
        table: ClassTable,
        ctor: &'a ConstructorDecl,
    },
    /// A `deinit` finalizer: runs at GC sweep on a dead instance, off the
    /// allocation path. Slot 0 is `this`; returns unit.
    Deinit {
        table: ClassTable,
        body: &'a Block,
    },
    /// A class/struct method (instance or static).
    Method { table: ClassTable, md: &'a MethodDecl },
    /// A property accessor: slot 0 is `this` (instance only; static
    /// properties are receiver-less), and setters take a `value` parameter,
    /// over the resolved property signature.
    Property {
        table: ClassTable,
        pd: &'a PropertyDecl,
        info: PropertyInfo,
        is_set: bool,
        is_static: bool,
    },
    /// The synthesized static-field initializer: one assignment per static
    /// field, evaluated once at program start. `None` means the field had no
    /// initializer and is stored as its default (zero/null) value.
    StaticInit {
        inits: Vec<(u32, usize, Option<&'a Expr>)>,
    },
}

/// A registered, lowerable user class/struct.
struct ClassPlan {
    name: String,
    class_id: u32,
    /// Superclass id for a class that `extends` another, else `None`.
    parent: Option<u32>,
    table: ClassTable,
}

/// The single `this(...)` delegation expression of a named-constructor body, or
/// `None` if the body is not exactly one such expression (as a statement or a
/// block tail).
fn named_ctor_delegation(body: &Block) -> Option<&Expr> {
    fn this_call(e: &Expr) -> Option<&Expr> {
        match &e.kind {
            ExprKind::Call { callee, .. } if matches!(&callee.kind, ExprKind::This) => Some(e),
            _ => None,
        }
    }
    match (body.stmts.as_slice(), body.expr.as_deref()) {
        ([], Some(e)) => this_call(e),
        ([Stmt::Expr(e)], None) => this_call(e),
        _ => None,
    }
}

/// Static slot number of static field `name` (its position among static
/// fields).
fn static_field_slot(table: &ClassTable, name: &str) -> Option<usize> {
    table.fields.iter().filter(|f| f.is_static).position(|f| f.name == name)
}

/// The declared info of static field `name`, when present.
fn static_field_info(table: &ClassTable, name: &str) -> Option<FieldInfo> {
    table.fields.iter().find(|f| f.is_static && f.name == name).cloned()
}

/// Runtime representation of a `List<T>` element.
#[derive(Clone, Copy)]
enum ElemRep {
    /// Pass the managed value through as a pointer (strings, classes, lists…).
    Ptr,
    /// Boxed/unboxed at the runtime boundary; `(box_sym, unbox_sym, ir_ty)`.
    Scalar(&'static str, &'static str, IrTy),
}

/// The IR type a `List<T>` element value carries outside the runtime.
fn elem_ir(elem: &Ty) -> IrTy {
    match elem {
        Ty::Int => IrTy::Int,
        Ty::Float => IrTy::Float,
        Ty::Bool => IrTy::Bool,
        Ty::Char => IrTy::Char,
        Ty::String => IrTy::Str,
        _ => IrTy::Ptr,
    }
}

struct Emitter<'a> {
    prog: &'a Program,
    resolved: &'a ResolvedProgram,
    diags: &'a DiagnosticSink,
    types: HashMap<Span, Ty>,
    module: IrModule,
    /// Top-level `const` name -> its initializer (inlined at use sites).
    consts_inits: HashMap<String, &'a Expr>,
    /// `(class id, const name)` -> declared info + initializer, inlined at use
    /// sites (`Type.NAME`, or the bare name inside the class body).
    class_consts: HashMap<(u32, String), (FieldInfo, &'a Expr)>,
    const_inlining: Vec<String>,
    /// Registered functions in emission order (mirrors `module.funcs`).
    fid_list: Vec<FuncId>,
    fsource: HashMap<FuncId, FnSource<'a>>,
    finfo: HashMap<FuncId, CallableInfo>,
    /// Registered user classes in id order.
    classes: Vec<ClassPlan>,
    /// Class/struct name -> assigned runtime class id (registered only).
    class_by_name: HashMap<String, u32>,
    /// Class name -> its declaration (for ancestor field initializers /
    /// `init` blocks and ancestor-first registration).
    class_decls: HashMap<String, &'a ClassDecl>,
    /// Names whose registration has been attempted, so ancestors are pulled in
    /// before descendants exactly once (and cycles terminate).
    class_attempted: Vec<String>,
    /// Class id -> implicit-constructor function.
    ctor_ids: HashMap<u32, FuncId>,
    /// (Class id, named-constructor name) -> redirect function.
    named_ctor_ids: HashMap<(u32, String), FuncId>,
    /// Class id -> `deinit` finalizer function.
    deinit_ids: HashMap<u32, FuncId>,
    /// (Class id, method name) -> (function, is_static).
    method_ids: HashMap<(u32, String), (FuncId, bool)>,
    /// (Class id, property name, is_setter) -> accessor function.
    property_ids: HashMap<(u32, String, bool), FuncId>,
    /// (Class id, static property name, is_setter) -> accessor function.
    static_property_ids: HashMap<(u32, String, bool), FuncId>,
    /// Static-field initializers in declaration order: `(class id, static slot,
    /// value expr)`. `None` means "no initializer" (store the default value).
    /// Emitted into the synthesized `pkl_static_init` function.
    static_inits: Vec<(u32, usize, Option<&'a Expr>)>,
    /// The synthesized static-initializer function, called once at the top of
    /// `main` after class registration.
    static_init_id: Option<FuncId>,
    /// Class id of the method/ctor currently being built (implicit receiver).
    owner: Option<i64>,

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
            if let ItemKind::Class(c) = &item.kind {
                self.class_decls.insert(c.name.clone(), c);
            }
        }
        let prog = self.prog;
        for item in &prog.items {
            match &item.kind {
                ItemKind::Fn(f) => self.register_fn(f, false),
                ItemKind::Test(f) => self.register_fn(f, true),
                ItemKind::Class(c) => self.register_class_by_name(&c.name),
                ItemKind::Struct(s) => self.register_struct_item(s),
                _ => {}
            }
        }
        // Static fields are initialized once at program start. Register the
        // initializer only after every class id is assigned, and before the
        // build loop so `main`'s preamble can call it.
        if !self.static_inits.is_empty() {
            let inits = std::mem::take(&mut self.static_inits);
            let fid = self.push_class_func(
                "static.init",
                "pkl_static_init",
                FnSource::StaticInit { inits },
            );
            self.static_init_id = Some(fid);
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
        self.fsource.insert(fid, FnSource::TopLevel(f));
        self.finfo.insert(fid, info.clone());
    }

    // ---- class/struct registration ---------------------------------------

    /// Register a class and, first, its ancestors, so a superclass always has a
    /// lower class id than its subclasses (field layout and the runtime parent
    /// links rely on it).
    fn register_class_by_name(&mut self, name: &str) {
        if self.class_attempted.iter().any(|n| n == name) {
            return;
        }
        self.class_attempted.push(name.to_string());
        let Some(decl) = self.class_decls.get(name).copied() else {
            return;
        };
        let table = match self.resolved.types.get(name) {
            Some(TypeTableEntry::Class(t)) => t.clone(),
            _ => return,
        };
        if let Some(p) = table
            .extends
            .as_ref()
            .and_then(|t| t.named().map(str::to_string))
        {
            if self.class_decls.contains_key(&p) {
                self.register_class_by_name(&p);
            }
        }
        self.maybe_register_class(name, &decl.members, table, decl.span);
    }

    fn register_struct_item(&mut self, s: &'a StructDecl) {
        let table = match self.resolved.types.get(&s.name) {
            Some(TypeTableEntry::Struct(t)) => t.clone(),
            _ => return,
        };
        self.maybe_register_class(&s.name, &s.members, table, s.span);
    }

    // ---- inheritance layout helpers ---------------------------------------

    /// The class/struct table for `name`, from the resolver.
    fn table_of(&self, name: &str) -> Option<ClassTable> {
        match self.resolved.types.get(name) {
            Some(TypeTableEntry::Class(t)) | Some(TypeTableEntry::Struct(t)) => Some(t.clone()),
            _ => None,
        }
    }

    /// The superclass name of `name`, when it extends another class.
    fn parent_name(&self, name: &str) -> Option<String> {
        self.table_of(name)?
            .extends
            .as_ref()
            .and_then(|t| t.named().map(str::to_string))
    }

    /// Ancestry from the rootmost superclass down to `name` inclusive.
    fn ancestry(&self, name: &str) -> Vec<String> {
        let mut chain = vec![name.to_string()];
        let mut cur = name.to_string();
        for _ in 0..64 {
            match self.parent_name(&cur) {
                Some(p) => {
                    chain.push(p.clone());
                    cur = p;
                }
                None => break,
            }
        }
        chain.reverse();
        chain
    }

    fn own_instance_fields(&self, name: &str) -> Vec<FieldInfo> {
        self.table_of(name)
            .map(|t| t.fields.into_iter().filter(|f| !f.is_static).collect())
            .unwrap_or_default()
    }

    /// Instance fields of `name` and every ancestor, root first (the runtime
    /// slot order: superclass fields occupy the low slots).
    fn all_instance_fields(&self, name: &str) -> Vec<FieldInfo> {
        let mut out = Vec::new();
        for cname in self.ancestry(name) {
            out.extend(self.own_instance_fields(&cname));
        }
        out
    }

    /// Total instance-field slot count of `name` including inherited fields.
    fn total_instance_slot_count(&self, name: &str) -> usize {
        self.ancestry(name)
            .iter()
            .map(|c| self.own_instance_fields(c).len())
            .sum()
    }

    /// Absolute (parent-first) slot of instance field `field` reachable from
    /// `name`, whether declared on `name` or inherited.
    fn abs_instance_field_slot(&self, name: &str, field: &str) -> Option<usize> {
        let mut base = 0usize;
        for cname in self.ancestry(name) {
            let own = self.own_instance_fields(&cname);
            if let Some(i) = own.iter().position(|f| f.name == field) {
                return Some(base + i);
            }
            base += own.len();
        }
        None
    }

    /// Instance-field initializers for `name` and its ancestors, root first,
    /// recorded at absolute slot numbers.
    fn collect_instance_inits(&self, name: &str) -> Vec<(usize, &'a Expr)> {
        let mut out = Vec::new();
        let mut base = 0usize;
        for cname in self.ancestry(name) {
            let own = self.own_instance_fields(&cname);
            if let Some(decl) = self.class_decls.get(&cname).copied() {
                for m in &decl.members {
                    if let ClassMember::Field {
                        name: fname,
                        init: Some(x),
                        is_static: false,
                        ..
                    } = m
                    {
                        if let Some(i) = own.iter().position(|f| &f.name == fname) {
                            out.push((base + i, x));
                        }
                    }
                }
            }
            base += own.len();
        }
        out
    }

    /// `init` blocks for `name` and its ancestors, root first.
    fn collect_init_blocks(&self, name: &str) -> Vec<&'a Block> {
        let mut out = Vec::new();
        for cname in self.ancestry(name) {
            if let Some(decl) = self.class_decls.get(&cname).copied() {
                for m in &decl.members {
                    if let ClassMember::Init(b) = m {
                        out.push(b);
                    }
                }
            }
        }
        out
    }

    /// True when `anc` is `desc` itself or one of its ancestors.
    fn is_ancestor(&self, anc: &str, desc: &str) -> bool {
        self.ancestry(desc).iter().any(|c| c == anc)
    }

    /// A non-generic user class type resolved to its registered runtime id.
    fn user_class_id(&self, ty: &Ty) -> Option<(String, u32)> {
        if let Ty::Class(n, args) = ty {
            if args.is_empty() {
                if let Some(&cid) = self.class_by_name.get(n) {
                    return Some((n.clone(), cid));
                }
            }
        }
        None
    }

    /// Register one class/struct: reserve its class id, its implicit-constructor
    /// function, and its method functions. Members outside the slice are
    /// rejected loudly rather than miscompiled. Single inheritance is lowered
    /// (parent-first field layout, inherited methods, hierarchy casts); generic
    /// and interface-implementing classes are still skipped entirely.
    fn maybe_register_class(
        &mut self,
        name: &str,
        members: &'a [ClassMember],
        table: ClassTable,
        span: Span,
    ) {
        if !table.generics.is_empty() || !table.implements.is_empty() {
            return;
        }
        if members
            .iter()
            .any(|m| matches!(m, ClassMember::Method(md) if md.is_override))
        {
            let _: Result<(), ()> = self.bad(
                span,
                format!("`{name}` uses `override`, which is not lowered yet"),
            );
            return;
        }
        let parent = table
            .extends
            .as_ref()
            .and_then(|t| t.named().map(str::to_string));
        let parent_cid = match &parent {
            Some(p) => match self.class_by_name.get(p) {
                Some(&pcid) => Some(pcid),
                // The superclass itself was not lowerable (generic/interface).
                None => return,
            },
            None => None,
        };
        // Explicit constructors plus inheritance need `super(...)` chaining,
        // which is not lowered yet: require the whole hierarchy to use the
        // synthesized constructor.
        if let Some(p) = &parent {
            let ancestor_has_ctor = self
                .table_of(p)
                .map(|pt| pt.ctor.is_some() || !pt.named_ctors.is_empty())
                .unwrap_or(false);
            if ancestor_has_ctor {
                let _: Result<(), ()> = self.bad(
                    span,
                    format!(
                        "`{name}` cannot extend `{p}` while it declares explicit constructors (not lowered yet)"
                    ),
                );
                return;
            }
        }
        let ctor_decl = members.iter().find_map(|m| match m {
            ClassMember::Constructor(cd) if cd.name.is_none() => Some(cd),
            _ => None,
        });
        let has_named_ctor = members
            .iter()
            .any(|m| matches!(m, ClassMember::Constructor(cd) if cd.name.is_some()));
        if parent.is_some() && (ctor_decl.is_some() || has_named_ctor) {
            let _: Result<(), ()> = self.bad(
                span,
                format!(
                    "`{name}` cannot declare an explicit constructor in a hierarchy (not lowered yet)"
                ),
            );
            return;
        }
        // Instance-field initializers across the whole hierarchy, root first,
        // at absolute (parent-first) slot numbers.
        let inits: Vec<(usize, &'a Expr)> = self.collect_instance_inits(name);
        let init_blocks: Vec<&'a Block> = self.collect_init_blocks(name);
        let deinit_block: Option<&'a Block> = members.iter().find_map(|m| match m {
            ClassMember::Deinit(b) => Some(b),
            _ => None,
        });

        // Every static field gets an initializer entry: the declared value, or
        // `None` for the default (zero/null) so a read never dereferences the
        // unset cell.
        let mut static_inits: Vec<(usize, Option<&'a Expr>)> = Vec::new();
        for f in table.fields.iter().filter(|f| f.is_static) {
            let Some(slot) = static_field_slot(&table, &f.name) else {
                continue;
            };
            let init = members
                .iter()
                .find_map(|m| match m {
                    ClassMember::Field {
                        name,
                        init,
                        is_static: true,
                        ..
                    } if name == &f.name => Some(init.as_ref()),
                    _ => None,
                })
                .flatten();
            static_inits.push((slot, init));
        }

        let cid = (PICKLE_CLASS_USER_BASE + self.classes.len() as i64) as u32;
        for (slot, x) in static_inits {
            self.static_inits.push((cid, slot, x));
        }
        // Class constants are compile-time values: record the declared info and
        // the initializer so reads can inline it (`Type.NAME` / bare name).
        for f in &table.consts {
            let Some(value) = members.iter().find_map(|m| match m {
                ClassMember::Const { name, value, .. } if name == &f.name => Some(value),
                _ => None,
            }) else {
                continue;
            };
            self.class_consts
                .insert((cid, f.name.clone()), (f.clone(), value));
        }

        // Constructor: `pkl_<Name>_new(...) -> Ptr`. Its parameters are the
        // explicit constructor's params when one is declared, otherwise the
        // fields without initializers (defaults are filled while running the
        // field initializers).

        let ctor_fid = self.push_class_func(
            &format!("{name}.new"),
            &format!("pkl_{name}_new"),
            FnSource::Ctor {
                table: table.clone(),
                inits,
                init_blocks,
                ctor: ctor_decl,
            },
        );
        self.ctor_ids.insert(cid, ctor_fid);

        // Named constructors: `constructor.NAME(...) { this(...) }`. Each is a
        // static factory `pkl_<Name>_nc_<NAME>` that evaluates the delegation
        // arguments and calls the primary constructor.
        for cd in members.iter().filter_map(|m| match m {
            ClassMember::Constructor(cd) if cd.name.is_some() => Some(cd),
            _ => None,
        }) {
            let cname = cd.name.clone().unwrap_or_default();
            let fid = self.push_class_func(
                &format!("{name}.{cname}"),
                &format!("pkl_{name}_nc_{cname}"),
                FnSource::NamedCtor {
                    table: table.clone(),
                    ctor: cd,
                },
            );
            self.named_ctor_ids.insert((cid, cname), fid);
        }

        // `deinit`: a finalizer `pkl_<Name>_deinit(this)` registered on the
        // class descriptor and run by the collector at sweep.
        if let Some(body) = deinit_block {
            let fid = self.push_class_func(
                &format!("{name}.deinit"),
                &format!("pkl_{name}_deinit"),
                FnSource::Deinit {
                    table: table.clone(),
                    body,
                },
            );
            self.deinit_ids.insert(cid, fid);
        }

        // Methods: `pkl_<Name>_<m>` (instance) and `pkl_<Name>_sm_<m>` (static).
        for md in members.iter().filter_map(|m| match m {
            ClassMember::Method(md) => Some(md),
            _ => None,
        }) {
            if md.is_async || md.is_override || md.body.is_none() || !md.generics.is_empty() {
                continue;
            }
            let Some(info) = table.methods.iter().find(|m| m.name == md.name) else {
                continue;
            };
            if info.params.iter().any(|p| p.has_default || p.rest) {
                continue;
            }
            let symbol = if info.is_static {
                format!("pkl_{name}_sm_{}", md.name)
            } else {
                format!("pkl_{name}_{}", md.name)
            };
            let mid = self.push_class_func(
                &format!("{name}.{}", md.name),
                &symbol,
                FnSource::Method {
                    table: table.clone(),
                    md,
                },
            );
            self.finfo.insert(mid, info.clone());
            self.method_ids.insert((cid, md.name.clone()), (mid, info.is_static));
        }

        // Property accessors: `pkl_<Name>_<p>_get`/`_set` (instance, slot 0 is
        // `this`) and `pkl_<Name>_sm_<p>_get`/`_set` (static, receiver-less).
        for pd in members.iter().filter_map(|m| match m {
            ClassMember::Property(pd) => Some(pd),
            _ => None,
        }) {
            let Some(info) = table.properties.iter().find(|i| i.name == pd.name) else {
                continue;
            };
            let stem = if pd.is_static {
                format!("pkl_{name}_sm_{}_", pd.name)
            } else {
                format!("pkl_{name}_{}_", pd.name)
            };
            if info.has_get {
                let fid = self.push_class_func(
                    &format!("{}.{}.get", name, pd.name),
                    &format!("{stem}get"),
                    FnSource::Property {
                        table: table.clone(),
                        pd,
                        info: info.clone(),
                        is_set: false,
                        is_static: pd.is_static,
                    },
                );
                if pd.is_static {
                    self.static_property_ids
                        .insert((cid, pd.name.clone(), false), fid);
                } else {
                    self.property_ids.insert((cid, pd.name.clone(), false), fid);
                }
            }
            if info.has_set {
                let fid = self.push_class_func(
                    &format!("{}.{}.set", name, pd.name),
                    &format!("{stem}set"),
                    FnSource::Property {
                        table: table.clone(),
                        pd,
                        info: info.clone(),
                        is_set: true,
                        is_static: pd.is_static,
                    },
                );
                if pd.is_static {
                    self.static_property_ids
                        .insert((cid, pd.name.clone(), true), fid);
                } else {
                    self.property_ids.insert((cid, pd.name.clone(), true), fid);
                }
            }
        }

        // Inherit the superclass's method and property slots; a member the
        // subclass declares itself wins (`or_insert`).
        if let Some(pcid) = parent_cid {
            for ((id, mname), v) in self.method_ids.clone() {
                if id == pcid {
                    self.method_ids.entry((cid, mname)).or_insert(v);
                }
            }
            for ((id, pname, is_set), fid) in self.property_ids.clone() {
                if id == pcid {
                    self.property_ids.entry((cid, pname, is_set)).or_insert(fid);
                }
            }
            for ((id, pname, is_set), fid) in self.static_property_ids.clone() {
                if id == pcid {
                    self.static_property_ids.entry((cid, pname, is_set)).or_insert(fid);
                }
            }
        }

        self.classes.push(ClassPlan {
            name: name.to_string(),
            class_id: cid,
            parent: parent_cid,
            table,
        });
        self.class_by_name.insert(name.to_string(), cid);
    }

    fn push_class_func(&mut self, name: &str, symbol: &str, src: FnSource<'a>) -> FuncId {
        let fid = FuncId(self.module.funcs.len());
        self.module.funcs.push(IrFunc {
            name: name.to_string(),
            symbol: symbol.to_string(),
            params: Vec::new(),
            ret: IrTy::Unit,
            slots: Vec::new(),
            entry: BlockId(0),
            blocks: Vec::new(),
            is_main: false,
            is_test: false,
        });
        self.fid_list.push(fid);
        self.fsource.insert(fid, src);
        fid
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
        self.owner = None;

        let Some(src) = self.fsource.get(&fid).cloned() else {
            return;
        };
        let is_main = self.module.funcs[fid.0].is_main;
        match src {
            FnSource::TopLevel(f) => {
                let Some(info) = self.finfo.get(&fid).cloned() else {
                    return;
                };
                self.declare_params(&info.params);
                self.fret = self.map_ty(&info.ret, f.span).unwrap_or(IrTy::Unit);
                self.emit_body(&f.body);
            }
            FnSource::Ctor {
                table,
                inits,
                init_blocks,
                ctor,
            } => {
                self.owner = self.class_by_name.get(&table.name).copied().map(|x| x as i64);
                let _ = self.build_ctor_body(&table, &inits, &init_blocks, ctor);
            }
            FnSource::NamedCtor { table, ctor } => {
                self.owner = self.class_by_name.get(&table.name).copied().map(|x| x as i64);
                let _ = self.build_named_ctor(&table, ctor);
            }
            FnSource::Deinit { table, body } => {
                self.owner = self.class_by_name.get(&table.name).copied().map(|x| x as i64);
                let _ = self.build_deinit_body(&table, body);
            }
            FnSource::Method { table, md } => {
                let Some(info) = self.finfo.get(&fid).cloned() else {
                    return;
                };
                self.owner = self.class_by_name.get(&table.name).copied().map(|x| x as i64);
                let _ = self.build_method_body(&table, md, &info);
            }
            FnSource::Property { table, pd, info, is_set, is_static } => {
                self.owner = self.class_by_name.get(&table.name).copied().map(|x| x as i64);
                let _ = self.build_property_body(&table, pd, &info, is_set, is_static);
            }
            FnSource::StaticInit { inits } => {
                self.fret = IrTy::Unit;
                // Default-initialize every cell first, so an initializer that
                // reads another static (including a later one) sees the field
                // type's zero/null rather than an unset cell. This mirrors
                // `class_new` zeroing instance slots before field inits run.
                for (cid, slot, _) in &inits {
                    if self.build_static_default(*cid, *slot).is_err() {
                        return;
                    }
                }
                for (cid, slot, x) in &inits {
                    let Some(x) = x else {
                        continue;
                    };
                    // Static-field initializers resolve bare names against the
                    // declaring class (there is no `this`).
                    self.owner = Some(*cid as i64);
                    if self.build_static_init_one(*cid, *slot, x).is_err() {
                        return;
                    }
                }
                self.term(IrTerm::Return { v: None });
            }
        }
        if self.failed {
            return;
        }

        // Class/struct registration calls run first inside `main`, before any
        // user statement, so every descriptor exists when the class functions
        // are called (the id each call site hardcodes is the runtime's too).
        if is_main {
            self.inject_class_registrations();
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

    /// Declare function parameters as slots 0..n (used by top-level fns).
    fn declare_params(&mut self, params: &[ParamInfo]) {
        for (i, p) in params.iter().enumerate() {
            let ir = self.map_ty(&p.ty, p.span).unwrap_or(IrTy::Ptr);
            let slot = Slot(i as u32);
            self.fslots.push(ir);
            self.fparams.push(IrParam {
                name: p.name.clone(),
                ty: ir,
            });
            self.declare(&p.name, slot);
        }
    }

    /// Lower a constructor. Parameters are the explicit `constructor(...)`'s
    /// params when one is declared, otherwise the instance fields without
    /// initializers (in declaration order). The body allocates the object,
    /// runs each field initializer in declaration order, runs the explicit
    /// constructor body followed by any `init` blocks, and returns the object.
    fn build_ctor_body(
        &mut self,
        table: &ClassTable,
        inits: &[(usize, &'a Expr)],
        init_blocks: &[&'a Block],
        ctor: Option<&'a ConstructorDecl>,
    ) -> Result<(), ()> {
        // All instance fields, superclass first, so the object's slots and the
        // synthesized constructor parameters cover inherited state too.
        let ifields: Vec<FieldInfo> = self.all_instance_fields(&table.name);
        // `(name, ty, span, field_slot)`: synthesized constructor parameters
        // carry the field slot they must be stored into; explicit constructor
        // parameters are plain locals assigned by the body.
        let params: Vec<(String, Ty, Span, Option<usize>)> = if let Some(c) = &table.ctor {
            c.params
                .iter()
                .map(|p| (p.name.clone(), p.ty.clone(), p.span, None))
                .collect()
        } else {
            ifields
                .iter()
                .enumerate()
                .filter(|(i, _)| !inits.iter().any(|(si, _)| si == i))
                .map(|(i, f)| (f.name.clone(), f.ty.clone(), f.span, Some(i)))
                .collect()
        };
        for (i, (name, ty, _span, _)) in params.iter().enumerate() {
            let ir = self.map_ty(ty, table.span).unwrap_or(IrTy::Ptr);
            let slot = Slot(i as u32);
            self.fslots.push(ir);
            self.fparams.push(IrParam {
                name: name.clone(),
                ty: ir,
            });
            self.declare(name, slot);
        }
        self.fret = IrTy::Ptr;

        let cid = self
            .class_by_name
            .get(&table.name)
            .copied()
            .unwrap_or(PICKLE_CLASS_USER_BASE as u32);
        let this_slot = self.new_slot(IrTy::Ptr);
        self.declare("this", this_slot);

        let cid_t = self.int_const(cid as i64);
        let n_t = self.int_const(ifields.len() as i64);
        let this = self.extern_call_t1(
            "pickle_class_new",
            vec![IrTy::Int, IrTy::Int],
            IrTy::Ptr,
            vec![cid_t, n_t],
        )?;
        self.instr(IrInstr::StoreSlot { slot: this_slot, v: this });

        let this = self.load(this_slot);

        // Field initializers, in declaration order; they may read earlier
        // fields through `this`.
        for &(slot, init) in inits {
            let ty = ifields[slot].ty.clone();
            let v = self.expr(init)?;
            let rep = self.elem_rep(&ty, table.span)?;
            let vt = self.ty_of(&init.span).unwrap_or(Ty::Unknown);
            let packed = self.pack_for_pointer_boundary(&rep, v, &vt, init.span)?;
            let boxed = self.box_for_store(&rep, packed, elem_ir(&ty))?;
            self.field_store(this, slot as i64, &ty, boxed);
        }

        // Store every synthesized parameter into its field slot (explicit
        // constructor parameters are locals the body assigns itself).
        for (i, (_, ty, span, field_slot)) in params.iter().enumerate() {
            if let Some(field_slot) = field_slot {
                let v = self.load(Slot(i as u32));
                let rep = self.elem_rep(ty, *span)?;
                let packed = self.pack_for_pointer_boundary(&rep, v, ty, *span)?;
                let boxed = self.box_for_store(&rep, packed, elem_ir(ty))?;
                self.field_store(this, *field_slot as i64, ty, boxed);
            }
        }

        // Explicit constructor body, then any `init` blocks.
        if let Some(cd) = ctor {
            self.emit_ctor_block(&cd.body)?;
        }
        for b in init_blocks {
            self.emit_ctor_block(b)?;
        }

        let done = self.load(this_slot);
        self.term(IrTerm::Return { v: Some(done) });
        let _ = self.tail_cleanup();
        Ok(())
    }

    /// Run a `Block` without turning its tail expression into the function's
    /// return value (used for constructor bodies and `init` blocks, where the
    /// constructor always returns `this`).
    fn emit_ctor_block(&mut self, b: &Block) -> Result<(), ()> {
        self.push_scope();
        for s in &b.stmts {
            if self.stmt(s).is_err() {
                self.pop_scope();
                return Err(());
            }
        }
        if let Some(e) = &b.expr {
            let _ = self.expr(e);
        }
        self.pop_scope();
        Ok(())
    }

    /// Lower a named constructor: a static factory that evaluates the
    /// delegation arguments and calls the primary constructor.
    fn build_named_ctor(
        &mut self,
        table: &ClassTable,
        ctor: &'a ConstructorDecl,
    ) -> Result<(), ()> {
        let info = table
            .named_ctors
            .iter()
            .find(|(n, _)| ctor.name.as_deref() == Some(n.as_str()))
            .map(|(_, c)| c);
        let params: Vec<ParamInfo> = info.map(|c| c.params.clone()).unwrap_or_default();
        for (i, p) in params.iter().enumerate() {
            let ir = self.map_ty(&p.ty, p.span).unwrap_or(IrTy::Ptr);
            let slot = Slot(i as u32);
            self.fslots.push(ir);
            self.fparams.push(IrParam {
                name: p.name.clone(),
                ty: ir,
            });
            self.declare(&p.name, slot);
        }
        self.fret = IrTy::Ptr;

        let Some(delegation) = named_ctor_delegation(&ctor.body) else {
            return self.bad(
                ctor.span,
                "named constructor body must be a single `this(...)` delegation to the primary constructor",
            );
        };
        let ExprKind::Call { args, .. } = &delegation.kind else {
            return self.bad(ctor.span, "named constructor delegation is malformed");
        };

        let cid = self
            .class_by_name
            .get(&table.name)
            .copied()
            .unwrap_or(PICKLE_CLASS_USER_BASE as u32);
        let Some(&primary) = self.ctor_ids.get(&cid) else {
            return self.bad(ctor.span, format!("`{}` has no constructor", table.name));
        };
        let fparams = self.module.funcs[primary.0].params.clone();
        let mut arg_temps = Vec::new();
        for (i, a) in args.iter().enumerate() {
            if a.spread {
                return self.bad(a.span, "spread arguments are not lowered yet");
            }
            let t = self.expr(&a.value)?;
            let t = if matches!(fparams.get(i).map(|p| p.ty), Some(IrTy::Ptr)) {
                let vt = self.ty_of(&a.value.span).unwrap_or(Ty::Unknown);
                self.option_wrap(t, &vt, a.value.span)?
            } else {
                t
            };
            arg_temps.push(t);
        }
        let dst = self.temp();
        self.instr(IrInstr::Call {
            dst: Some(dst),
            callee: Callee::Func(primary),
            args: arg_temps,
        });
        self.term(IrTerm::Return { v: Some(dst) });
        Ok(())
    }

    /// Lower a `deinit` finalizer: slot 0 is `this`, the body runs with `this`
    /// live, and the function returns unit. Deinit runs off the allocation path
    /// during GC sweep (see `docs/05-memory.md`).
    fn build_deinit_body(&mut self, _table: &ClassTable, body: &'a Block) -> Result<(), ()> {
        self.fslots.push(IrTy::Ptr);
        self.fparams.push(IrParam {
            name: "this".to_string(),
            ty: IrTy::Ptr,
        });
        self.declare("this", Slot(0));
        self.fret = IrTy::Unit;
        self.emit_ctor_block(body)?;
        self.tail_cleanup();
        Ok(())
    }

    /// Lower a method: slot 0 is `this` (instance methods), followed by the
    /// declared parameters.
    fn build_method_body(
        &mut self,
        _table: &ClassTable,
        md: &'a MethodDecl,
        info: &CallableInfo,
    ) -> Result<(), ()> {
        let mut d = 0usize;
        if !info.is_static {
            let this_slot = Slot(0);
            self.fslots.push(IrTy::Ptr);
            self.fparams.push(IrParam {
                name: "this".to_string(),
                ty: IrTy::Ptr,
            });
            self.declare("this", this_slot);
            d = 1;
        }
        for (i, p) in info.params.iter().enumerate() {
            let ir = self.map_ty(&p.ty, p.span).unwrap_or(IrTy::Ptr);
            let slot = Slot((d + i) as u32);
            self.fslots.push(ir);
            self.fparams.push(IrParam {
                name: p.name.clone(),
                ty: ir,
            });
            self.declare(&p.name, slot);
        }
        self.fret = self.map_ty(&info.ret, md.span).unwrap_or(IrTy::Unit);
        self.emit_body(&md.body);
        let _ = self.tail_cleanup();
        Ok(())
    }

    /// Lower a property accessor. Slot 0 is `this` (always; static properties
    /// are deferred). Getters return the property value; setters take `value`
    /// in slot 1, run the accessor body, and return unit.
    fn build_property_body(
        &mut self,
        _table: &ClassTable,
        pd: &PropertyDecl,
        info: &PropertyInfo,
        is_set: bool,
        is_static: bool,
    ) -> Result<(), ()> {
        let mut d = 0usize;
        if !is_static {
            self.fslots.push(IrTy::Ptr);
            self.fparams.push(IrParam {
                name: "this".to_string(),
                ty: IrTy::Ptr,
            });
            self.declare("this", Slot(0));
            d = 1;
        }
        if is_set {
            let ir = self.map_ty(&info.ty, pd.span).unwrap_or(IrTy::Ptr);
            self.fslots.push(ir);
            self.fparams.push(IrParam {
                name: "value".to_string(),
                ty: ir,
            });
            self.declare("value", Slot(d as u32));
            self.fret = IrTy::Unit;
        } else {
            self.fret = self.map_ty(&info.ty, pd.span).unwrap_or(IrTy::Ptr);
        }
        let accessor = if is_set { &pd.set } else { &pd.get };
        if let Some(a) = accessor {
            match a {
                PropertyAccessor::Expr(e) => {
                    if self.fret.is_unit() {
                        let _ = self.expr(e);
                        self.term(IrTerm::Return { v: None });
                    } else if let Ok(t) = self.expr(e) {
                        self.term(IrTerm::Return { v: Some(t) });
                    }
                }
                PropertyAccessor::Block(b) => {
                    let _ = self.emit_body(&Some(FnBody::Block(Box::new(b.clone()))));
                }
            }
        }
        let _ = self.tail_cleanup();
        Ok(())
    }

    /// Register every user class descriptor at the top of `main`'s entry block.
    /// Each is a
    /// `pickle_class_register(name_ptr, name_len, slot_count, mask, finalizer,
    /// parent)` void call; the returned id is discarded (call sites hardcode
    /// it). `finalizer` is the address of the class's `pkl_<Name>_deinit`
    /// function, or 0 when it has no `deinit` block; `parent` is the
    /// superclass id, or 0 for a root class.
    fn inject_class_registrations(&mut self) {
        if self.classes.is_empty() {
            return;
        }
        let plans: Vec<(String, usize, u32, u32)> = self
            .classes
            .iter()
            .map(|p| {
                (
                    p.name.clone(),
                    self.total_instance_slot_count(&p.name),
                    p.class_id,
                    p.parent.unwrap_or(0),
                )
            })
            .collect();
        let mut instrs: Vec<IrInstr> = Vec::new();
        for (name, field_count, cid, parent) in plans {
            let sid = StrId(self.intern_string(&name));
            let addr = self.temp();
            instrs.push(IrInstr::Const {
                dst: addr,
                c: IrConst::StrAddr(sid),
            });
            let len = self.temp();
            instrs.push(IrInstr::Const {
                dst: len,
                c: IrConst::Int(name.len() as i64),
            });
            let n = self.temp();
            instrs.push(IrInstr::Const {
                dst: n,
                c: IrConst::Int(field_count as i64),
            });
            let mask = self.temp();
            let m = if field_count >= 64 { u64::MAX } else { (1u64 << field_count) - 1 };
            instrs.push(IrInstr::Const {
                dst: mask,
                c: IrConst::Int(m as i64),
            });
            let fin = self.temp();
            let c = match self.deinit_ids.get(&cid) {
                Some(fid) => IrConst::FuncAddr(*fid),
                None => IrConst::Int(0),
            };
            instrs.push(IrInstr::Const { dst: fin, c });
            let par = self.temp();
            instrs.push(IrInstr::Const {
                dst: par,
                c: IrConst::Int(parent as i64),
            });
            let ex = self.module.extern_id(IrExtern {
                symbol: "pickle_class_register".to_string(),
                params: vec![
                    IrTy::Int,
                    IrTy::Int,
                    IrTy::Int,
                    IrTy::Int,
                    IrTy::Int,
                    IrTy::Int,
                ],
                ret: IrTy::Unit,
            });
            instrs.push(IrInstr::Call {
                dst: None,
                callee: Callee::Extern(ex),
                args: vec![addr, len, n, mask, fin, par],
            });
        }
        if let Some(fid) = self.static_init_id {
            instrs.push(IrInstr::Call {
                dst: None,
                callee: Callee::Func(fid),
                args: Vec::new(),
            });
        }
        let blk = &mut self.blocks[0];
        blk.instrs.splice(0..0, instrs);
    }

    fn int_const(&mut self, v: i64) -> Temp {
        let t = self.temp();
        self.instr(IrInstr::Const {
            dst: t,
            c: IrConst::Int(v),
        });
        t
    }

    /// Emit the function body; the current block is left terminator-clean.
    fn emit_body(&mut self, body: &Option<FnBody>) -> bool {
        let body = match body {
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
                    let t = if self.fret == IrTy::Ptr {
                        let vt = self.ty_of(&e.span).unwrap_or(Ty::Unknown);
                        self.option_wrap(t, &vt, e.span).unwrap_or(t)
                    } else {
                        t
                    };
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
                    let t = if self.fret == IrTy::Ptr {
                        let vt = self.ty_of(&e.span).unwrap_or(Ty::Unknown);
                        self.option_wrap(t, &vt, e.span).unwrap_or(t)
                    } else {
                        t
                    };
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
                    Pattern::Binding { ty: pat_ty, .. } => {
                        let annot: &Option<TypeExpr> = if pat_ty.is_some() { pat_ty } else { ty };
                        let ann_option = matches!(
                            annot.as_ref().map(|t| &t.kind),
                            Some(TypeExprKind::Option(_))
                        );
                        let slot_ty = if ann_option {
                            IrTy::Ptr
                        } else {
                            match init {
                                Some(e) => self.irty(e.span)?,
                                None => self.annot_ty(annot, *span)?,
                            }
                        };
                        let slot = self.new_slot(slot_ty);
                        if let Some(e) = init {
                            let t = self.expr(e)?;
                            let v = if slot_ty == IrTy::Ptr {
                                let vt = self.ty_of(&e.span).unwrap_or(Ty::Unknown);
                                self.option_wrap(t, &vt, e.span)?
                            } else {
                                t
                            };
                            self.instr(IrInstr::StoreSlot { slot, v });
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
                let v = if ty == IrTy::Ptr {
                    let vt = self.ty_of(&value.span).unwrap_or(Ty::Unknown);
                    self.option_wrap(t, &vt, value.span)?
                } else {
                    t
                };
                self.instr(IrInstr::StoreSlot { slot, v });
                self.declare(name, slot);
                Ok(())
            }
            Stmt::Return { value, .. } => {
                let v = match value {
                    Some(e) => {
                        let t = self.expr(e)?;
                        if self.fret == IrTy::Ptr {
                            let vt = self.ty_of(&e.span).unwrap_or(Ty::Unknown);
                            Some(self.option_wrap(t, &vt, e.span)?)
                        } else {
                            Some(t)
                        }
                    }
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
                let step_id = self.new_block();
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
                    // `continue` must still run the step expression, so it
                    // targets the step block, not the condition.
                    continue_target: step_id,
                    break_target: end_id,
                });
                self.push_scope();
                self.block_body_only(body)?;
                self.pop_scope();
                self.loops.pop();
                self.term(IrTerm::Branch { target: step_id });
                self.cur = step_id;
                let _ = self.expr(step);
                self.term(IrTerm::Branch { target: cond_id });
                self.cur = end_id;
                Ok(())
            }
            ForHeader::In { pattern, sequence } => self.for_in(pattern, sequence, body, span),
        }
    }

    /// `for (x in <seq>)`: slice-1 supports integer range sequences and lists.
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
        if let Some(Ty::String) = self.ty_of(&sequence.span) {
            let seq_t = self.expr(sequence)?;
            return self.for_in_string(name, seq_t, body);
        }
        if let Some(Ty::List(inner)) = self.ty_of(&sequence.span) {
            let elem = inner.as_ref().clone();
            let seq_t = self.expr(sequence)?;
            return self.for_in_values(name, seq_t, elem, body, span);
        }
        if let Some(Ty::Map(k, v)) = self.ty_of(&sequence.span) {
            if k.as_ref() != &Ty::String {
                return self.bad(sequence.span, "map keys must be `string` values");
            }
            let map_t = self.expr(sequence)?;
            let vals = self.extern_call_t1(
                "pickle_map_values",
                vec![IrTy::Ptr],
                IrTy::Ptr,
                vec![map_t],
            )?;
            return self.for_in_values(name, vals, v.as_ref().clone(), body, span);
        }
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
                "`for (x in ...)` over non-range, non-list sequences is not lowered yet",
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
        let next_id = self.new_block();
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
            // `continue` must still advance the loop variable, so it targets
            // the increment block, not the condition.
            continue_target: next_id,
            break_target: end_id,
        });
        self.push_scope();
        self.declare(name, idx);
        self.block_body_only(body)?;
        self.pop_scope();
        self.loops.pop();
        self.term(IrTerm::Branch { target: next_id });
        self.cur = next_id;
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

    /// `for (x in seq)` over a sequence value. `seq` is a boxed (managed)
    /// sequence — a `List` produced directly by the sequence expression or by
    /// `pickle_map_values` for a map. Iterates by index; scalar elements are
    /// unboxed each trip. The sequence lives in a managed slot so the collector
    /// keeps it and its boxed elements reachable for the whole loop.
    fn for_in_values(
        &mut self,
        name: &str,
        seq_t: Temp,
        elem: Ty,
        body: &Block,
        span: Span,
    ) -> Result<(), ()> {
        let rep = self.elem_rep(&elem, span)?;
        let elem_ir = elem_ir(&elem);
        let seq_slot = self.new_slot(IrTy::Ptr);
        let idx_slot = self.new_slot(IrTy::Int);
        let len_slot = self.new_slot(IrTy::Int);
        self.instr(IrInstr::StoreSlot { slot: seq_slot, v: seq_t });
        let zero = self.temp();
        self.instr(IrInstr::Const {
            dst: zero,
            c: IrConst::Int(0),
        });
        self.instr(IrInstr::StoreSlot { slot: idx_slot, v: zero });
        let seq_l = self.load(seq_slot);
        let len_t = self.extern_call_t1("pickle_list_len", vec![IrTy::Ptr], IrTy::Int, vec![seq_l])?;
        self.instr(IrInstr::StoreSlot { slot: len_slot, v: len_t });

        let cond_id = self.new_block();
        let body_id = self.new_block();
        let next_id = self.new_block();
        let end_id = self.new_block();
        self.term(IrTerm::Branch { target: cond_id });
        self.cur = cond_id;
        let cur_t = self.load(idx_slot);
        let end_l = self.load(len_slot);
        let cmp = self.temp();
        self.instr(IrInstr::BinOp {
            dst: cmp,
            op: IrBinOp::Lt,
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
            continue_target: next_id,
            break_target: end_id,
        });
        self.push_scope();
        let cur_i = self.load(idx_slot);
        let seq_l = self.load(seq_slot);
        let raw = self.extern_call_t1(
            "pickle_list_get",
            vec![IrTy::Ptr, IrTy::Int],
            IrTy::Ptr,
            vec![seq_l, cur_i],
        )?;
        let v = match rep {
            ElemRep::Scalar(_, unbox_sym, _) => {
                self.extern_call_t1(unbox_sym, vec![IrTy::Ptr], elem_ir, vec![raw])?
            }
            ElemRep::Ptr => raw,
        };
        let elem_slot = self.new_slot(elem_ir);
        self.instr(IrInstr::StoreSlot { slot: elem_slot, v });
        self.declare(name, elem_slot);
        self.block_body_only(body)?;
        self.pop_scope();
        self.loops.pop();
        self.term(IrTerm::Branch { target: next_id });
        self.cur = next_id;
        let c = self.load(idx_slot);
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
        self.instr(IrInstr::StoreSlot { slot: idx_slot, v: nxt });
        self.term(IrTerm::Branch { target: cond_id });
        self.cur = end_id;
        Ok(())
    }

    /// `for (c in s)` over a string: iterates byte indices, char-binds the raw
    /// `i32` from `pickle_str_get` (no unbox). The string lives in a managed
    /// slot for the whole loop.
    fn for_in_string(
        &mut self,
        name: &str,
        seq_t: Temp,
        body: &Block,
    ) -> Result<(), ()> {
        let seq_slot = self.new_slot(IrTy::Ptr);
        let idx_slot = self.new_slot(IrTy::Int);
        let len_slot = self.new_slot(IrTy::Int);
        self.instr(IrInstr::StoreSlot { slot: seq_slot, v: seq_t });
        let zero = self.temp();
        self.instr(IrInstr::Const {
            dst: zero,
            c: IrConst::Int(0),
        });
        self.instr(IrInstr::StoreSlot { slot: idx_slot, v: zero });
        let seq_l = self.load(seq_slot);
        let len_t = self.extern_call_t1("pickle_str_len", vec![IrTy::Ptr], IrTy::Int, vec![seq_l])?;
        self.instr(IrInstr::StoreSlot { slot: len_slot, v: len_t });

        let cond_id = self.new_block();
        let body_id = self.new_block();
        let next_id = self.new_block();
        let end_id = self.new_block();
        self.term(IrTerm::Branch { target: cond_id });
        self.cur = cond_id;
        let cur_t = self.load(idx_slot);
        let end_l = self.load(len_slot);
        let cmp = self.temp();
        self.instr(IrInstr::BinOp {
            dst: cmp,
            op: IrBinOp::Lt,
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
            continue_target: next_id,
            break_target: end_id,
        });
        self.push_scope();
        let cur_i = self.load(idx_slot);
        let seq_l = self.load(seq_slot);
        let v = self.extern_call_t1(
            "pickle_str_get",
            vec![IrTy::Ptr, IrTy::Int],
            IrTy::Char,
            vec![seq_l, cur_i],
        )?;
        let elem_slot = self.new_slot(IrTy::Char);
        self.instr(IrInstr::StoreSlot { slot: elem_slot, v });
        self.declare(name, elem_slot);
        self.block_body_only(body)?;
        self.pop_scope();
        self.loops.pop();
        self.term(IrTerm::Branch { target: next_id });
        self.cur = next_id;
        let c = self.load(idx_slot);
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
        self.instr(IrInstr::StoreSlot { slot: idx_slot, v: nxt });
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
            ExprKind::This => self.this_value(e),
            // `super` denotes the same receiver as `this`; the checker types it
            // as the superclass, so `super.m(...)` statically dispatches to the
            // superclass's method on the same object.
            ExprKind::Super => self.this_value(e),
            ExprKind::Member { object, name } => self.member_value(e, object, name),
            ExprKind::Index { object, index } => self.index_read(e, object, index),
            ExprKind::OptAccess { object, name } => self.opt_access(e, object, name),
            ExprKind::OptUnwrap(inner) => self.opt_unwrap(e, inner),
            ExprKind::Lambda { .. } => self.bad(e.span, "lambda values are not lowered yet"),
            ExprKind::Match {
                scrutinee,
                arms,
            } => self.match_expr(e, scrutinee, arms),
            ExprKind::Await(_) => self.bad(e.span, "`await` is not lowered yet"),
            ExprKind::GenericCall { .. } => self.bad(e.span, "generic calls are not lowered yet"),
            ExprKind::Cast { expr, ty, kind } => self.cast(e, expr, ty, *kind),
            ExprKind::Unsafe(_) => self.bad(e.span, "`unsafe` blocks are not lowered yet"),
            ExprKind::Tuple(_) => self.bad(e.span, "tuple values are not lowered yet"),
            ExprKind::Array(items) => self.array_literal(e, items),
            ExprKind::Map(pairs) => self.map_literal(e, pairs),
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
            Lit::None => return self.null_temp(),
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
            None => {
                // `""`: the lexer drops empty text parts, so there are no
                // parts to concatenate. Still intern the empty string so a
                // zero-length string-data symbol exists downstream.
                let sid = StrId(self.intern_string(""));
                let t = self.temp();
                self.instr(IrInstr::Const {
                    dst: t,
                    c: IrConst::Str(sid),
                });
                Ok(t)
            }
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
        // Inside an instance method a bare field name reads `this.field`.
        if let Some(cid) = self.owner {
            if let Some(slot) = self.instance_field_index(cid, name) {
                let field_ty = self.field_at(cid, slot).ty.clone();
                let this = self.this_value(e)?;
                return self.field_read(e.span, this, &field_ty, slot);
            }
            // Inside an instance method a bare property name reads the getter.
            if let Some(&fid) = self.property_ids.get(&(cid as u32, name.to_string(), false)) {
                let this = self.this_value(e)?;
                return self.call_method(e, fid, &[], Some(this));
            }
            // A bare name may also be a static field of the enclosing class.
            if let Some((slot, info)) = self.static_field(cid, name) {
                return self.static_read(e.span, cid as u32, slot, &info.ty);
            }
            // ... or a constant of the enclosing class.
            if let Some(r) = self.read_class_const(e.span, cid, name) {
                return r;
            }
        }
        self.bad(e.span, format!("using `{name}` as a value is not lowered yet"))
    }

    fn this_value(&mut self, e: &Expr) -> Result<Temp, ()> {
        match self.lookup("this") {
            Some(slot) => Ok(self.load(slot)),
            None => self.bad(e.span, "`this` is not available here"),
        }
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
            AstBinOp::NullCoalesce => return self.null_coalesce(e, lhs, rhs),
            AstBinOp::Range | AstBinOp::RangeIncl | AstBinOp::Send
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
        let mut a = self.expr(lhs)?;
        let mut b = self.expr(rhs)?;
        let aty = self.irty(lhs.span).ok();
        let bty = self.irty(rhs.span).ok();
        if matches!(aty, Some(IrTy::Float)) && matches!(bty, Some(IrTy::Int)) {
            let t = self.temp();
            self.instr(IrInstr::Itof { dst: t, v: b });
            b = t;
        } else if matches!(aty, Some(IrTy::Int)) && matches!(bty, Some(IrTy::Float)) {
            let t = self.temp();
            self.instr(IrInstr::Itof { dst: t, v: a });
            a = t;
        }
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

    /// Test a pointer-valued option for presence (`ptr != null`), yielding a
    /// `Bool` temp so it can drive a branch.
    fn opt_is_present(&mut self, p: Temp) -> Result<Temp, ()> {
        let null = self.null_temp()?;
        let dst = self.temp();
        self.instr(IrInstr::BinOp {
            dst,
            op: IrBinOp::Ne,
            a: p,
            b: null,
        });
        Ok(dst)
    }

    /// Resolve a present option pointer to its inner value: scalars are
    /// unboxed to the element IR type; managed values pass through.
    fn opt_resolve(&mut self, p: Temp, inner: &Ty, span: Span) -> Result<Temp, ()> {
        match self.map_ty(inner, span)? {
            IrTy::Int => {
                self.extern_call_t1("pickle_unbox_i64", vec![IrTy::Ptr], IrTy::Int, vec![p])
            }
            IrTy::Float => {
                self.extern_call_t1("pickle_unbox_f64", vec![IrTy::Ptr], IrTy::Float, vec![p])
            }
            IrTy::Bool => {
                self.extern_call_t1("pickle_unbox_bool", vec![IrTy::Ptr], IrTy::Bool, vec![p])
            }
            IrTy::Char => {
                self.extern_call_t1("pickle_unbox_char", vec![IrTy::Ptr], IrTy::Char, vec![p])
            }
            _ => Ok(p),
        }
    }

    /// `a ?? b`: yield the unwrapped `a` when present, else `b`. `a` is an
    /// option pointer; the result is the inner type (scalar or managed).
    fn null_coalesce(&mut self, e: &Expr, lhs: &Expr, rhs: &Expr) -> Result<Temp, ()> {
        let Some(inner) = self.ty_of(&lhs.span).and_then(|t| t.inner_option()) else {
            return self.bad(e.span, "left side of `??` is not an option");
        };
        let res_ir = self.map_ty(&inner, e.span)?;
        let res_slot = self.new_slot(res_ir);
        let a = self.expr(lhs)?;
        let present = self.opt_is_present(a)?;
        let some_id = self.new_block();
        let rhs_id = self.new_block();
        let join = self.new_block();
        self.term(IrTerm::BranchIf {
            cond: present,
            then: some_id,
            else_: rhs_id,
        });
        self.cur = some_id;
        let av = self.opt_resolve(a, &inner, e.span)?;
        self.instr(IrInstr::StoreSlot { slot: res_slot, v: av });
        self.term(IrTerm::Branch { target: join });
        self.cur = rhs_id;
        let b = self.expr(rhs)?;
        let b = if res_ir == IrTy::Ptr {
            let vt = self.ty_of(&rhs.span).unwrap_or(Ty::Unknown);
            self.option_wrap(b, &vt, rhs.span)?
        } else {
            b
        };
        self.instr(IrInstr::StoreSlot { slot: res_slot, v: b });
        self.term(IrTerm::Branch { target: join });
        self.cur = join;
        Ok(self.load(res_slot))
    }

    /// `a?`: unwrap `a` to its inner value, panicking at runtime on `none`.
    fn opt_unwrap(&mut self, e: &Expr, operand: &Expr) -> Result<Temp, ()> {
        let Some(inner) = self.ty_of(&operand.span).and_then(|t| t.inner_option()) else {
            return self.bad(e.span, "`?` operand is not an option");
        };
        let p = self.expr(operand)?;
        self.opt_unwrap_val(e.span, p, &inner)
    }

    /// Unwrap an already-evaluated option pointer `p` (type `inner?`) to its
    /// inner value, panicking at runtime on `none`.
    fn opt_unwrap_val(&mut self, span: Span, p: Temp, inner: &Ty) -> Result<Temp, ()> {
        let res_ir = self.map_ty(inner, span)?;
        let res_slot = self.new_slot(res_ir);
        let present = self.opt_is_present(p)?;
        let some_id = self.new_block();
        let none_id = self.new_block();
        let join = self.new_block();
        self.term(IrTerm::BranchIf {
            cond: present,
            then: some_id,
            else_: none_id,
        });
        self.cur = none_id;
        self.extern_call_void("pickle_panic_none_unwrap", vec![], vec![]);
        self.term(IrTerm::Branch { target: join });
        self.cur = some_id;
        let v = self.opt_resolve(p, inner, span)?;
        self.instr(IrInstr::StoreSlot { slot: res_slot, v });
        self.term(IrTerm::Branch { target: join });
        self.cur = join;
        Ok(self.load(res_slot))
    }

    /// `a?.name`: when `a` is present, read `name` off the receiver and lift it
    /// to `T?`; when it is `none`, the result is `none`. The receiver is
    /// evaluated exactly once.
    fn opt_access(&mut self, e: &Expr, object: &Expr, name: &str) -> Result<Temp, ()> {
        let Some(inner) = self.ty_of(&object.span).and_then(|t| t.inner_option()) else {
            return self.bad(e.span, "`?.` requires an option receiver");
        };
        let o = self.expr(object)?;
        let res_slot = self.new_slot(IrTy::Ptr);
        let present = self.opt_is_present(o)?;
        let some_id = self.new_block();
        let none_id = self.new_block();
        let join = self.new_block();
        self.term(IrTerm::BranchIf {
            cond: present,
            then: some_id,
            else_: none_id,
        });
        self.cur = none_id;
        let n = self.null_temp()?;
        self.instr(IrInstr::StoreSlot { slot: res_slot, v: n });
        self.term(IrTerm::Branch { target: join });
        self.cur = some_id;
        let (member_ty, mv) = self.opt_member_read(e, o, &inner, name)?;
        let mv = self.option_wrap(mv, &member_ty, e.span)?;
        self.instr(IrInstr::StoreSlot { slot: res_slot, v: mv });
        self.term(IrTerm::Branch { target: join });
        self.cur = join;
        Ok(self.load(res_slot))
    }

    /// Read field `name` (or dispatch the getter for property `name`) from an
    /// already-evaluated class/struct receiver, returning its declared type and
    /// the value temp.
    fn opt_member_read(
        &mut self,
        e: &Expr,
        obj: Temp,
        recv_ty: &Ty,
        name: &str,
    ) -> Result<(Ty, Temp), ()> {
        let (cid, cname) = match recv_ty {
            Ty::Class(cn, _) | Ty::Struct(cn, _) => {
                (self.class_by_name.get(cn).copied(), cn.clone())
            }
            _ => (None, String::new()),
        };
        let Some(cid) = cid else {
            return self
                .bad(e.span, "optional access is only lowered for class/struct members");
        };
        if let Some(slot) = self.instance_field_index(cid as i64, name) {
            let field_ty = self.field_at(cid as i64, slot).ty.clone();
            let v = self.field_read(e.span, obj, &field_ty, slot)?;
            return Ok((field_ty, v));
        }
        if let Some(&fid) = self.property_ids.get(&(cid, name.to_string(), false)) {
            let pty = self.property_ty_of(&cname, name);
            let v = self.call_method(e, fid, &[], Some(obj))?;
            return Ok((pty, v));
        }
        self.bad(
            e.span,
            format!("no field or property `{name}` for optional access"),
        )
    }

    /// The declared type of property `name` on class `cname`.
    fn property_ty_of(&self, cname: &str, name: &str) -> Ty {
        match self.resolved.types.get(cname) {
            Some(TypeTableEntry::Class(t)) | Some(TypeTableEntry::Struct(t)) => t
                .properties
                .iter()
                .find(|p| p.name == name)
                .map(|p| p.ty.clone())
                .unwrap_or(Ty::Unknown),
            _ => Ty::Unknown,
        }
    }

    // ---- casts -----------------------------------------------------------

    /// `x is T` / `x as T` / `x as? T`. The result stays in the typed IR value
    /// domain: `is` -> bool, `as` -> `T`, `as?` -> `T?` (a pointer).
    fn cast(&mut self, e: &Expr, operand: &Expr, ty: &TypeExpr, kind: CastKind) -> Result<Temp, ()> {
        let src = self.ty_of(&operand.span).unwrap_or(Ty::Unknown);
        let dst = self.resolved.resolve_ty(ty, &[], self.diags);
        match kind {
            CastKind::Is => self.cast_is(e, operand, &src, &dst),
            CastKind::As => self.cast_as(e, operand, &src, &dst),
            CastKind::TryAs => {
                // `x as? T` always yields an option, never panics.
                let v = self.expr(operand)?;
                let inner = dst.inner_option().unwrap_or_else(|| dst.clone());
                self.cast_to_option(e.span, v, &src, &inner)
            }
        }
    }

    fn cast_is(&mut self, e: &Expr, operand: &Expr, src: &Ty, dst: &Ty) -> Result<Temp, ()> {
        // A cast strips at most one option layer on each side; the underlying
        // (non-option) types decide whether the test can succeed, because in
        // this statically-typed subset the runtime type of a value is known
        // from its static type.
        let src_opt = src.inner_option();
        let dst_opt = dst.inner_option();
        let base_src = src_opt.clone().unwrap_or_else(|| src.clone());
        let base_dst = dst_opt.clone().unwrap_or_else(|| dst.clone());
        let v = self.expr(operand)?;
        if base_src == base_dst {
            if src_opt.is_some() && dst_opt.is_none() {
                // `x: T?` is `T` -> "is it present?".
                return self.opt_is_present(v);
            }
            // `T is T`, `T is T?`, `T? is T?` -> always true.
            return Ok(self.bool_const(true));
        }
        if base_src.is_numeric() && base_dst.is_numeric() {
            // `int is float` etc. is a static mismatch; `v` was evaluated above.
            return Ok(self.bool_const(false));
        }
        // Class/struct types along the inheritance chain: an upcast (the target
        // is an ancestor of the static type) always holds; a downcast needs a
        // runtime identity test; unrelated classes can never match.
        if let (Some((sn, _)), Some((dn, dcid))) =
            (self.user_class_id(&base_src), self.user_class_id(&base_dst))
        {
            if self.is_ancestor(&dn, &sn) {
                return Ok(self.bool_const(true));
            }
            if self.is_ancestor(&sn, &dn) {
                let dc = self.int_const(dcid as i64);
                return self.extern_call_t1(
                    "pickle_class_is",
                    vec![IrTy::Ptr, IrTy::Int],
                    IrTy::Bool,
                    vec![v, dc],
                );
            }
            return Ok(self.bool_const(false));
        }
        self.bad(
            e.span,
            format!("`{src} is {dst}` is not lowered yet (class/interface tests need inheritance)"),
        )
    }

    fn cast_as(&mut self, e: &Expr, operand: &Expr, src: &Ty, dst: &Ty) -> Result<Temp, ()> {
        let v = self.expr(operand)?;
        if let Some(inner) = dst.inner_option() {
            // Casting to an option type never panics: `none` stays `none`.
            return self.cast_to_option(e.span, v, src, &inner);
        }
        if let Some(si) = src.inner_option() {
            // `x: T?` as `U` asserts presence: unwrap (panics on none) then
            // convert the inner value.
            let ival = self.opt_unwrap_val(e.span, v, &si)?;
            return self.convert(e.span, ival, &si, dst);
        }
        self.convert(e.span, v, src, dst)
    }

    /// Produce the option `inner?` from an already-evaluated value `v` of type
    /// `src`. A managed source passes through; scalars are boxed. When `src` is
    /// itself an option, `none` is preserved and the present value is converted.
    fn cast_to_option(
        &mut self,
        span: Span,
        v: Temp,
        src: &Ty,
        inner: &Ty,
    ) -> Result<Temp, ()> {
        if let Some(si) = src.inner_option() {
            if &si == inner {
                // Already the requested option; `none` is preserved.
                return Ok(v);
            }
            if !(si.is_numeric() && inner.is_numeric()) {
                return self.bad(
                    span,
                    format!("`{src} as? {inner}` is not lowered yet"),
                );
            }
            // Present -> convert the inner; absent -> none.
            let res_slot = self.new_slot(IrTy::Ptr);
            let present = self.opt_is_present(v)?;
            let some_id = self.new_block();
            let none_id = self.new_block();
            let join = self.new_block();
            self.term(IrTerm::BranchIf {
                cond: present,
                then: some_id,
                else_: none_id,
            });
            self.cur = none_id;
            let n = self.null_temp()?;
            self.instr(IrInstr::StoreSlot { slot: res_slot, v: n });
            self.term(IrTerm::Branch { target: join });
            self.cur = some_id;
            let iv = self.opt_resolve(v, &si, span)?;
            let cv = self.convert(span, iv, &si, inner)?;
            let wrapped = self.option_wrap(cv, inner, span)?;
            self.instr(IrInstr::StoreSlot { slot: res_slot, v: wrapped });
            self.term(IrTerm::Branch { target: join });
            self.cur = join;
            return Ok(self.load(res_slot));
        }
        let cv = self.convert(span, v, src, inner)?;
        self.option_wrap(cv, inner, span)
    }

    /// Convert a non-option value between related (non-option) types. Identity
    /// and numeric conversions are supported; anything else bails.
    fn convert(&mut self, span: Span, v: Temp, from: &Ty, to: &Ty) -> Result<Temp, ()> {
        if from == to {
            return Ok(v);
        }
        if from.is_numeric() && to.is_numeric() {
            let dst = self.temp();
            let instr = match (from, to) {
                (Ty::Int, Ty::Float) => IrInstr::Itof { dst, v },
                (Ty::Float, Ty::Int) => IrInstr::Ftoi { dst, v },
                _ => return self.bad(span, format!("cannot cast `{from}` to `{to}`")),
            };
            self.instr(instr);
            return Ok(dst);
        }
        // Class/struct casts: an upcast reinterprets the same object; a
        // downcast is checked by the runtime and panics on a mismatch.
        if let (Some((sn, _)), Some((dn, dcid))) =
            (self.user_class_id(from), self.user_class_id(to))
        {
            if self.is_ancestor(&dn, &sn) {
                return Ok(v);
            }
            let dc = self.int_const(dcid as i64);
            return self.extern_call_t1(
                "pickle_class_cast",
                vec![IrTy::Ptr, IrTy::Int],
                IrTy::Ptr,
                vec![v, dc],
            );
        }
        self.bad(
            span,
            format!("`{from} as {to}` is not lowered yet (class/interface casts need inheritance)"),
        )
    }

    fn bool_const(&mut self, b: bool) -> Temp {
        let dst = self.temp();
        self.instr(IrInstr::Const {
            dst,
            c: IrConst::Bool(b),
        });
        dst
    }

    fn assign(&mut self, e: &Expr, target: &Expr, op: AssignOp, value: &Expr) -> Result<Temp, ()> {
        let span = target.span;
        if let ExprKind::Index { object, index } = &target.kind {
            return self.index_assign(e, op, object, index, value);
        }
        if let ExprKind::Member { object, name } = &target.kind {
            return self.member_assign(e, op, object, name, value);
        }
        let ExprKind::Ident(name) = &target.kind else {
            return self.bad(span, "assignment targets other than names are not lowered yet");
        };
        let v = self.expr(value)?;
        let vt = self.ty_of(&value.span).unwrap_or(Ty::Unknown);
        if let Some(slot) = self.lookup(name) {
            if op == AssignOp::Assign {
                let sv = if matches!(self.fslots.get(slot.0 as usize), Some(IrTy::Ptr)) {
                    self.option_wrap(v, &vt, value.span)?
                } else {
                    v
                };
                self.instr(IrInstr::StoreSlot { slot, v: sv });
                return Ok(v);
            }
            let cur = self.load(slot);
            let dst = match self.fslots.get(slot.0 as usize).copied() {
                Some(IrTy::Str) => self.extern_call_t1(
                    "pickle_str_concat",
                    vec![IrTy::Str, IrTy::Str],
                    IrTy::Str,
                    vec![cur, v],
                )?,
                _ => {
                    let t = self.temp();
                    self.instr(IrInstr::BinOp {
                        dst: t,
                        op: assign_opcode(op),
                        a: cur,
                        b: v,
                    });
                    t
                }
            };
            self.instr(IrInstr::StoreSlot { slot, v: dst });
            return Ok(dst);
        }
        // Inside an instance method a bare field name assigns `this.field`.
        if let Some(cid) = self.owner {
            if let Some(idx) = self.instance_field_index(cid, name) {
                let field_ty = self.field_at(cid, idx).ty.clone();
                let this = self.this_value(e)?;
                return self.field_assign(e.span, op, this, &field_ty, idx, v, &vt);
            }
            // A bare name may also be a static field of the enclosing class.
            if let Some((slot, info)) = self.static_field(cid, name) {
                if !info.mutable {
                    return self.bad(
                        span,
                        format!("cannot assign to immutable static field `{name}`"),
                    );
                }
                return self.static_assign(span, op, cid as u32, slot, &info.ty, v, &vt);
            }
        }
        self.bad(span, format!("cannot assign to `{name}`"))
    }

    /// `obj.field` and `obj.field op= value` on class/struct instances.
    fn member_assign(
        &mut self,
        e: &Expr,
        op: AssignOp,
        object: &Expr,
        name: &str,
        value: &Expr,
    ) -> Result<Temp, ()> {
        // `Type.staticProperty = v` dispatches the receiver-less setter.
        if let ExprKind::Ident(tname) = &object.kind {
            if let Some(&cid) = self.class_by_name.get(tname) {
                if let Some(&fid) = self.static_property_ids.get(&(cid, name.to_string(), true)) {
                    if op != AssignOp::Assign {
                        return self.bad(
                            e.span,
                            "compound assignment to a static property is not lowered yet",
                        );
                    }
                    let v = self.expr(value)?;
                    let vt = self.ty_of(&value.span).unwrap_or(Ty::Unknown);
                    let packed = if matches!(
                        self.module.funcs[fid.0].params.first().map(|p| p.ty),
                        Some(IrTy::Ptr)
                    ) {
                        self.option_wrap(v, &vt, value.span)?
                    } else {
                        v
                    };
                    let dst = self.temp();
                    self.instr(IrInstr::Call {
                        dst: Some(dst),
                        callee: Callee::Func(fid),
                        args: vec![packed],
                    });
                    return Ok(v);
                }
                if let Some((slot, info)) = self.static_field(cid as i64, name) {
                    if !info.mutable {
                        return self.bad(
                            e.span,
                            format!("cannot assign to immutable static field `{name}`"),
                        );
                    }
                    let v = self.expr(value)?;
                    let vt = self.ty_of(&value.span).unwrap_or(Ty::Unknown);
                    return self.static_assign(e.span, op, cid, slot, &info.ty, v, &vt);
                }
                return self.bad(
                    e.span,
                    format!("`{tname}` has no settable static member `{name}`"),
                );
            }
        }
        let ot = self.ty_of(&object.span);
        let cid = match ot {
            Some(Ty::Class(cn, _)) | Some(Ty::Struct(cn, _)) => {
                self.class_by_name.get(&cn).copied()
            }
            _ => None,
        };
        let Some(cid) = cid else {
            return self.bad(e.span, "assignment over this member type is not lowered yet");
        };
        let Some(idx) = self.instance_field_index(cid as i64, name) else {
            // `obj.prop = value` dispatches the property's setter.
            if let Some(&fid) = self.property_ids.get(&(cid, name.to_string(), true)) {
                if op != AssignOp::Assign {
                    return self.bad(e.span, "compound assignment to a property is not lowered yet");
                }
                let obj = self.expr(object)?;
                let v = self.expr(value)?;
                let vt = self.ty_of(&value.span).unwrap_or(Ty::Unknown);
                let packed = if matches!(
                    self.module.funcs[fid.0].params.get(1).map(|p| p.ty),
                    Some(IrTy::Ptr)
                ) {
                    self.option_wrap(v, &vt, value.span)?
                } else {
                    v
                };
                let dst = self.temp();
                self.instr(IrInstr::Call {
                    dst: Some(dst),
                    callee: Callee::Func(fid),
                    args: vec![obj, packed],
                });
                return Ok(v);
            }
            return self.bad(
                e.span,
                format!(
                    "`{name}` is not a field or settable property of this `{}`",
                    self.class_name_of(cid)
                ),
            );
        };
        let field_ty = self.field_at(cid as i64, idx).ty.clone();
        let v = self.expr(value)?;
        let vvt = self.ty_of(&value.span).unwrap_or(Ty::Unknown);
        let obj = self.expr(object)?;
        self.field_assign(e.span, op, obj, &field_ty, idx, v, &vvt)
    }

    /// Read-modify-write/plain store of one field slot.
    #[allow(clippy::too_many_arguments)]
    fn field_assign(
        &mut self,
        span: Span,
        op: AssignOp,
        obj: Temp,
        field_ty: &Ty,
        slot: usize,
        v: Temp,
        vt: &Ty,
    ) -> Result<Temp, ()> {
        let rep = self.elem_rep(field_ty, span)?;
        if op == AssignOp::Assign {
            let packed = self.pack_for_pointer_boundary(&rep, v, vt, span)?;
            let boxed = self.box_for_store(&rep, packed, elem_ir(field_ty))?;
            self.field_store(obj, slot as i64, field_ty, boxed);
            return Ok(v);
        }
        if matches!(rep, ElemRep::Ptr) {
            return self.bad(span, "compound assignment to a non-scalar field is not lowered yet");
        }
        let obj_c = obj;
        // Read the current value (unboxed), combine, write back.
        let idx = self.int_const(slot as i64);
        let raw = self.extern_call_t1(
            "pickle_obj_slot_get",
            vec![IrTy::Ptr, IrTy::Int],
            IrTy::Ptr,
            vec![obj_c, idx],
        )?;
        let cur = match rep {
            ElemRep::Scalar(_, unbox, ir) => {
                self.extern_call_t1(unbox, vec![IrTy::Ptr], ir, vec![raw])?
            }
            ElemRep::Ptr => raw,
        };
        let dst = self.temp();
        self.instr(IrInstr::BinOp {
            dst,
            op: assign_opcode(op),
            a: cur,
            b: v,
        });
        let boxed = self.box_for_store(&rep, dst, elem_ir(field_ty))?;
        self.field_store(obj_c, slot as i64, field_ty, boxed);
        Ok(dst)
    }

    /// Emit `pickle_obj_slot_set(obj, slot, boxed_value)`.
    fn field_store(&mut self, obj: Temp, slot: i64, _field_ty: &Ty, boxed: Temp) {
        let idx = self.int_const(slot);
        self.extern_call_void(
            "pickle_obj_slot_set",
            vec![IrTy::Ptr, IrTy::Int, IrTy::Ptr],
            vec![obj, idx, boxed],
        );
    }

    /// Declared info of the static field at static slot `idx` of class `cid`.
    fn static_field_at(&self, cid: i64, idx: usize) -> Option<FieldInfo> {
        let plan = self.classes.iter().find(|p| p.class_id as i64 == cid)?;
        plan.table.fields.iter().filter(|f| f.is_static).nth(idx).cloned()
    }

    /// Static-field read: `pickle_static_get` then unbox scalars.
    fn static_read(
        &mut self,
        span: Span,
        cid: u32,
        slot: usize,
        field_ty: &Ty,
    ) -> Result<Temp, ()> {
        let rep = self.elem_rep(field_ty, span)?;
        let c = self.int_const(cid as i64);
        let s = self.int_const(slot as i64);
        let raw = self.extern_call_t1(
            "pickle_static_get",
            vec![IrTy::Int, IrTy::Int],
            IrTy::Ptr,
            vec![c, s],
        )?;
        match rep {
            ElemRep::Scalar(_, unbox, ir) => {
                self.extern_call_t1(unbox, vec![IrTy::Ptr], ir, vec![raw])
            }
            ElemRep::Ptr => Ok(raw),
        }
    }

    /// Static-field store: box scalars, then `pickle_static_set(cid, slot, v)`.
    fn static_store(&mut self, cid: u32, slot: usize, boxed: Temp) {
        let c = self.int_const(cid as i64);
        let s = self.int_const(slot as i64);
        self.extern_call_void(
            "pickle_static_set",
            vec![IrTy::Int, IrTy::Int, IrTy::Ptr],
            vec![c, s, boxed],
        );
    }

    /// Plain or compound (`op=`) store of one static field cell.
    #[allow(clippy::too_many_arguments)]
    fn static_assign(
        &mut self,
        span: Span,
        op: AssignOp,
        cid: u32,
        slot: usize,
        field_ty: &Ty,
        v: Temp,
        vt: &Ty,
    ) -> Result<Temp, ()> {
        let rep = self.elem_rep(field_ty, span)?;
        if op == AssignOp::Assign {
            let packed = self.pack_for_pointer_boundary(&rep, v, vt, span)?;
            let boxed = self.box_for_store(&rep, packed, elem_ir(field_ty))?;
            self.static_store(cid, slot, boxed);
            return Ok(v);
        }
        if matches!(rep, ElemRep::Ptr) {
            return self.bad(
                span,
                "compound assignment to a non-scalar static field is not lowered yet",
            );
        }
        let c = self.int_const(cid as i64);
        let s = self.int_const(slot as i64);
        let raw = self.extern_call_t1(
            "pickle_static_get",
            vec![IrTy::Int, IrTy::Int],
            IrTy::Ptr,
            vec![c, s],
        )?;
        let cur = match rep {
            ElemRep::Scalar(_, unbox, ir) => {
                self.extern_call_t1(unbox, vec![IrTy::Ptr], ir, vec![raw])?
            }
            ElemRep::Ptr => raw,
        };
        let dst = self.temp();
        self.instr(IrInstr::BinOp {
            dst,
            op: assign_opcode(op),
            a: cur,
            b: v,
        });
        let boxed = self.box_for_store(&rep, dst, elem_ir(field_ty))?;
        self.static_store(cid, slot, boxed);
        Ok(dst)
    }

    /// Store the default (zero scalar / null pointer) into one static cell.
    fn build_static_default(&mut self, cid: u32, slot: usize) -> Result<(), ()> {
        let Some(info) = self.static_field_at(cid as i64, slot) else {
            return Ok(());
        };
        let field_ty = info.ty.clone();
        let rep = self.elem_rep(&field_ty, info.span)?;
        let default = match rep {
            ElemRep::Scalar(..) => self.zero_scalar(elem_ir(&field_ty), info.span)?,
            ElemRep::Ptr => self.null_temp()?,
        };
        let boxed = self.box_for_store(&rep, default, elem_ir(&field_ty))?;
        self.static_store(cid, slot, boxed);
        Ok(())
    }

    /// Evaluate one static-field initializer and store it.
    fn build_static_init_one(&mut self, cid: u32, slot: usize, x: &Expr) -> Result<(), ()> {
        let Some(info) = self.static_field_at(cid as i64, slot) else {
            return Ok(());
        };
        let field_ty = info.ty.clone();
        let v = self.expr(x)?;
        let vt = self.ty_of(&x.span).unwrap_or(Ty::Unknown);
        self.static_assign(x.span, AssignOp::Assign, cid, slot, &field_ty, v, &vt)?;
        Ok(())
    }

    /// `xs[i] = v` and `xs[i] op= v` for `List<T>` targets.
    fn index_assign(
        &mut self,
        e: &Expr,
        op: AssignOp,
        object: &Expr,
        index: &Expr,
        value: &Expr,
    ) -> Result<Temp, ()> {
        let ot = self.ty_of(&object.span);
        if let Some(Ty::Map(k, v)) = &ot {
            return self.map_index_assign(
                op,
                object,
                index,
                value,
                k.as_ref().clone(),
                v.as_ref().clone(),
            );
        }
        if !matches!(ot, Some(Ty::List(_))) {
            // Maps are handled above; other types are typed but unlowered.
            return self.bad(e.span, "index assignment over this type is not lowered yet");
        }
        let elem = match &ot {
            Some(Ty::List(inner)) => inner.as_ref().clone(),
            _ => unreachable!(),
        };
        let rep = self.elem_rep(&elem, e.span)?;
        let obj = self.expr(object)?;
        let idx = self.expr(index)?;
        if op == AssignOp::Assign {
            let v = self.expr(value)?;
            self.list_store(obj, idx, &rep, &elem, v)?;
            return Ok(v);
        }
        if matches!(rep, ElemRep::Ptr) {
            return self.bad(
                e.span,
                "compound assignment to a non-scalar list element is not lowered yet",
            );
        }
        let cur = self.extern_call_t1(
            "pickle_list_get",
            vec![IrTy::Ptr, IrTy::Int],
            IrTy::Ptr,
            vec![obj, idx],
        )?;
        let cur_v = match rep {
            ElemRep::Scalar(_, unbox_sym, ir) => {
                self.extern_call_t1(unbox_sym, vec![IrTy::Ptr], ir, vec![cur])?
            }
            ElemRep::Ptr => cur,
        };
        let v = self.expr(value)?;
        let dst = self.temp();
        self.instr(IrInstr::BinOp {
            dst,
            op: assign_opcode(op),
            a: cur_v,
            b: v,
        });
        self.list_store(obj, idx, &rep, &elem, dst)?;
        Ok(dst)
    }

    /// `pickle_list_set(obj, idx, boxed)`: boxes a scalar element first.
    fn list_store(
        &mut self,
        obj: Temp,
        idx: Temp,
        rep: &ElemRep,
        elem: &Ty,
        v: Temp,
    ) -> Result<(), ()> {
        let boxed = match rep {
            ElemRep::Scalar(box_sym, _, _) => {
                self.extern_call_t1(box_sym, vec![elem_ir(elem)], IrTy::Ptr, vec![v])?
            }
            ElemRep::Ptr => v,
        };
        self.extern_call_void(
            "pickle_list_set",
            vec![IrTy::Ptr, IrTy::Int, IrTy::Ptr],
            vec![obj, idx, boxed],
        );
        Ok(())
    }

    /// `xs[i]` element read for a `List<T>` or `Map<string, V>` (and a clear
    /// error for strings).
    fn index_read(&mut self, e: &Expr, object: &Expr, index: &Expr) -> Result<Temp, ()> {
        if let Some(Ty::Map(k, v)) = self.ty_of(&object.span) {
            return self.map_index_read(
                e,
                object,
                index,
                k.as_ref().clone(),
                v.as_ref().clone(),
            );
        }
        let ot = self.ty_of(&object.span);
        let elem = match ot {
            Some(Ty::List(inner)) => inner.as_ref().clone(),
            Some(Ty::String) => {
                let obj = self.expr(object)?;
                let idx = self.expr(index)?;
                return self.extern_call_t1(
                    "pickle_str_get",
                    vec![IrTy::Ptr, IrTy::Int],
                    IrTy::Char,
                    vec![obj, idx],
                );
            }
            _ => return self.bad(e.span, "indexing this type is not lowered yet"),
        };
        let rep = self.elem_rep(&elem, e.span)?;
        let obj = self.expr(object)?;
        let idx = self.expr(index)?;
        let raw = self.extern_call_t1(
            "pickle_list_get",
            vec![IrTy::Ptr, IrTy::Int],
            IrTy::Ptr,
            vec![obj, idx],
        )?;
        match rep {
            ElemRep::Scalar(_, unbox_sym, ir) => {
                self.extern_call_t1(unbox_sym, vec![IrTy::Ptr], ir, vec![raw])
            }
            ElemRep::Ptr => Ok(raw),
        }
    }

    /// `m[k]` read for a `Map<string, V>`. Absent keys yield the value type's
    /// default (`0`/`0.0`/`false`, the empty string, or null) via the runtime's
    /// boxed get, so a scalar is never unboxed from a null pointer.
    fn map_index_read(
        &mut self,
        e: &Expr,
        object: &Expr,
        index: &Expr,
        kty: Ty,
        vty: Ty,
    ) -> Result<Temp, ()> {
        if kty != Ty::String {
            return self.bad(index.span, "map keys must be `string` values");
        }
        let vrep = self.elem_rep(&vty, e.span)?;
        let obj = self.expr(object)?;
        let k = self.expr(index)?;
        if !matches!(self.irty(index.span)?, IrTy::Str) {
            return self.bad(index.span, "map keys must be `string` values");
        }
        let default: Temp = match vrep {
            ElemRep::Scalar(box_sym, _, ir) => {
                let zero = self.zero_scalar(ir, index.span)?;
                self.extern_call_t1(box_sym, vec![ir], IrTy::Ptr, vec![zero])?
            }
            ElemRep::Ptr => {
                if vty == Ty::String {
                    self.string_literal(&[])?
                } else {
                    let nul = self.null_temp()?;
                    self.extern_call_t1(
                        "pickle_map_get_boxed",
                        vec![IrTy::Ptr, IrTy::Str, IrTy::Ptr],
                        IrTy::Ptr,
                        vec![obj, k, nul],
                    )?
                }
            }
        };
        let raw = self.extern_call_t1(
            "pickle_map_get_boxed",
            vec![IrTy::Ptr, IrTy::Str, IrTy::Ptr],
            IrTy::Ptr,
            vec![obj, k, default],
        )?;
        match vrep {
            ElemRep::Scalar(_, unbox_sym, ir) => {
                self.extern_call_t1(unbox_sym, vec![IrTy::Ptr], ir, vec![raw])
            }
            ElemRep::Ptr => Ok(raw),
        }
    }

    /// `m[k] = v` and `m[k] op= v` for `Map<string, V>` targets.
    fn map_index_assign(
        &mut self,
        op: AssignOp,
        object: &Expr,
        index: &Expr,
        value: &Expr,
        kty: Ty,
        vty: Ty,
    ) -> Result<Temp, ()> {
        if kty != Ty::String {
            return self.bad(index.span, "map keys must be `string` values");
        }
        let vrep = self.elem_rep(&vty, object.span)?;
        let obj = self.expr(object)?;
        let k = self.expr(index)?;
        if !matches!(self.irty(index.span)?, IrTy::Str) {
            return self.bad(index.span, "map keys must be `string` values");
        }
        if op == AssignOp::Assign {
            let v = self.expr(value)?;
            let v_ty = self.irty(value.span)?;
            let boxed = self.box_for_store(&vrep, v, v_ty)?;
            self.extern_call_void(
                "pickle_map_set",
                vec![IrTy::Ptr, IrTy::Str, IrTy::Ptr],
                vec![obj, k, boxed],
            );
            return Ok(v);
        }
        let (box_sym, unbox_sym, ir) = match vrep {
            ElemRep::Scalar(box_sym, unbox_sym, ir) => (box_sym, unbox_sym, ir),
            ElemRep::Ptr => {
                return self.bad(
                    object.span,
                    "compound assignment to a non-scalar map value is not lowered yet",
                )
            }
        };
        let zero = self.zero_scalar(ir, object.span)?;
        let default = self.extern_call_t1(box_sym, vec![ir], IrTy::Ptr, vec![zero])?;
        let cur_ptr = self.extern_call_t1(
            "pickle_map_get_boxed",
            vec![IrTy::Ptr, IrTy::Str, IrTy::Ptr],
            IrTy::Ptr,
            vec![obj, k, default],
        )?;
        let cur = self.extern_call_t1(unbox_sym, vec![IrTy::Ptr], ir, vec![cur_ptr])?;
        let v = self.expr(value)?;
        let dst = self.temp();
        self.instr(IrInstr::BinOp {
            dst,
            op: assign_opcode(op),
            a: cur,
            b: v,
        });
        let boxed = self.extern_call_t1(box_sym, vec![ir], IrTy::Ptr, vec![dst])?;
        self.extern_call_void(
            "pickle_map_set",
            vec![IrTy::Ptr, IrTy::Str, IrTy::Ptr],
            vec![obj, k, boxed],
        );
        Ok(dst)
    }

    /// `[a, b, c]` array literal: build a `List` by pushing each element,
    /// boxing scalar elements along the way.
    fn array_literal(&mut self, e: &Expr, items: &[Expr]) -> Result<Temp, ()> {
        let elem = match self.ty_of(&e.span) {
            Some(Ty::List(inner)) => inner.as_ref().clone(),
            _ => return self.bad(e.span, "array literal does not have a `List` type"),
        };
        if items.is_empty() {
            return self.bad(
                e.span,
                "empty array literal cannot be typed; add an element to infer `List<T>`",
            );
        }
        let rep = self.elem_rep(&elem, e.span)?;
        let cap = self.temp();
        self.instr(IrInstr::Const {
            dst: cap,
            c: IrConst::Int(items.len() as i64),
        });
        let list = self.extern_call_t1("pickle_list_new", vec![IrTy::Int], IrTy::Ptr, vec![cap])?;
        for it in items {
            let v = self.expr(it)?;
            let push_v = match rep {
                ElemRep::Scalar(box_sym, _, _) => {
                    let v_ty = self.irty(it.span)?;
                    self.extern_call_t1(box_sym, vec![v_ty], IrTy::Ptr, vec![v])?
                }
                ElemRep::Ptr => v,
            };
            self.extern_call_void("pickle_list_push", vec![IrTy::Ptr, IrTy::Ptr], vec![list, push_v]);
        }
        Ok(list)
    }

    /// `{ "k": v, ... }` map literal: build a `Map` by inserting each entry,
    /// boxing scalar values along the way. Keys must be strings (v1 runtime).
    fn map_literal(&mut self, e: &Expr, pairs: &[(Expr, Expr)]) -> Result<Temp, ()> {
        let (kty, vty) = match self.ty_of(&e.span) {
            Some(Ty::Map(k, v)) => (k.as_ref().clone(), v.as_ref().clone()),
            _ => return self.bad(e.span, "map literal does not have a `Map` type"),
        };
        let vrep = self.elem_rep(&vty, e.span)?;
        let cap = self.temp();
        self.instr(IrInstr::Const {
            dst: cap,
            c: IrConst::Int(0),
        });
        let map = self.extern_call_t1("pickle_map_new", vec![IrTy::Int], IrTy::Ptr, vec![cap])?;
        for (k, v) in pairs {
            let kt = self.expr(k)?;
            let k_ir = self.irty(k.span)?;
            if kty != Ty::String || !matches!(k_ir, IrTy::Str) {
                return self.bad(k.span, "map keys must be `string` values");
            }
            let vt = self.expr(v)?;
            let boxed = match vrep {
                ElemRep::Scalar(box_sym, _, _) => {
                    let v_ty = self.irty(v.span)?;
                    self.extern_call_t1(box_sym, vec![v_ty], IrTy::Ptr, vec![vt])?
                }
                ElemRep::Ptr => vt,
            };
            self.extern_call_void(
                "pickle_map_set",
                vec![IrTy::Ptr, IrTy::Str, IrTy::Ptr],
                vec![map, kt, boxed],
            );
        }
        Ok(map)
    }

    /// Box a scalar so it can be stored in a List/Map, or pass a pointer type
    /// through untouched.
    fn box_for_store(
        &mut self,
        rep: &ElemRep,
        v: Temp,
        v_ty: IrTy,
    ) -> Result<Temp, ()> {
        match rep {
            ElemRep::Scalar(box_sym, _, _) => {
                self.extern_call_t1(box_sym, vec![v_ty], IrTy::Ptr, vec![v])
            }
            ElemRep::Ptr => Ok(v),
        }
    }

    /// Lift a scalar value to the managed-pointer representation used for
    /// `Option<T>` (and for any pointer-typed boundary). A `none` or an
    /// already-managed value passes through untouched; scalars are boxed.
    fn option_wrap(&mut self, v: Temp, ty: &Ty, span: Span) -> Result<Temp, ()> {
        match ty {
            Ty::Int => self.extern_call_t1("pickle_box_i64", vec![IrTy::Int], IrTy::Ptr, vec![v]),
            Ty::Float => {
                self.extern_call_t1("pickle_box_f64", vec![IrTy::Float], IrTy::Ptr, vec![v])
            }
            Ty::Bool => {
                self.extern_call_t1("pickle_box_bool", vec![IrTy::Bool], IrTy::Ptr, vec![v])
            }
            Ty::Char => {
                self.extern_call_t1("pickle_box_char", vec![IrTy::Char], IrTy::Ptr, vec![v])
            }
            _ => {
                let _ = span;
                Ok(v)
            }
        }
    }

    /// Mirror of `box_for_store` for boundaries that are not list/map element
    /// slots: when the destination representation is `Ptr`, a scalar source
    /// (e.g. `let m: int? = 5`) must be lifted via `option_wrap` first.
    fn pack_for_pointer_boundary(
        &mut self,
        rep: &ElemRep,
        v: Temp,
        vt: &Ty,
        span: Span,
    ) -> Result<Temp, ()> {
        if matches!(rep, ElemRep::Ptr) {
            self.option_wrap(v, vt, span)
        } else {
            Ok(v)
        }
    }

    /// The zero value of a scalar type, as an IR temp.
    fn zero_scalar(&mut self, ir: IrTy, span: Span) -> Result<Temp, ()> {
        let dst = self.temp();
        let c = match ir {
            IrTy::Int => IrConst::Int(0),
            IrTy::Float => IrConst::Float(0.0f64.to_bits()),
            IrTy::Bool => IrConst::Bool(false),
            IrTy::Char => IrConst::Char(0),
            other => return self.bad(span, format!("no zero value for `{other:?}`")),
        };
        self.instr(IrInstr::Const { dst, c });
        Ok(dst)
    }

    /// A null pointer constant (`IrConst::Null`).
    fn null_temp(&mut self) -> Result<Temp, ()> {
        let dst = self.temp();
        self.instr(IrInstr::Const {
            dst,
            c: IrConst::Null,
        });
        Ok(dst)
    }

    /// Lower `match (scrutinee) { case p -> body ... }` into a chain of
    /// `tag == variant` checks. The scrutinee is kept in a managed slot across
    /// the chain; payload bindings extract and unbox each field like list
    /// elements. A chain that reaches its end without a catch-all arm calls
    /// `pickle_panic_no_match`.
    fn match_expr(
        &mut self,
        e: &Expr,
        scrutinee: &Expr,
        arms: &[MatchArm],
    ) -> Result<Temp, ()> {
        let st = self.ty_of(&scrutinee.span);
        let Some(st) = st else {
            return self.bad(e.span, "matching over non-enum values is not lowered yet");
        };
        let Ty::Enum(en_name, _) = &st else {
            return self.bad(e.span, "matching over non-enum values is not lowered yet");
        };
        let enum_name = en_name.clone();
        let table = match self.resolved.types.get(&enum_name) {
            Some(TypeTableEntry::Enum(t)) => t.clone(),
            _ => return self.bad(e.span, format!("unknown enum type `{enum_name}`")),
        };

        // The scrutinee is a managed pointer; keep it in a slot so the
        // collector retains it across any allocation the arms perform.
        let s = self.expr(scrutinee)?;
        let s_slot = self.new_slot(IrTy::Ptr);
        self.instr(IrInstr::StoreSlot { slot: s_slot, v: s });

        let res_ty = self.irty(e.span).ok();
        let res_slot = if !matches!(res_ty, Some(IrTy::Unit) | None) {
            Some(self.new_slot(res_ty.unwrap()))
        } else {
            None
        };

        let sv = self.load(s_slot);
        let tag = self.extern_call_t1(
            "pickle_enum_tag",
            vec![IrTy::Ptr],
            IrTy::Int,
            vec![sv],
        )?;
        let join = self.new_block();

        let mut cur = self.cur;
        let mut chain_live = true;
        for arm in arms {
            let body = self.new_block();
            let guard = arm.guard.as_ref();
            // With a guard, bindings and the guard condition are evaluated in
            // a `guard_in` block that the tag check branches into; on a false
            // guard the chain falls through to the next arm (or the
            // non-exhaustive panic when nothing else remains).
            let guard_in = if guard.is_some() {
                self.new_block()
            } else {
                body
            };
            match &arm.pattern {
                Pattern::Variant { path, payloads } => {
                    let vname = match path.len() {
                        2 => &path[1],
                        1 => &path[0],
                        _ => return self.bad(arm.span, "invalid enum variant pattern"),
                    };
                    let Some(vi) = table.variants.iter().position(|(n, ..)| n == vname) else {
                        return self.bad(
                            arm.span,
                            format!("enum `{enum_name}` has no variant `{vname}`"),
                        );
                    };
                    if chain_live {
                        self.cur = cur;
                        let vi_t = self.temp();
                        self.instr(IrInstr::Const {
                            dst: vi_t,
                            c: IrConst::Int(vi as i64),
                        });
                        let c = self.temp();
                        self.instr(IrInstr::BinOp {
                            dst: c,
                            op: IrBinOp::Eq,
                            a: tag,
                            b: vi_t,
                        });
                        let next = self.new_block();
                        self.term(IrTerm::BranchIf {
                            cond: c,
                            then: guard_in,
                            else_: next,
                        });
                        cur = next;
                    }
                    self.cur = guard_in;
                    self.push_scope();
                    self.bind_enum_payloads(arm.span, &table, vi, payloads, s_slot)?;
                    if let Some(g) = guard {
                        let cond = self.expr(g)?;
                        let fall = if chain_live { cur } else { join };
                        self.term(IrTerm::BranchIf {
                            cond,
                            then: body,
                            else_: fall,
                        });
                        self.cur = body;
                    }
                    self.match_arm_body(&arm.body, res_slot)?;
                    self.pop_scope();
                    self.term(IrTerm::Branch { target: join });
                }
                Pattern::Wildcard | Pattern::Binding { .. } => {
                    if let Some(g) = guard {
                        if chain_live {
                            // A failing guard must keep checking the remaining
                            // arms, so the chain stays live through a fresh
                            // fall-through block.
                            self.cur = cur;
                            self.term(IrTerm::Branch { target: guard_in });
                            let fall = self.new_block();
                            cur = fall;
                            self.cur = guard_in;
                            self.push_scope();
                            if let Pattern::Binding { name, .. } = &arm.pattern {
                                let slot = self.new_slot(IrTy::Ptr);
                                let sv = self.load(s_slot);
                                self.instr(IrInstr::StoreSlot { slot, v: sv });
                                self.declare(name, slot);
                            }
                            let cond = self.expr(g)?;
                            self.term(IrTerm::BranchIf {
                                cond,
                                then: body,
                                else_: fall,
                            });
                            self.cur = body;
                            self.match_arm_body(&arm.body, res_slot)?;
                            self.pop_scope();
                            self.term(IrTerm::Branch { target: join });
                        } else {
                            // Dead guarded catch-all after an unguarded one;
                            // still emit a terminated guard+body for the graph.
                            self.cur = guard_in;
                            self.push_scope();
                            if let Pattern::Binding { name, .. } = &arm.pattern {
                                let slot = self.new_slot(IrTy::Ptr);
                                let sv = self.load(s_slot);
                                self.instr(IrInstr::StoreSlot { slot, v: sv });
                                self.declare(name, slot);
                            }
                            let cond = self.expr(g)?;
                            self.term(IrTerm::BranchIf {
                                cond,
                                then: body,
                                else_: join,
                            });
                            self.cur = body;
                            self.match_arm_body(&arm.body, res_slot)?;
                            self.pop_scope();
                            self.term(IrTerm::Branch { target: join });
                        }
                        continue;
                    }
                    if chain_live {
                        self.cur = cur;
                        self.term(IrTerm::Branch { target: body });
                        chain_live = false;
                    }
                    self.cur = body;
                    self.push_scope();
                    if let Pattern::Binding { name, .. } = &arm.pattern {
                        let slot = self.new_slot(IrTy::Ptr);
                        let sv = self.load(s_slot);
                        self.instr(IrInstr::StoreSlot { slot, v: sv });
                        self.declare(name, slot);
                    }
                    self.match_arm_body(&arm.body, res_slot)?;
                    self.pop_scope();
                    self.term(IrTerm::Branch { target: join });
                }
                Pattern::Literal(_) | Pattern::Tuple(_) | Pattern::Or(_) => {
                    return self.bad(arm.span, "this match pattern is not lowered yet");
                }
            }
        }
        // An enum match whose chain checked every arm and still fell through
        // is non-exhaustive: the checker does not require exhaustiveness, so
        // this must fail loudly at runtime rather than read garbage.
        if chain_live {
            self.cur = cur;
            self.extern_call_void("pickle_panic_no_match", vec![], vec![]);
            self.term(IrTerm::Branch { target: join });
        }
        self.cur = join;
        match res_slot {
            Some(slot) => Ok(self.load(slot)),
            None => Ok(self.unit_temp()),
        }
    }

    /// Emit the body expression of a match arm, storing its value into
    /// `res_slot` when the match produces a value.
    fn match_arm_body(&mut self, body: &Expr, res_slot: Option<Slot>) -> Result<(), ()> {
        let bt = self.expr(body)?;
        if let Some(slot) = res_slot {
            self.instr(IrInstr::StoreSlot { slot, v: bt });
        }
        Ok(())
    }

    /// Bind the payload fields of variant `vi` of `table` into fresh slots for
    /// `Color.Rgb(r, g, b)` patterns. Fields are boxed in the object, so each
    /// is unboxed to its element IR type; `_` payloads bind nothing.
    fn bind_enum_payloads(
        &mut self,
        span: Span,
        table: &EnumTable,
        vi: usize,
        payloads: &[Pattern],
        s_slot: Slot,
    ) -> Result<(), ()> {
        let ftypes = table.variants[vi].1.clone();
        for (pi, p) in payloads.iter().enumerate() {
            match p {
                Pattern::Wildcard => {}
                Pattern::Binding { name, .. } => {
                    let ft = ftypes.get(pi).cloned().unwrap_or(Ty::Unknown);
                    let rep = self.elem_rep(&ft, span)?;
                    let idx = self.temp();
                    self.instr(IrInstr::Const {
                        dst: idx,
                        c: IrConst::Int(pi as i64),
                    });
                    let raw = {
                        let sv = self.load(s_slot);
                        self.extern_call_t1(
                            "pickle_enum_field",
                            vec![IrTy::Ptr, IrTy::Int],
                            IrTy::Ptr,
                            vec![sv, idx],
                        )?
                    };
                    let (v, ir) = match &rep {
                        ElemRep::Ptr => (raw, IrTy::Ptr),
                        ElemRep::Scalar(_, unbox_sym, unbox_ir) => (
                            self.extern_call_t1(unbox_sym, vec![IrTy::Ptr], *unbox_ir, vec![raw])?,
                            *unbox_ir,
                        ),
                    };
                    let slot = self.new_slot(ir);
                    self.instr(IrInstr::StoreSlot { slot, v });
                    self.declare(name, slot);
                }
                _ => {
                    return self.bad(
                        span,
                        "enum payload patterns bind only names or `_`; nested patterns are not lowered yet",
                    )
                }
            }
        }
        Ok(())
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
        if let ExprKind::Member { object, name } = &callee.kind {
            // `Enum.Variant(...)` constructor call.
            if let ExprKind::Ident(enum_name) = &object.kind {
                if let Some(TypeTableEntry::Enum(t)) = self.resolved.types.get(enum_name) {
                    return self.enum_ctor(e, t, name, args);
                }
            }
            return self.method_call(e, object, name, args);
        }
        let ExprKind::Ident(name) = &callee.kind else {
            return self.bad(e.span, "only plain function calls are lowered yet");
        };
        // User function?
        if let Some(&fid) = self.module.funcs_by_name.get(name) {
            let fparams = self.module.funcs[fid.0].params.clone();
            let mut arg_temps = Vec::new();
            for (i, a) in args.iter().enumerate() {
                if a.spread {
                    return self.bad(a.span, "spread arguments are not lowered yet");
                }
                let t = self.expr(&a.value)?;
                let t = if matches!(fparams.get(i).map(|p| p.ty), Some(IrTy::Ptr)) {
                    let vt = self.ty_of(&a.value.span).unwrap_or(Ty::Unknown);
                    self.option_wrap(t, &vt, a.value.span)?
                } else {
                    t
                };
                arg_temps.push(t);
            }
            let dst = self.temp();
            self.instr(IrInstr::Call {
                dst: Some(dst),
                callee: Callee::Func(fid),
                args: arg_temps,
            });
            return Ok(dst);
        }
        // Class/struct constructor call: `TypeName(arg...)`.
        if let Some(&cid) = self.class_by_name.get(name) {
            let Some(&fid) = self.ctor_ids.get(&cid) else {
                return self.bad(e.span, format!("`{name}` has no constructor"));
            };
            let fparams = self.module.funcs[fid.0].params.clone();
            let mut arg_temps = Vec::new();
            for (i, a) in args.iter().enumerate() {
                if a.spread {
                    return self.bad(a.span, "spread arguments are not lowered yet");
                }
                let t = self.expr(&a.value)?;
                let t = if matches!(fparams.get(i).map(|p| p.ty), Some(IrTy::Ptr)) {
                    let vt = self.ty_of(&a.value.span).unwrap_or(Ty::Unknown);
                    self.option_wrap(t, &vt, a.value.span)?
                } else {
                    t
                };
                arg_temps.push(t);
            }
            let dst = self.temp();
            self.instr(IrInstr::Call {
                dst: Some(dst),
                callee: Callee::Func(fid),
                args: arg_temps,
            });
            return Ok(dst);
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
                        Some(Ty::List(_))
                        | Some(Ty::Map(_, _))
                        | Some(Ty::Enum(..))
                        | Some(Ty::Class(..))
                        | Some(Ty::Struct(..)) => {
                            "pickle_print_obj"
                        }
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
                        Some(Ty::String)
                        | Some(Ty::List(_))
                        | Some(Ty::Map(_, _))
                        | Some(Ty::Enum(..))
                        | Some(Ty::Class(..))
                        | Some(Ty::Struct(..)) => IrTy::Ptr,
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
                    Some(Ty::List(_)) => {
                        self.extern_call_t1("pickle_list_len", vec![IrTy::Ptr], IrTy::Int, vec![t])
                    }
                    Some(Ty::Map(_, _)) => {
                        self.extern_call_t1("pickle_map_len", vec![IrTy::Ptr], IrTy::Int, vec![t])
                    }
                    _ => self.bad(e.span, "`len` over this type is not lowered yet"),
                }
            }
            _ => self.bad(e.span, format!("`{name}` is not lowered yet")),
        }
    }

    /// Build an enum value via `Enum.Variant(arg...)`. Payload scalars are
    /// boxed exactly like list elements; the value is a managed object whose
    /// tag is the variant index and whose fields are the boxed args.
    fn enum_ctor(
        &mut self,
        e: &Expr,
        table: &EnumTable,
        variant_name: &str,
        args: &[CallArg],
    ) -> Result<Temp, ()> {
        if args.iter().any(|a| a.spread) {
            return self.bad(e.span, "spread arguments are not lowered yet");
        }
        let Some((vi, ftypes)) = table
            .variants
            .iter()
            .enumerate()
            .find(|(_, (n, ..))| n == variant_name)
            .map(|(i, (_, ft, _))| (i as i64, ft.clone()))
        else {
            return self.bad(
                e.span,
                format!("enum `{}` has no variant `{variant_name}`", table.name),
            );
        };
        let n = ftypes.len();
        if args.len() != n {
            return self.bad(
                e.span,
                format!(
                    "`{variant_name}` takes {n} argument(s), got {}",
                    args.len()
                ),
            );
        }
        let tag = self.temp();
        self.instr(IrInstr::Const {
            dst: tag,
            c: IrConst::Int(vi),
        });
        let count = self.temp();
        self.instr(IrInstr::Const {
            dst: count,
            c: IrConst::Int(n as i64),
        });
        let obj = self.extern_call_t1(
            "pickle_enum_new",
            vec![IrTy::Int, IrTy::Int],
            IrTy::Ptr,
            vec![tag, count],
        )?;
        for (i, a) in args.iter().enumerate() {
            let ft = &ftypes[i];
            let rep = self.elem_rep(ft, e.span)?;
            let v = self.expr(&a.value)?;
            let vt = self.ty_of(&a.value.span).unwrap_or(Ty::Unknown);
            let packed = self.pack_for_pointer_boundary(&rep, v, &vt, e.span)?;
            let boxed = self.box_for_store(&rep, packed, elem_ir(ft))?;
            let idx = self.temp();
            self.instr(IrInstr::Const {
                dst: idx,
                c: IrConst::Int(i as i64),
            });
            self.extern_call_void(
                "pickle_enum_set_field",
                vec![IrTy::Ptr, IrTy::Int, IrTy::Ptr],
                vec![obj, idx, boxed],
            );
        }
        Ok(obj)
    }

    /// A bare `Enum.Variant(...)` access with no parentheses. Unit variants
    /// (no payload) lower to a tag-only enum via `pickle_enum_new`.
    fn member_value(&mut self, e: &Expr, object: &Expr, name: &str) -> Result<Temp, ()> {
        if let ExprKind::Ident(enum_name) = &object.kind {
            if let Some(TypeTableEntry::Enum(t)) = self.resolved.types.get(enum_name) {
                let Some((vi, fields)) = t
                    .variants
                    .iter()
                    .enumerate()
                    .find(|(_, (n, _, _))| n == name)
                    .map(|(i, (_, ft, _))| (i as i64, ft))
                else {
                    return self.bad(
                        e.span,
                        format!("enum `{}` has no variant `{name}`", t.name),
                    );
                };
                if !fields.is_empty() {
                    return self.bad(
                        e.span,
                        format!(
                            "variant `{name}` carries {} payload field(s) and needs `{name}(...)`",
                            fields.len()
                        ),
                    );
                }
                let tag = self.temp();
                self.instr(IrInstr::Const {
                    dst: tag,
                    c: IrConst::Int(vi),
                });
                let zero = self.temp();
                self.instr(IrInstr::Const {
                    dst: zero,
                    c: IrConst::Int(0),
                });
                return self.extern_call_t1(
                    "pickle_enum_new",
                    vec![IrTy::Int, IrTy::Int],
                    IrTy::Ptr,
                    vec![tag, zero],
                );
            }
        }
        // `object.field` on a class/struct instance.
        let ot = self.ty_of(&object.span);
        let cid = match ot {
            Some(Ty::Class(cn, _)) | Some(Ty::Struct(cn, _)) => {
                self.class_by_name.get(&cn).copied()
            }
            _ => None,
        };
        if let Some(cid) = cid {
            if let Some(slot) = self.instance_field_index(cid as i64, name) {
                let field_ty = self.field_at(cid as i64, slot).ty.clone();
                let obj = self.expr(object)?;
                return self.field_read(e.span, obj, &field_ty, slot);
            }
            if let Some(&fid) = self.property_ids.get(&(cid, name.to_string(), false)) {
                let obj = self.expr(object)?;
                return self.call_method(e, fid, &[], Some(obj));
            }
            return self.bad(
                e.span,
                format!("method `{name}` of `{}` cannot be used as a value", self.class_name_of(cid)),
            );
        }
        // `Type.staticProperty` reads a receiver-less accessor; `Type.field`
        // reads a static field cell.
        if let ExprKind::Ident(tname) = &object.kind {
            if let Some(&cid) = self.class_by_name.get(tname) {
                if let Some(&fid) = self.static_property_ids.get(&(cid, name.to_string(), false)) {
                    return self.call_method(e, fid, &[], None);
                }
                if let Some((slot, info)) = self.static_field(cid as i64, name) {
                    return self.static_read(e.span, cid, slot, &info.ty);
                }
                if let Some(r) = self.read_class_const(e.span, cid as i64, name) {
                    return r;
                }
                return self.bad(
                    e.span,
                    format!(
                        "static member `{name}` on `{tname}` is not lowered yet"
                    ),
                );
            }
        }
        self.bad(e.span, "member access is not lowered yet")
    }

    /// The runtime slot of instance field `name` on class `cid`, following the
    /// parent-first layout, when registered.
    fn instance_field_index(&self, cid: i64, name: &str) -> Option<usize> {
        let plan = self.classes.iter().find(|p| p.class_id as i64 == cid)?;
        self.abs_instance_field_slot(&plan.name, name)
    }

    /// The static slot of static field `name` on class `cid`, with its declared
    /// type, when the class is registered and the field exists.
    fn static_field(&self, cid: i64, name: &str) -> Option<(usize, FieldInfo)> {
        let plan = self.classes.iter().find(|p| p.class_id as i64 == cid)?;
        let slot = static_field_slot(&plan.table, name)?;
        let info = static_field_info(&plan.table, name)?;
        Some((slot, info))
    }

    /// Inline a class constant's initializer at a use site. Returns `None` when
    /// `name` is not a constant of class `cid`; `Some(Err)` on a cyclic constant.
    fn read_class_const(
        &mut self,
        span: Span,
        cid: i64,
        name: &str,
    ) -> Option<Result<Temp, ()>> {
        let (_, value) = self
            .class_consts
            .get(&(cid as u32, name.to_string()))?
            .clone();
        let key = format!("{}.{}", self.class_name_of(cid as u32), name);
        if self.const_inlining.iter().any(|n| n == &key) {
            return Some(
                self.bad(span, format!("cyclic `const` initialization of `{key}`")),
            );
        }
        self.const_inlining.push(key);
        // Constant initializers are evaluated in the declaring class's scope so
        // a constant can reference another constant of the same class by name.
        let saved_owner = self.owner;
        self.owner = Some(cid);
        let t = self.expr(value);
        self.owner = saved_owner;
        self.const_inlining.pop();
        Some(t)
    }

    fn field_at(&self, cid: i64, idx: usize) -> FieldInfo {
        let plan = self
            .classes
            .iter()
            .find(|p| p.class_id as i64 == cid)
            .unwrap_or_else(|| unreachable!("field index on an unregistered class"));
        self.all_instance_fields(&plan.name)
            .into_iter()
            .nth(idx)
            .unwrap_or_else(|| unreachable!("field index out of range"))
    }

    fn class_name_of(&self, cid: u32) -> String {
        self.classes
            .iter()
            .find(|p| p.class_id == cid)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| format!("class#{cid}"))
    }

    /// `obj.slot` field read: `pickle_obj_slot_get` then unbox scalars.
    fn field_read(&mut self, span: Span, obj: Temp, field_ty: &Ty, slot: usize) -> Result<Temp, ()> {
        let rep = self.elem_rep(field_ty, span)?;
        let idx = self.int_const(slot as i64);
        let raw = self.extern_call_t1(
            "pickle_obj_slot_get",
            vec![IrTy::Ptr, IrTy::Int],
            IrTy::Ptr,
            vec![obj, idx],
        )?;
        match rep {
            ElemRep::Scalar(_, unbox, ir) => {
                self.extern_call_t1(unbox, vec![IrTy::Ptr], ir, vec![raw])
            }
            ElemRep::Ptr => Ok(raw),
        }
    }

    /// `xs.push(v)` and `xs.pop()` for `List<T>` receivers.
    fn method_call(
        &mut self,
        e: &Expr,
        object: &Expr,
        name: &str,
        args: &[CallArg],
    ) -> Result<Temp, ()> {
        if args.iter().any(|a| a.spread) {
            return self.bad(e.span, "spread arguments are not lowered yet");
        }
        let ot = self.ty_of(&object.span);

        // `Type.staticMethod(...)`.
        if let ExprKind::Ident(tname) = &object.kind {
            if let Some(&cid) = self.class_by_name.get(tname) {
                if let Some(&fid) = self.named_ctor_ids.get(&(cid, name.to_string())) {
                    return self.call_method(e, fid, args, None);
                }
                if let Some(&(fid, is_static)) = self.method_ids.get(&(cid, name.to_string())) {
                    if !is_static {
                        return self.bad(
                            e.span,
                            format!("instance method `{name}` must be called on an instance of `{tname}`"),
                        );
                    }
                    return self.call_method(e, fid, args, None);
                }
                return self.bad(
                    e.span,
                    format!("`{tname}` has no static method `{name}`"),
                );
            }
        }

        // `instance.method(...)` on a class/struct instance.
        if let Some(cid) = match ot {
            Some(Ty::Class(ref cn, _)) | Some(Ty::Struct(ref cn, _)) => {
                self.class_by_name.get(cn).copied()
            }
            _ => None,
        } {
            if let Some(&(fid, is_static)) = self.method_ids.get(&(cid, name.to_string())) {
                if is_static {
                    return self.bad(
                        e.span,
                        format!("static method `{name}` must be called on the type, not an instance"),
                    );
                }
                let receiver = self.expr(object)?;
                return self.call_method(e, fid, args, Some(receiver));
            }
            return self.bad(
                e.span,
                format!("`{}` has no method `{name}`", self.class_name_of(cid)),
            );
        }

        match ot {
            Some(Ty::Map(k, v)) => {
                if k.as_ref() != &Ty::String {
                    return self.bad(object.span, "map keys must be `string` values");
                }
                let obj = self.expr(object)?;
                let vrep = self.elem_rep(&v, e.span)?;
                match name {
                    "has" => {
                        if args.len() != 1 {
                            return self.bad(e.span, "`has` takes one argument");
                        }
                        let kt = self.expr(&args[0].value)?;
                        if !matches!(self.irty(args[0].value.span)?, IrTy::Str) {
                            return self.bad(args[0].value.span, "map keys must be `string` values");
                        }
                        self.extern_call_t1(
                            "pickle_map_has",
                            vec![IrTy::Ptr, IrTy::Str],
                            IrTy::Bool,
                            vec![obj, kt],
                        )
                    }
                    "keys" => {
                        if !args.is_empty() {
                            return self.bad(e.span, "`keys` takes no arguments");
                        }
                        self.extern_call_t1("pickle_map_keys", vec![IrTy::Ptr], IrTy::Ptr, vec![obj])
                    }
                    "values" => {
                        if !args.is_empty() {
                            return self.bad(e.span, "`values` takes no arguments");
                        }
                        self.extern_call_t1(
                            "pickle_map_values",
                            vec![IrTy::Ptr],
                            IrTy::Ptr,
                            vec![obj],
                        )
                    }
                    other => {
                        let _ = vrep;
                        self.bad(
                            e.span,
                            format!("`{other}` method on `Map` is not lowered yet"),
                        )
                    }
                }
            }
            Some(Ty::List(_)) => self.list_method_call(e, object, name, args),
            _ => self.bad(e.span, "method calls on this type are not lowered yet"),
        }
    }

    /// Emit a call to a class method function, passing the receiver first for
    /// instance methods.
    fn call_method(
        &mut self,
        _e: &Expr,
        fid: FuncId,
        args: &[CallArg],
        receiver: Option<Temp>,
    ) -> Result<Temp, ()> {
        let fparams = self.module.funcs[fid.0].params.clone();
        let mut call_args = match receiver {
            Some(r) => vec![r],
            None => Vec::new(),
        };
        let base = call_args.len();
        for (i, a) in args.iter().enumerate() {
            let t = self.expr(&a.value)?;
            let t = if matches!(fparams.get(base + i).map(|p| p.ty), Some(IrTy::Ptr)) {
                let vt = self.ty_of(&a.value.span).unwrap_or(Ty::Unknown);
                self.option_wrap(t, &vt, a.value.span)?
            } else {
                t
            };
            call_args.push(t);
        }
        let dst = self.temp();
        self.instr(IrInstr::Call {
            dst: Some(dst),
            callee: Callee::Func(fid),
            args: call_args,
        });
        Ok(dst)
    }

    /// `xs.push(v)` and `xs.pop()` for `List<T>` receivers.
    fn list_method_call(
        &mut self,
        e: &Expr,
        object: &Expr,
        name: &str,
        args: &[CallArg],
    ) -> Result<Temp, ()> {
        let ot = self.ty_of(&object.span);
        let elem = match &ot {
            Some(Ty::List(inner)) => inner.as_ref().clone(),
            _ => return self.bad(e.span, "receiver is not a `List`"),
        };
        let obj = self.expr(object)?;
        match name {
            "push" => {
                if args.len() != 1 {
                    return self.bad(e.span, "`push` takes one argument");
                }
                let rep = self.elem_rep(&elem, e.span)?;
                let v = self.expr(&args[0].value)?;
                let push_v = match rep {
                    ElemRep::Scalar(box_sym, _, _) => {
                        let vt = self.irty(args[0].value.span)?;
                        self.extern_call_t1(box_sym, vec![vt], IrTy::Ptr, vec![v])?
                    }
                    ElemRep::Ptr => v,
                };
                self.extern_call_void(
                    "pickle_list_push",
                    vec![IrTy::Ptr, IrTy::Ptr],
                    vec![obj, push_v],
                );
                Ok(self.unit_temp())
            }
            "pop" => {
                if !args.is_empty() {
                    return self.bad(e.span, "`pop` takes no arguments");
                }
                let rep = self.elem_rep(&elem, e.span)?;
                let raw = self.extern_call_t1("pickle_list_pop", vec![IrTy::Ptr], IrTy::Ptr, vec![obj])?;
                match rep {
                    ElemRep::Scalar(_, unbox_sym, ir) => {
                        self.extern_call_t1(unbox_sym, vec![IrTy::Ptr], ir, vec![raw])
                    }
                    ElemRep::Ptr => Ok(raw),
                }
            }
            other => {
                self.bad(e.span, format!("`{other}` method on `List` is not lowered yet"))
            }
        }
    }

    // ---- primitive ops ----

    /// How a `List<T>` element is represented at the runtime boundary.
    fn elem_rep(&mut self, elem: &Ty, span: Span) -> Result<ElemRep, ()> {
        use ElemRep::*;
        match elem {
            Ty::Int => Ok(Scalar("pickle_box_i64", "pickle_unbox_i64", IrTy::Int)),
            Ty::Float => Ok(Scalar("pickle_box_f64", "pickle_unbox_f64", IrTy::Float)),
            Ty::Bool => Ok(Scalar("pickle_box_bool", "pickle_unbox_bool", IrTy::Bool)),
            Ty::Char => Ok(Scalar("pickle_box_char", "pickle_unbox_char", IrTy::Char)),
            Ty::String
            | Ty::Option(..)
            | Ty::Class(..)
            | Ty::Struct(..)
            | Ty::Enum(..)
            | Ty::Interface(..)
            | Ty::List(..)
            | Ty::Map(..)
            | Ty::Tuple(..)
            | Ty::Range(..) => Ok(Ptr),
            Ty::None | Ty::Empty => self.bad(span, "a list of `none` has no element representation"),
            Ty::Fn(..) => self.bad(span, "function values are not lowered yet"),
            Ty::Unknown => self.bad(span, "list element type is not statically known"),
            Ty::Var(_) => self.bad(span, "generic element types are not lowered yet"),
        }
    }

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