use std::collections::HashMap;

use crate::ast::*;
use crate::diag::{Diagnostic, DiagnosticSink, Span};
use crate::ty::Ty;

/// Signature of a single callable (top-level function or method).
#[derive(Debug, Clone)]
pub struct CallableInfo {
    pub name: String,
    pub span: Span,
    pub visibility: Visibility,
    pub is_async: bool,
    pub is_static: bool,
    pub is_override: bool,
    pub is_abstract: bool,
    pub operator: Option<String>,
    pub generics: Vec<String>,
    pub params: Vec<ParamInfo>,
    pub ret: Ty,
}

#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub name: String,
    pub ty: Ty,
    pub has_default: bool,
    pub rest: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub mutable: bool,
    pub const_: bool,
    /// Owned (`#[manualAlloc]`) instance field: its value is freed recursively
    /// when the owning object is freed.
    pub manual: bool,
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct PropertyInfo {
    pub name: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub ty: Ty,
    pub has_get: bool,
    pub has_set: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CtorInfo {
    pub span: Span,
    pub visibility: Visibility,
    pub params: Vec<ParamInfo>,
}

#[derive(Debug, Clone)]
pub struct ClassTable {
    pub name: String,
    pub span: Span,
    pub visibility: Visibility,
    pub generics: Vec<String>,
    pub extends: Option<Ty>,
    pub implements: Vec<Ty>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<CallableInfo>,
    pub properties: Vec<PropertyInfo>,
    pub ctor: Option<CtorInfo>,
    pub named_ctors: Vec<(String, CtorInfo)>,
    pub consts: Vec<FieldInfo>,
}

#[derive(Debug, Clone)]
pub struct EnumTable {
    pub name: String,
    pub span: Span,
    pub visibility: Visibility,
    pub generics: Vec<String>,
    pub implements: Vec<Ty>,
    pub variants: Vec<(String, Vec<Ty>, Span)>,
}

#[derive(Debug, Clone)]
pub struct InterfaceMemberInfo {
    pub span: Span,
    pub name: String,
    pub is_property: bool,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub struct InterfaceTable {
    pub name: String,
    pub span: Span,
    pub visibility: Visibility,
    pub generics: Vec<String>,
    pub extends: Vec<Ty>,
    pub members: Vec<InterfaceMemberInfo>,
}

#[derive(Debug, Clone)]
pub enum TypeTableEntry {
    Class(ClassTable),
    Struct(ClassTable),
    Enum(EnumTable),
    Interface(InterfaceTable),
}

impl TypeTableEntry {
    pub fn name(&self) -> &str {
        match self {
            TypeTableEntry::Class(t) | TypeTableEntry::Struct(t) => &t.name,
            TypeTableEntry::Interface(t) => &t.name,
            TypeTableEntry::Enum(t) => &t.name,
        }
    }

    pub fn generics(&self) -> &[String] {
        match self {
            TypeTableEntry::Class(t) | TypeTableEntry::Struct(t) => &t.generics,
            TypeTableEntry::Interface(t) => &t.generics,
            TypeTableEntry::Enum(t) => &t.generics,
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            TypeTableEntry::Class(_) => "class",
            TypeTableEntry::Struct(_) => "struct",
            TypeTableEntry::Enum(_) => "enum",
            TypeTableEntry::Interface(_) => "interface",
        }
    }
}

/// The result of resolving and declaring all names in one source file.
pub struct ResolvedProgram {
    pub types: HashMap<String, TypeTableEntry>,
    pub fns: HashMap<String, Vec<CallableInfo>>,
    pub consts: Vec<FieldInfo>,
    pub import_aliases: HashMap<String, Vec<String>>,
    pub module_path: Vec<String>,
}

pub struct Resolver<'a> {
    pub diags: &'a DiagnosticSink,
    types: HashMap<String, TypeTableEntry>,
    fns: HashMap<String, Vec<CallableInfo>>,
    consts: Vec<FieldInfo>,
    import_aliases: HashMap<String, Vec<String>>,
    module_path: Vec<String>,
}

impl<'a> Resolver<'a> {
    pub fn new(diags: &'a DiagnosticSink) -> Resolver<'a> {
        Resolver {
            diags,
            types: HashMap::new(),
            fns: HashMap::new(),
            consts: Vec::new(),
            import_aliases: HashMap::new(),
            module_path: Vec::new(),
        }
    }

    pub fn resolve(mut self, prog: &'a Program) -> ResolvedProgram {
        self.module_path = prog
            .module
            .as_ref()
            .map(|m| m.path.clone())
            .unwrap_or_default();

        self.declare_builtins();

        for imp in &prog.imports {
            if let ImportKind::Module { path, alias } = &imp.kind {
                let alias = alias
                    .clone()
                    .unwrap_or_else(|| path.last().cloned().unwrap_or_default());
                if self.import_aliases.contains_key(&alias) {
                    self.diags.emit(
                        Diagnostic::error_at(
                            imp.span,
                            format!("duplicate import alias `{alias}`"),
                        )
                        .with_code(crate::error::ErrorCode::DuplicateImportAlias),
                    );
                }
                self.import_aliases.insert(alias, path.clone());
            }
        }

        let mut nmap: HashMap<String, Span> = HashMap::new();
        for item in &prog.items {
            let (name, kind) = match &item.kind {
                ItemKind::Fn(f) => (f.name.clone(), None),
                ItemKind::Class(c) => (c.name.clone(), Some(("class", ItemKindDecl::Class(c)))),
                ItemKind::Struct(s) => (s.name.clone(), Some(("struct", ItemKindDecl::Struct(s)))),
                ItemKind::Enum(e) => (e.name.clone(), Some(("enum", ItemKindDecl::Enum(e)))),
                ItemKind::Interface(i) => {
                    (i.name.clone(), Some(("interface", ItemKindDecl::Interface(i))))
                }
                ItemKind::Const(c) => (c.name.clone(), None),
                ItemKind::Test(f) => (f.name.clone(), None),
            };
            if let Some(&first) = nmap.get(&name) {
                self.diags.emit(
                    Diagnostic::error_at(item.span, format!("duplicate declaration `{name}`"))
                        .with_code(crate::error::ErrorCode::DuplicateItem)
                        .note_at(first, "first declared here"),
                );
                continue;
            }
            nmap.insert(name, item.span);

            match &item.kind {
                ItemKind::Fn(f) => self.declare_fn(f, false),
                ItemKind::Test(f) => self.declare_fn(f, true),
                ItemKind::Const(c) => self.declare_const(c),
                _ => {}
            }
            if let Some((_, k)) = kind {
                self.declare_kind(k);
            }
        }
        let _ = prog;

        self.post_checks();

        ResolvedProgram {
            types: std::mem::take(&mut self.types),
            fns: std::mem::take(&mut self.fns),
            consts: std::mem::take(&mut self.consts),
            import_aliases: std::mem::take(&mut self.import_aliases),
            module_path: self.module_path,
        }
    }
}

#[derive(Clone)]
enum ItemKindDecl<'a> {
    Class(&'a ClassDecl),
    Struct(&'a StructDecl),
    Enum(&'a EnumDecl),
    Interface(&'a InterfaceDecl),
}

impl<'a> Resolver<'a> {
    fn declare_kind(&mut self, k: ItemKindDecl<'a>) {
        match k {
            ItemKindDecl::Class(c) => self.declare_class(c),
            ItemKindDecl::Struct(s) => self.declare_struct(s),
            ItemKindDecl::Enum(e) => self.declare_enum(e),
            ItemKindDecl::Interface(i) => self.declare_interface(i),
        }
    }

    fn declare_fn(&mut self, f: &FnDecl, is_test: bool) {
        let generics: Vec<String> = f.generics.iter().map(|g| g.name.clone()).collect();
        let params = self.resolve_params(&f.params, &generics);
        let ret = self.resolve_ret(&f.return_ty, &generics);
        self.fns.entry(f.name.clone()).or_default().push(CallableInfo {
            name: f.name.clone(),
            span: f.span,
            visibility: f.visibility,
            is_async: f.is_async,
            is_static: true,
            is_override: false,
            is_abstract: false,
            operator: None,
            generics,
            params,
            ret,
        });
        let _ = is_test;
    }

    /// Register built-in functions that every module sees without imports.
    /// User declarations with the same name win over these defaults.
    fn declare_builtins(&mut self) {
        let span = crate::diag::Span::new(crate::diag::FileId(0), 0, 0);
        let mk = |name: &str,
                  params: Vec<ParamInfo>,
                  ret: Ty,
                  rest: bool|
         -> CallableInfo {
            CallableInfo {
                name: name.to_string(),
                span,
                visibility: Visibility::Public,
                is_async: false,
                is_static: true,
                is_override: false,
                is_abstract: false,
                operator: None,
                generics: Vec::new(),
                params: params
                    .into_iter()
                    .map(|mut p| {
                        p.rest = rest;
                        p
                    })
                    .collect(),
                ret,
            }
        };
        let any = |name: &str| ParamInfo {
            name: name.to_string(),
            ty: Ty::Unknown,
            has_default: false,
            rest: true,
            span,
        };
        // Variadic `print`/`println` accept any number of args (rest=truthy so
        // the checker treats remaining args as unconstrained).
        let builtins: Vec<CallableInfo> = vec![
            mk("print", vec![any("args")], Ty::Empty, true),
            mk("println", vec![any("args")], Ty::Empty, true),
            mk("len", vec![any("items")], Ty::Int, true),
            mk("abs", vec![any("x")], Ty::Unknown, true),
            mk("min", vec![any("bounds")], Ty::Unknown, true),
            mk("max", vec![any("bounds")], Ty::Unknown, true),
            mk("clamp", vec![any("bounds")], Ty::Unknown, true),
            // `str(x)`: numeric/bool/char value rendered as `string`; the
            // checker matches the runtime's `pickle_str_from_*` family.
            mk("str", vec![any("x")], Ty::String, true),
            // `range(end)`, `range(start, end)`, `range(start, end, step)`:
            // typed as `List<int>` by a dedicated checker rule; the loose
            // declaration just makes the name resolvable and dispatchable.
            mk(
                "range",
                vec![any("bounds")],
                Ty::List(Box::new(Ty::Int)),
                true,
            ),
            // `read_file(path)`: whole-file bytes as `string`, or `none` --
            // checked strictly by a dedicated rule; a loose declaration makes
            // the name resolvable and dispatchable.
            mk(
                "read_file",
                vec![any("path")],
                Ty::Option(Box::new(Ty::String)),
                true,
            ),
            // `write_file(path, text)`: overwrite; `bool` success.
            mk("write_file", vec![any("path"), any("text")], Ty::Bool, true),
            // `file_exists(path)`: `bool`.
            mk("file_exists", vec![any("path")], Ty::Bool, true),
            // `delete(path)`: remove a file; `bool` success.
            mk("delete", vec![any("path")], Ty::Bool, true),
            // `mkdir(path)`: create a directory; `bool` success.
            mk("mkdir", vec![any("path")], Ty::Bool, true),
            // `list_dir(path)`: child paths as `List<string>?`.
            mk(
                "list_dir",
                vec![any("path")],
                Ty::Option(Box::new(Ty::List(Box::new(Ty::String)))),
                true,
            ),
            // `bytes(s)`: snapshot a string's raw bytes as a `List<byte>`
            // (checked strictly by a dedicated rule; `str(xs)` inverts it).
            mk(
                "bytes",
                vec![any("s")],
                Ty::List(Box::new(Ty::Byte)),
                true,
            ),
            // `stream_open_read/write/append(path)`: open a buffered file
            // stream as `Stream?` — checked strictly by dedicated rules; the
            // loose declaration makes the name resolvable and dispatchable.
            mk(
                "stream_open_read",
                vec![any("path")],
                Ty::Option(Box::new(Ty::Stream)),
                true,
            ),
            mk(
                "stream_open_write",
                vec![any("path")],
                Ty::Option(Box::new(Ty::Stream)),
                true,
            ),
            mk(
                "stream_open_append",
                vec![any("path")],
                Ty::Option(Box::new(Ty::Stream)),
                true,
            ),
            // `stdout_stream()` / `stderr_stream()`: write-only console
            // streams; checked strictly by dedicated rules.
            mk("stdout_stream", vec![], Ty::Stream, true),
            mk("stderr_stream", vec![], Ty::Stream, true),
            // `assert(cond, msg?)` is checked strictly (`bool`, optional
            // `string`) by a dedicated rule; the loose declaration just makes
            // the name resolvable and dispatchable.
            mk("assert", vec![any("args")], Ty::Empty, true),
        ];
        for c in builtins {
            self.fns.entry(c.name.clone()).or_default().push(c);
        }
    }

    fn declare_const(&mut self, c: &ConstDecl) {
        self.consts.push(FieldInfo {
            name: c.name.clone(),
            visibility: c.visibility,
            is_static: true,
            mutable: false,
            const_: true,
            manual: false,
            ty: c
                .ty
                .as_ref()
                .map(|t| self.resolve_ty(t, &[]))
                .unwrap_or(Ty::Unknown),
            span: c.span,
        });
    }
}

fn render_path(path: &[String]) -> String {
    path.join(".")
}

fn render_list(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

impl<'a> Resolver<'a> {
    // ---- type expression resolution ---------------------------------------

    /// Resolve a type expression against this resolver's declared tables.
    pub fn resolve_ty(&self, te: &TypeExpr, generics: &[String]) -> Ty {
        self.type_ctx().resolve_ty(te, generics)
    }

    fn type_ctx(&self) -> TypeCtx<'_> {
        TypeCtx {
            diags: self.diags,
            types: &self.types,
            import_aliases: &self.import_aliases,
        }
    }

    fn resolve_param(&self, p: &Param, generics: &[String]) -> ParamInfo {
        let ctx = self.type_ctx();
        ParamInfo {
            name: p.name.clone(),
            ty: p
                .ty
                .as_ref()
                .map(|t| ctx.resolve_ty(t, generics))
                .unwrap_or(Ty::Unknown),
            has_default: p.default.is_some(),
            rest: p.rest,
            span: p.span,
        }
    }

    fn resolve_params(&self, params: &[Param], generics: &[String]) -> Vec<ParamInfo> {
        params
            .iter()
            .map(|p| self.resolve_param(p, generics))
            .collect()
    }

    fn resolve_ret(&self, rt: &Option<TypeExpr>, generics: &[String]) -> Ty {
        let ctx = self.type_ctx();
        match rt {
            Some(t) => ctx.resolve_ty(t, generics),
            None => Ty::Empty,
        }
    }
}

/// Context for resolving `TypeExpr`s against already-collected tables.
/// Shared by `Resolver` (during name resolution) and `ResolvedProgram`
/// (during type-checking, e.g. resolving bodies' param types).
pub struct TypeCtx<'a> {
    pub diags: &'a DiagnosticSink,
    pub types: &'a HashMap<String, TypeTableEntry>,
    pub import_aliases: &'a HashMap<String, Vec<String>>,
}

impl<'a> TypeCtx<'a> {
    pub fn resolve_ty(&self, te: &TypeExpr, generics: &[String]) -> Ty {
        match &te.kind {
            TypeExprKind::Path(p) => self.resolve_path(p, generics, te.span),
            TypeExprKind::Generic(base, args) => {
                let bare = self.resolve_ty(base, generics);
                let arg_tys: Vec<Ty> = args.iter().map(|a| self.resolve_ty(a, generics)).collect();
                self.instantiate(&bare, &arg_tys, te.span)
            }
            TypeExprKind::Option(inner) => self.resolve_ty(inner, generics).opt_of(),
            TypeExprKind::Tuple(items) => {
                if items.len() == 1 {
                    return self.resolve_ty(&items[0], generics);
                }
                Ty::Tuple(items.iter().map(|t| self.resolve_ty(t, generics)).collect())
            }
            TypeExprKind::Fn {
                params,
                ret,
                is_async,
            } => {
                let ps = params.iter().map(|t| self.resolve_ty(t, generics)).collect();
                let r = match ret {
                    Some(r) => self.resolve_ty(r, generics),
                    None => Ty::Empty,
                };
                let _ = is_async;
                Ty::Fn(ps, Box::new(r))
            }
            TypeExprKind::Pointer(inner) => Ty::Ptr(Box::new(self.resolve_ty(inner, generics))),
            TypeExprKind::Ref(inner) => Ty::Ref(Box::new(self.resolve_ty(inner, generics))),
            TypeExprKind::Infer => Ty::Unknown,
        }
    }

    fn resolve_path(&self, path: &[String], generics: &[String], span: Span) -> Ty {
        if path.len() == 1 {
            if path[0] == "List" {
                // Builtin list type; elements resolved by `instantiate`.
                return Ty::List(Box::new(Ty::Unknown));
            }
            if path[0] == "Map" {
                // Builtin map type; key/value resolved by `instantiate`.
                return Ty::Map(Box::new(Ty::Unknown), Box::new(Ty::Unknown));
            }
            if path[0] == "Stream" {
                // Builtin stream type (open buffered file/console handle).
                return Ty::Stream;
            }
            if let Some(g) = generics.iter().find(|g| **g == path[0]) {
                return Ty::Var(g.clone());
            }
            if let Some(t) = self.primitive(&path[0]) {
                return t;
            }
        }
        if path.len() == 1 {
            if let Some(entry) = self.types.get(&path[0]) {
                return self.bare_type(entry);
            }
        } else if self.import_aliases.contains_key(&path[0]) {
            if let Some(entry) = self.types.get(&path[path.len() - 1]) {
                return self.bare_type(entry);
            }
        }
        self.diags.emit(
            Diagnostic::error_at(span, format!("unknown type `{}`", render_path(path)))
                .with_code(crate::error::ErrorCode::UnknownType)
                .note("expected a class, struct, enum, interface, or built-in type"),
        );
        Ty::Unknown
    }

    fn primitive(&self, name: &str) -> Option<Ty> {
        let t = match name {
            "bool" => Ty::Bool,
            "char" => Ty::Char,
            "byte" => Ty::Byte,
            "int" | "i64" => Ty::Int,
            "float" | "f64" => Ty::Float,
            "string" => Ty::String,
            "void" => Ty::Empty,
            _ => return None,
        };
        Some(t)
    }

    fn bare_type(&self, entry: &TypeTableEntry) -> Ty {
        let gargs: Vec<Ty> = entry.generics().iter().map(|g| Ty::Var(g.clone())).collect();
        match entry {
            TypeTableEntry::Class(t) => Ty::Class(t.name.clone(), gargs),
            TypeTableEntry::Struct(t) => Ty::Struct(t.name.clone(), gargs),
            TypeTableEntry::Enum(t) => Ty::Enum(t.name.clone(), gargs),
            TypeTableEntry::Interface(t) => Ty::Interface(t.name.clone(), gargs),
        }
    }

    fn instantiate(&self, bare: &Ty, args: &[Ty], span: Span) -> Ty {
        let (name, expected): (String, Vec<String>) = match bare {
            Ty::List(_) => {
                if let Some(e) = args.first() {
                    let holds_refs = matches!(e, Ty::Ref(_))
                        || matches!(e, Ty::Option(inner) if matches!(inner.as_ref(), Ty::Ref(_)));
                    if holds_refs {
                        self.diags.emit(
                            Diagnostic::error_at(
                                span,
                                "lists of `&T` references are not supported yet",
                            )
                            .with_code(crate::error::ErrorCode::ListOfRefs),
                        );
                    }
                }
                return Ty::List(Box::new(args.first().cloned().unwrap_or(Ty::Unknown)));
            }
            Ty::Map(..) => {
                let k = args.first().cloned().unwrap_or(Ty::Unknown);
                let v = args.get(1).cloned().unwrap_or(Ty::Unknown);
                return Ty::Map(Box::new(k), Box::new(v));
            }
            Ty::Class(n, _) | Ty::Struct(n, _) | Ty::Enum(n, _) | Ty::Interface(n, _) => {
                (n.clone(), self.generics_of(n))
            }
            _ => {
                self.diags
                    .emit(Diagnostic::error_at(span, "type is not generic").with_code(crate::error::ErrorCode::NotGeneric));
                return bare.clone();
            }
        };
        if args.len() != expected.len() {
            self.diags.emit(
                Diagnostic::error_at(
                    span,
                    format!(
                        "wrong number of type arguments for `{name}`: expected {}, found {}",
                        expected.len(),
                        args.len()
                    ),
                )
                .with_code(crate::error::ErrorCode::WrongTypeArgs)
                .note(format!("declared generic parameters: {}", render_list(&expected))),
            );
        }
        match bare {
            Ty::Class(..) => Ty::Class(name, args.to_vec()),
            Ty::Struct(..) => Ty::Struct(name, args.to_vec()),
            Ty::Enum(..) => Ty::Enum(name, args.to_vec()),
            Ty::Interface(..) => Ty::Interface(name, args.to_vec()),
            _ => bare.clone(),
        }
    }

    fn generics_of(&self, name: &str) -> Vec<String> {
        self.types
            .get(name)
            .map(|e| e.generics().to_vec())
            .unwrap_or_default()
    }
}

impl ResolvedProgram {
    /// Convenience entry point for type-checking: resolve `te` with the final
    /// module-wide tables (not a fresh resolver).
    pub fn resolve_ty(
        &self,
        te: &TypeExpr,
        generics: &[String],
        diags: &DiagnosticSink,
    ) -> Ty {
        TypeCtx {
            diags,
            types: &self.types,
            import_aliases: &self.import_aliases,
        }
        .resolve_ty(te, generics)
    }
}

struct MemberTables {
    fields: Vec<FieldInfo>,
    methods: Vec<CallableInfo>,
    properties: Vec<PropertyInfo>,
    ctor: Option<CtorInfo>,
    named_ctors: Vec<(String, CtorInfo)>,
    consts: Vec<FieldInfo>,
}

impl<'a> Resolver<'a> {
    // ---- type declarations ------------------------------------------------

    /// Infer the type of a field/const initializer syntactically, so a member
    /// declared without an annotation still types member reads (`this.x`,
    /// `obj.x`) instead of resolving those to `Ty::Unknown`. This mirrors what
    /// the checker infers for the initializer; anything not covered here stays
    /// `Unknown` and the checker's own inference takes over at the use site.
    fn init_expr_ty(&self, e: &Expr, generics: &[String]) -> Option<Ty> {
        match &e.kind {
            ExprKind::Lit(lit) => match lit {
                Lit::String(_) => Some(Ty::String),
                Lit::Int { .. } => Some(Ty::Int),
                Lit::Float { .. } => Some(Ty::Float),
                Lit::Char(_) => Some(Ty::Char),
                Lit::Bool(_) => Some(Ty::Bool),
                Lit::None => Some(Ty::None),
            },
            ExprKind::Array(items) => {
                let elem = items
                    .first()
                    .and_then(|i| self.init_expr_ty(i, generics))
                    .unwrap_or(Ty::Unknown);
                Some(Ty::List(Box::new(elem)))
            }
            ExprKind::Map(pairs) => {
                let k = pairs
                    .first()
                    .map(|(k, _)| self.init_expr_ty(k, generics).unwrap_or(Ty::Unknown))
                    .unwrap_or(Ty::Unknown);
                let v = pairs
                    .first()
                    .map(|(_, v)| self.init_expr_ty(v, generics).unwrap_or(Ty::Unknown))
                    .unwrap_or(Ty::Unknown);
                Some(Ty::Map(Box::new(k), Box::new(v)))
            }
            ExprKind::Tuple(items) => Some(Ty::Tuple(
                items
                    .iter()
                    .map(|i| self.init_expr_ty(i, generics).unwrap_or(Ty::Unknown))
                    .collect(),
            )),
            ExprKind::Call { callee, .. } => match &callee.kind {
                ExprKind::GenericCall { type_args, .. } => {
                    type_args.first().map(|t| self.resolve_ty(t, generics))
                }
                ExprKind::Ident(name) => match self.types.get(name) {
                    Some(TypeTableEntry::Class(t)) if t.generics.is_empty() => {
                        Some(Ty::Class(name.clone(), Vec::new()))
                    }
                    Some(TypeTableEntry::Struct(t)) if t.generics.is_empty() => {
                        Some(Ty::Struct(name.clone(), Vec::new()))
                    }
                    Some(TypeTableEntry::Enum(t)) if t.generics.is_empty() => {
                        Some(Ty::Enum(name.clone(), Vec::new()))
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    fn declare_class(&mut self, c: &ClassDecl) {
        let generics: Vec<String> = c.generics.iter().map(|g| g.name.clone()).collect();
        let extends = c.extends.as_ref().map(|e| self.resolve_ty(e, &generics));
        if let Some(t) = &extends {
            if !matches!(t, Ty::Class(..)) {
                self.diags.emit(
                    Diagnostic::error_at(c.span, format!("`{}` may only extend a class", c.name))
                        .with_code(crate::error::ErrorCode::InvalidExtends),
                );
            }
        }
        let implements: Vec<Ty> = c
            .implements
            .iter()
            .map(|t| self.resolve_ty(t, &generics))
            .collect();
        let members = self.resolve_class_members(&c.members, &generics, &c.name);
        self.types.insert(
            c.name.clone(),
            TypeTableEntry::Class(ClassTable {
                name: c.name.clone(),
                span: c.span,
                visibility: c.visibility,
                generics,
                extends,
                implements,
                fields: members.fields,
                methods: members.methods,
                properties: members.properties,
                ctor: members.ctor,
                named_ctors: members.named_ctors,
                consts: members.consts,
            }),
        );
    }

    fn declare_struct(&mut self, s: &StructDecl) {
        let generics: Vec<String> = s.generics.iter().map(|g| g.name.clone()).collect();
        let implements: Vec<Ty> = s
            .implements
            .iter()
            .map(|t| self.resolve_ty(t, &generics))
            .collect();
        let members = self.resolve_class_members(&s.members, &generics, &s.name);
        self.types.insert(
            s.name.clone(),
            TypeTableEntry::Struct(ClassTable {
                name: s.name.clone(),
                span: s.span,
                visibility: s.visibility,
                generics,
                extends: None,
                implements,
                fields: members.fields,
                methods: members.methods,
                properties: members.properties,
                ctor: members.ctor,
                named_ctors: members.named_ctors,
                consts: members.consts,
            }),
        );
    }

    fn declare_enum(&mut self, e: &EnumDecl) {
        let generics: Vec<String> = e.generics.iter().map(|g| g.name.clone()).collect();
        // Register the type early so payloads can reference the enum itself.
        self.types.insert(
            e.name.clone(),
            TypeTableEntry::Enum(EnumTable {
                name: e.name.clone(),
                span: e.span,
                visibility: e.visibility,
                generics: generics.clone(),
                implements: Vec::new(),
                variants: Vec::new(),
            }),
        );
        let implements: Vec<Ty> = e
            .implements
            .iter()
            .map(|t| self.resolve_ty(t, &generics))
            .collect();
        let variants: Vec<(String, Vec<Ty>, Span)> = e
            .variants
            .iter()
            .map(|v| {
                let tys = v
                    .fields
                    .iter()
                    .map(|f| self.resolve_ty(&f.ty, &generics))
                    .collect();
                (v.name.clone(), tys, v.span)
            })
            .collect();
        if let Some(TypeTableEntry::Enum(t)) = self.types.get_mut(&e.name) {
            t.implements = implements;
            t.variants = variants;
        }
    }

    fn declare_interface(&mut self, i: &InterfaceDecl) {
        let generics: Vec<String> = i.generics.iter().map(|g| g.name.clone()).collect();
        let extends: Vec<Ty> = i
            .extends
            .iter()
            .map(|t| self.resolve_ty(t, &generics))
            .collect();
        let members: Vec<InterfaceMemberInfo> = i
            .members
            .iter()
            .map(|m| match m {
                InterfaceMember::Method(md) => InterfaceMemberInfo {
                    span: md.span,
                    name: md.name.clone(),
                    is_property: false,
                    ty: self.resolve_ret(&md.return_ty, &generics),
                },
                InterfaceMember::Property { name, ty, span } => InterfaceMemberInfo {
                    span: *span,
                    name: name.clone(),
                    is_property: true,
                    ty: self.resolve_ty(ty, &generics),
                },
                InterfaceMember::Const { name, span, .. } => InterfaceMemberInfo {
                    span: *span,
                    name: name.clone(),
                    is_property: true,
                    ty: Ty::Unknown,
                },
            })
            .collect();
        self.types.insert(
            i.name.clone(),
            TypeTableEntry::Interface(InterfaceTable {
                name: i.name.clone(),
                span: i.span,
                visibility: i.visibility,
                generics,
                extends,
                members,
            }),
        );
    }

    fn resolve_class_members(
        &self,
        members: &[ClassMember],
        generics: &[String],
        class_name: &str,
    ) -> MemberTables {
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut properties = Vec::new();
        let mut ctor = None;
        let mut named_ctors: Vec<(String, CtorInfo)> = Vec::new();
        let mut consts = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        let mut has_deinit = false;

        let mut record = |name: &str, span: Span| {
            if seen.iter().any(|s| s == name) {
                self.diags.emit(
                    Diagnostic::error_at(
                        span,
                        format!("duplicate member `{name}` in `{class_name}`"),
                    )
                    .with_code(crate::error::ErrorCode::DuplicateMember),
                );
            } else {
                seen.push(name.to_string());
            }
        };

        for m in members {
            match m {
                ClassMember::Field {
                    name,
                    ty,
                    visibility,
                    is_static,
                    const_,
                    attrs,
                    init,
                    span,
                } => {
                    record(name, *span);
                    let field_ty = ty
                        .as_ref()
                        .map(|t| self.resolve_ty(t, generics))
                        .or_else(|| {
                            init.as_ref()
                                .and_then(|e| self.init_expr_ty(e, generics))
                        })
                        .unwrap_or(Ty::Unknown);
                    fields.push(FieldInfo {
                        name: name.clone(),
                        visibility: *visibility,
                        is_static: *is_static,
                        mutable: !const_,
                        const_: *const_,
                        manual: attrs.iter().any(|a| a.name == "manualAlloc"),
                        ty: field_ty,
                        span: *span,
                    });
                }
                ClassMember::Const {
                    name,
                    visibility,
                    ty,
                    span,
                    ..
                } => {
                    record(name, *span);
                    consts.push(FieldInfo {
                        name: name.clone(),
                        visibility: *visibility,
                        is_static: true,
                        mutable: false,
                        const_: true,
                        manual: false,
                        ty: ty
                            .as_ref()
                            .map(|t| self.resolve_ty(t, generics))
                            .unwrap_or(Ty::Unknown),
                        span: *span,
                    });
                }
                ClassMember::Method(md) => {
                    record(&md.name, md.span);
                    let mg: Vec<String> = md.generics.iter().map(|g| g.name.clone()).collect();
                    methods.push(CallableInfo {
                        name: md.name.clone(),
                        span: md.span,
                        visibility: md.visibility,
                        is_async: md.is_async,
                        is_static: md.is_static,
                        is_override: md.is_override,
                        is_abstract: md.body.is_none(),
                        operator: md.operator.clone(),
                        generics: mg,
                        params: self.resolve_params(&md.params, generics),
                        ret: self.resolve_ret(&md.return_ty, generics),
                    });
                }
                ClassMember::Constructor(cd) => {
                    let info = CtorInfo {
                        span: cd.span,
                        visibility: cd.visibility,
                        params: self.resolve_params(&cd.params, generics),
                    };
                    match &cd.name {
                        Some(n) => {
                            record(n, cd.span);
                            named_ctors.push((n.clone(), info));
                        }
                        None => {
                            if ctor.is_some() {
                                self.diags.emit(
                                    Diagnostic::error_at(
                                        cd.span,
                                        format!("`{class_name}` already has a constructor"),
                                    )
                                    .with_code(crate::error::ErrorCode::DuplicateConstructor),
                                );
                            }
                            ctor = Some(info);
                        }
                    }
                }
                ClassMember::Property(p) => {
                    record(&p.name, p.span);
                    properties.push(PropertyInfo {
                        name: p.name.clone(),
                        visibility: p.visibility,
                        is_static: p.is_static,
                        ty: p
                            .ty
                            .as_ref()
                            .map(|t| self.resolve_ty(t, generics))
                            .unwrap_or(Ty::Unknown),
                        has_get: p.get.is_some(),
                        has_set: p.set.is_some(),
                        span: p.span,
                    });
                }
                ClassMember::Init(_) => {}
                ClassMember::Deinit(b) => {
                    if has_deinit {
                        self.diags.emit(
                            Diagnostic::error_at(
                                b.span,
                                format!("`{class_name}` already has a `deinit` block"),
                            )
                            .with_code(crate::error::ErrorCode::DuplicateDeinit),
                        );
                    } else {
                        has_deinit = true;
                    }
                }
            }
        }
        MemberTables {
            fields,
            methods,
            properties,
            ctor,
            named_ctors,
            consts,
        }
    }

    fn post_checks(&mut self) {
        let classes: Vec<ClassTable> = self
            .types
            .values()
            .filter_map(|v| match v {
                TypeTableEntry::Class(c) => Some(c.clone()),
                _ => None,
            })
            .collect();
        for c in classes {
            if let Some(parent) = c.extends.as_ref().and_then(|t| t.named().map(str::to_string)) {
                self.check_inheritance(&c, &parent);
            }
        }
    }

    /// Validate `override` flags between a class and its parent class chain,
    /// plus the parent class being real and direct.
    fn check_inheritance(&mut self, c: &ClassTable, parent: &str) {
        let TypeTableEntry::Class(pclass) = &self.types[parent] else {
            self.diags.emit(
                Diagnostic::error_at(
                    c.span,
                    format!("`{}` cannot inherit from non-class type `{parent}`", c.name),
                )
                .with_code(crate::error::ErrorCode::InvalidExtends),
            );
            return;
        };
        let pname = pclass.name.clone();

        let parent_methods: Vec<(String, Vec<ParamInfo>, Ty, bool)> =
            self.collect_methods_rec(&pname, &mut Vec::new());

        for m in &c.methods {
            let found = parent_methods.iter().find(|(n, ..)| n == &m.name);
            if let Some(pm) = found {
                // A redeclaration cannot switch kind. `static` and instance
                // methods share one name table, so an opposite-kind shadow
                // would route calls to the wrong function.
                if pm.3 != m.is_static {
                    self.diags.emit(
                        Diagnostic::error_at(
                            m.span,
                            format!(
                                "`{} {}` is `{}` but `{}.{}` is an {} method; a redeclaration must keep the same kind",
                                c.name,
                                m.name,
                                if m.is_static { "static" } else { "instance" },
                                pname,
                                pm.0,
                                if pm.3 { "static" } else { "instance" }
                            ),
                        )
                        .with_code(crate::error::ErrorCode::MixedMethodKind),
                    );
                    continue;
                }
            }
            match (found, m.is_override) {
                (Some(pm), true) => {
                    // Signature compatibility check.
                    let pm = pm.clone();
                    let (pparams, pret) = (pm.1, pm.2);
                    if !sig_compatible(&pparams, &pret, &m.params, &m.ret) {
                        self.diags.emit(
                            Diagnostic::error_at(
                                m.span,
                                format!(
                                    "`{} {}` does not override `{} {}`; signature mismatch",
                                    c.name, m.name, pname, m.name
                                ),
                            )
                            .with_code(crate::error::ErrorCode::OverrideSignature)
                            .note("an override must match the parent's parameter and return types exactly"),
                        );
                    }
                }
                (Some(pm), false) => {
                    self.diags.emit(
                        Diagnostic::error_at(
                            m.span,
                            format!("method `{}` hides `{}.{}` without `override`", m.name, pname, pm.0),
                        )
                        .with_code(crate::error::ErrorCode::MissingOverride)
                        .note("add `override` to override the inherited method, or rename this method"),
                    );
                }
                (None, true) => {
                    self.diags.emit(
                        Diagnostic::error_at(
                            m.span,
                            format!("`override` method `{}` has no matching method in `{pname}` or its parents", m.name),
                        )
                        .with_code(crate::error::ErrorCode::OrphanOverride),
                    );
                }
                (None, false) => {}
            }
        }
    }

    /// Collect all method signatures from a class and its ancestors
    /// (name -> (params, ret, is_static)).
    fn collect_methods_rec(
        &self,
        class: &str,
        visited: &mut Vec<String>,
    ) -> Vec<(String, Vec<ParamInfo>, Ty, bool)> {
        if visited.contains(&class.to_string()) {
            return Vec::new();
        }
        visited.push(class.to_string());
        let mut out = Vec::new();
        if let Some(TypeTableEntry::Class(t)) = self.types.get(class) {
            for m in &t.methods {
                out.push((m.name.clone(), m.params.clone(), m.ret.clone(), m.is_static));
            }
            if let Some(parent) = t.extends.as_ref().and_then(|e| e.named().map(str::to_string)) {
                out.extend(self.collect_methods_rec(&parent, visited));
            }
        }
        out
    }
}

fn sig_compatible(
    pp: &[ParamInfo],
    pret: &Ty,
    cp: &[ParamInfo],
    cret: &Ty,
) -> bool {
    if pp.len() != cp.len() {
        return false;
    }
    for (a, b) in pp.iter().zip(cp.iter()) {
        if a.ty != b.ty {
            return false;
        }
    }
    pret == cret
}