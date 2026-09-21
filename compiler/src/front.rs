use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::ast::*;
use crate::check::check_program;
use crate::diag::{Diagnostic, DiagnosticSink, FileId, SourceMap, Span};
use crate::error::ErrorCode;
use crate::parser::parse;
use crate::resolve::{ResolvedProgram, Resolver};

pub struct FrontOutput {
    pub program: Program,
    pub resolved: ResolvedProgram,
}

/// Lex, parse, resolve, and type-check a single source module. The caller owns
/// the `SourceMap` and `DiagnosticSink` (checked after this returns).
///
/// Any `import`/`use` declarations are loaded module-by-module from disk and
/// merged into one flat program; qualified references are rewritten before
/// resolution. The `file_name` is used as a filesystem path for import
/// resolution, so callers that want on-disk modules must pass a full path
/// (the CLI passes the file's display path).
pub fn frontend(
    file_name: &str,
    source: &str,
    map: &mut SourceMap,
    diags: &DiagnosticSink,
) -> Option<FrontOutput> {
    let loaded = Loader::new(map, diags).load_root(file_name, source)?;
    let mut program = loaded.merged_program;
    Rewriter {
        ctx: &loaded.imports_for,
        collided: &loaded.collided,
        diags,
    }
    .rewrite_program(&mut program);
    let resolved = Resolver::with_files(diags, loaded.file_roots).resolve(&program);
    Some(FrontOutput { program, resolved })
}

/// Lex, parse, resolve, and type-check a source module, returning only whether
/// the whole front end passed. Bodies are checked after name resolution so the
/// checker sees the final tables.
pub fn frontend_checked(
    file_name: &str,
    source: &str,
    map: &mut SourceMap,
    diags: &DiagnosticSink,
) -> Option<FrontOutput> {
    let out = frontend(file_name, source, map, diags)?;
    check_program(&out.program, &out.resolved, diags);
    Some(out)
}

fn render_path(path: &[String]) -> String {
    path.join(".")
}

/// `A`, `B`, `C`, ... for the alias suggestions in ambiguity notes.
fn alias_suffix(i: usize) -> String {
    let mut n = i;
    let mut s = String::new();
    loop {
        s.push((b'A' + (n % 26) as u8) as char);
        n /= 26;
        if n == 0 {
            break;
        }
        n -= 1;
    }
    s
}

fn top_level_name(item: &Item) -> Option<&str> {
    match &item.kind {
        ItemKind::Fn(f) => Some(&f.name),
        ItemKind::Class(c) => Some(&c.name),
        ItemKind::Struct(s) => Some(&s.name),
        ItemKind::Enum(e) => Some(&e.name),
        ItemKind::Interface(i) => Some(&i.name),
        ItemKind::Const(c) => Some(&c.name),
        ItemKind::Test(f) => Some(&f.name),
    }
}

fn stmt_span(s: &Stmt) -> Span {
    match s {
        Stmt::Let { span, .. }
        | Stmt::Const { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::Break { span }
        | Stmt::Continue { span }
        | Stmt::While { span, .. }
        | Stmt::For { span, .. }
        | Stmt::Expr(Expr { span, .. })
        | Stmt::Empty(span) => *span,
    }
}

/// What a single source file is allowed to see through its own imports.
#[derive(Default)]
struct FileImports {
    /// Importable module paths (full `a.b.c` paths from `import`/`use`), with
    /// the exported names of each resolved module.
    modules: HashMap<Vec<String>, ModuleBinding>,
    /// `import a.b as x` binds `x` to module path `a.b`.
    aliases: HashMap<String, Vec<String>>,
    /// `use a.b.item as x` binds local name `x` to exported item (`item`,
    /// provider module path). Aliased imports introduce the item to this file
    /// under the alias only.
    renames: HashMap<String, (String, Vec<String>)>,
    /// Public names this file may refer to bare (its own exports plus every
    /// `use a.b.item` / `use a.b.*` target), each with the provider module
    /// path(s). When a name resolves to two or more providers the reference
    /// is ambiguous and must be disambiguated with an alias.
    bindings: HashMap<String, Vec<String>>,
    /// The module path this file declares, when it declares one.
    own_module: Option<String>,
}

struct ModuleBinding {
    exports: Rc<HashSet<String>>,
}

impl FileImports {
    fn exports_of(&self, path: &[String]) -> Option<&HashSet<String>> {
        self.modules.get(path).map(|m| m.exports.as_ref())
    }
}

struct LoadedModules {
    merged_program: Program,
    imports_for: HashMap<FileId, Rc<FileImports>>,
    file_roots: HashMap<FileId, String>,
    /// Item names exported by two or more loaded modules: references to them
    /// must be disambiguated before flat resolution.
    collided: HashSet<String>,
}

struct ModuleFile {
    file_path: PathBuf,
    fid: FileId,
    module_path: Vec<String>,
    exports: HashSet<String>,
    imports: Vec<ImportDecl>,
    program: Program,
}

struct Loader<'a> {
    map: &'a mut SourceMap,
    diags: &'a DiagnosticSink,
    loaded: HashMap<PathBuf, usize>,
    files: Vec<ModuleFile>,
    imports_for: HashMap<FileId, Rc<FileImports>>,
    file_roots: HashMap<FileId, String>,
}

impl<'a> Loader<'a> {
    fn new(map: &'a mut SourceMap, diags: &'a DiagnosticSink) -> Loader<'a> {
        Loader {
            map,
            diags,
            loaded: HashMap::new(),
            files: Vec::new(),
            imports_for: HashMap::new(),
            file_roots: HashMap::new(),
        }
    }

    /// Parse the root module (source supplied by the caller) and all modules
    /// reachable from its imports, recursively. Returns the merged flat
    /// program plus the per-file import tables used by the rewrite pass.
    fn load_root(mut self, file_name: &str, source: &str) -> Option<LoadedModules> {
        let fid = self.map.add(file_name.to_string(), source.to_string());
        let program = self.parse_file(fid, source)?;
        let declared = program.module.as_ref().map(|m| m.path.clone());
        let module_path = declared.unwrap_or_default();
        let mut exports = HashSet::new();
        for item in &program.items {
            if let Some(n) = top_level_name(item) {
                exports.insert(n.to_string());
            }
        }
        let root_path = PathBuf::from(file_name);
        self.files.push(ModuleFile {
            file_path: root_path.clone(),
            fid,
            module_path,
            exports,
            imports: program.imports.clone(),
            program,
        });
        self.loaded.insert(
            root_path.canonicalize().unwrap_or_else(|_| root_path.clone()),
            0,
        );
        if !self.files[0].module_path.is_empty() {
            self.file_roots
                .insert(fid, self.files[0].module_path.join("."));
        }
        self.process_imports(0);

        let mut item_counts: HashMap<&str, usize> = HashMap::new();
        for f in &self.files {
            if f.module_path.is_empty() {
                continue;
            }
            for n in &f.exports {
                *item_counts.entry(n.as_str()).or_default() += 1;
            }
        }
        let collided = item_counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(n, _)| n.to_string())
            .collect();

        let mut items = Vec::new();
        for f in &self.files {
            items.extend(f.program.items.iter().cloned());
        }
        let merged_program = Program {
            module: self.files[0].program.module.clone(),
            imports: Vec::new(),
            items,
        };
        Some(LoadedModules {
            merged_program,
            imports_for: self.imports_for,
            file_roots: self.file_roots,
            collided,
        })
    }

    fn parse_file(&mut self, fid: FileId, source: &str) -> Option<Program> {
        let tokens = crate::lexer::lex(fid, source, self.diags);
        match parse(tokens, self.diags) {
            Ok(program) => Some(program),
            Err(()) => None,
        }
    }

    /// Resolve a module path to a file: first relative to the importing
    /// file's directory, then under `<cwd>/src/`.
    fn resolve_module_file(&self, importer: &Path, segments: &[String]) -> Option<PathBuf> {
        let mut rel = PathBuf::new();
        for s in segments {
            rel.push(s);
        }
        rel.set_extension("pkl");
        let importer_dir = importer
            .parent()
            .map(Path::to_path_buf)
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| PathBuf::from("."));
        let candidate = importer_dir.join(&rel);
        if candidate.is_file() {
            return Some(candidate);
        }
        if let Ok(cwd) = std::env::current_dir() {
            let candidate = cwd.join("src").join(&rel);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        None
    }

    /// Load the file for module `segments` (if not already loaded) and return
    /// its index. Emits `ModuleNotFound` when no file matches.
    fn ensure_module(&mut self, importer: &Path, segments: &[String], span: Span) -> Option<usize> {
        if segments.is_empty() {
            self.diags.emit(
                Diagnostic::error_at(span, "module path must not be empty")
                    .with_code(ErrorCode::ModuleNotFound),
            );
            return None;
        }
        match self.resolve_module_file(importer, segments) {
            Some(file) => self.load_file(&file, Some((segments, span))),
            None => {
                self.diags.emit(
                    Diagnostic::error_at(
                        span,
                        format!("cannot find module `{}`", render_path(segments)),
                    )
                    .with_code(ErrorCode::ModuleNotFound)
                    .note(
                        "searched relative to the importing file, then `<cwd>/src/<path>.pkl`",
                    ),
                );
                None
            }
        }
    }

    fn load_file(&mut self, path: &Path, imported_as: Option<(&[String], Span)>) -> Option<usize> {
        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if let Some(&i) = self.loaded.get(&canon) {
            return Some(i);
        }
        let source = std::fs::read_to_string(path).ok()?;
        let fid = self.map.add(path.display().to_string(), source.clone());
        let program = self.parse_file(fid, &source)?;

        let declared = program.module.as_ref().map(|m| m.path.clone());
        if let (Some(decl_path), Some((import_path, ispan))) = (declared.as_ref(), imported_as) {
            if decl_path != import_path {
                self.diags.emit(
                    Diagnostic::error_at(
                        ispan,
                        format!(
                            "file `{}` declares module `{}`, not `{}`",
                            path.display(),
                            render_path(decl_path),
                            render_path(import_path),
                        ),
                    )
                    .with_code(ErrorCode::ModuleMismatch),
                );
            }
        }
        let module_path = declared
            .clone()
            .or_else(|| imported_as.map(|(ip, _)| ip.to_vec()))
            .unwrap_or_default();

        let mut exports = HashSet::new();
        for item in &program.items {
            if let Some(n) = top_level_name(item) {
                exports.insert(n.to_string());
            }
        }
        let idx = self.files.len();
        self.files.push(ModuleFile {
            file_path: path.to_path_buf(),
            fid,
            module_path,
            exports,
            imports: program.imports.clone(),
            program,
        });
        self.loaded.insert(canon, idx);
        if !self.files[idx].module_path.is_empty() {
            self.file_roots
                .insert(fid, self.files[idx].module_path.join("."));
        }
        self.process_imports(idx);
        Some(idx)
    }

    /// Build the import tables for one module and load its dependencies.
    fn process_imports(&mut self, idx: usize) {
        let imports = self.files[idx].imports.clone();
        let importer = self.files[idx].file_path.clone();
        let mut fi = FileImports::default();
        let mut bound_aliases = HashSet::new();
        for imp in &imports {
            match &imp.kind {
                ImportKind::Module { path, alias } => {
                    let target = self.ensure_module(&importer, path, imp.span);
                    let exports = target
                        .map(|t| self.files[t].exports.clone())
                        .unwrap_or_default();
                    let alias_name = alias
                        .clone()
                        .unwrap_or_else(|| path.last().cloned().unwrap_or_default());
                    if !bound_aliases.insert(alias_name.clone()) || fi.aliases.contains_key(&alias_name)
                    {
                        self.diags.emit(
                            Diagnostic::error_at(
                                imp.span,
                                format!("duplicate import alias `{alias_name}`"),
                            )
                            .with_code(ErrorCode::DuplicateImportAlias),
                        );
                    }
                    fi.aliases.insert(alias_name, path.clone());
                    fi.modules.insert(
                        path.clone(),
                        ModuleBinding {
                            exports: Rc::new(exports),
                        },
                    );
                }
                ImportKind::Item { path, alias } => {
                    if path.len() < 2 {
                        self.diags.emit(
                            Diagnostic::error_at(
                                imp.span,
                                "`use` requires a module-qualified path like `use a.b.item`",
                            )
                            .with_code(ErrorCode::ModuleNotFound),
                        );
                        continue;
                    }
                    let mpath = path[..path.len() - 1].to_vec();
                    let item = path[path.len() - 1].clone();
                    let provider = render_path(&mpath);
                    let target = self.ensure_module(&importer, &mpath, imp.span);
                    let exports = target
                        .map(|t| self.files[t].exports.clone())
                        .unwrap_or_default();
                    if target.is_some() && !exports.contains(&item) {
                        self.diags.emit(
                            Diagnostic::error_at(
                                imp.span,
                                format!(
                                    "`{item}` is not exported by module `{}`",
                                    render_path(&mpath)
                                ),
                            )
                            .with_code(ErrorCode::NotExported),
                        );
                    }
                    fi.modules
                        .entry(mpath.clone())
                        .or_insert(ModuleBinding {
                            exports: Rc::new(exports.clone()),
                        });
                    if let Some(alias) = alias {
                        if fi.renames.contains_key(alias) {
                            self.diags.emit(
                                Diagnostic::error_at(
                                    imp.span,
                                    format!("duplicate import alias `{alias}`"),
                                )
                                .with_code(ErrorCode::DuplicateImportAlias),
                            );
                        }
                        fi.renames.insert(alias.clone(), (item, mpath));
                    } else {
                        fi.bindings.entry(item).or_default().push(provider);
                    }
                }
                ImportKind::Star { path } => {
                    let target = self.ensure_module(&importer, path, imp.span);
                    let exports = target
                        .map(|t| self.files[t].exports.clone())
                        .unwrap_or_default();
                    let provider = render_path(path);
                    for name in &exports {
                        fi.bindings
                            .entry(name.clone())
                            .or_default()
                            .push(provider.clone());
                    }
                    fi.modules.entry(path.clone()).or_insert(ModuleBinding {
                        exports: Rc::new(exports),
                    });
                }
            }
        }
        let own = self.files[idx].module_path.clone();
        if !own.is_empty() {
            let own_provider = own.join(".");
            for name in self.files[idx].exports.clone() {
                fi.bindings.entry(name).or_default().push(own_provider.clone());
            }
            fi.own_module = Some(own_provider);
        }
        self.imports_for
            .insert(self.files[idx].fid, Rc::new(fi));
    }
}

/// Rewrite module-qualified references (`compiler.lexer.Lexer(...)`, `lex.X`,
/// `use a.b.item as x`) into the plain names the flat resolver understands.
/// The rewrite is driven per source file by that file's own imports.
struct Rewriter<'a> {
    ctx: &'a HashMap<FileId, Rc<FileImports>>,
    /// Item names exported by more than one loaded module. Such items are
    /// flattened under their module path (`math.ops.helper`), and any bare
    /// reference with several in-scope providers is an error until the
    /// programmer aliases it.
    collided: &'a HashSet<String>,
    diags: &'a DiagnosticSink,
}

impl<'a> Rewriter<'a> {
    fn rewrite_program(&self, program: &mut Program) {
        for item in &mut program.items {
            self.rewrite_item(item);
        }
    }

    fn ctx_for(&self, fid: FileId) -> Option<&FileImports> {
        self.ctx.get(&fid).map(|c| c.as_ref())
    }

    /// The flat name of `item` declared by module `provider`: the plain item
    /// name when it is unique, otherwise the module-qualified name.
    fn canon(&self, item: &str, provider: &str) -> String {
        if self.collided.contains(item) {
            format!("{provider}.{item}")
        } else {
            item.to_string()
        }
    }

    /// Resolve a bare in-scope name. A single provider canonicalizes; two or
    /// more is the ambiguity the aliasing syntax exists to resolve.
    fn canonical_or_ambiguous(
        &self,
        name: &str,
        providers: &[String],
        span: Span,
    ) -> Option<Expr> {
        let mut uniq: Vec<&String> = providers.iter().collect();
        uniq.sort_unstable();
        uniq.dedup();
        match uniq.len() {
            0 => None,
            1 => Some(qualified_expr(
                span,
                &[self.canon(name, uniq[0])],
            )),
            _ => {
                let mut diag =
                    Diagnostic::error_at(span, format!("ambiguous imported name `{name}`"))
                        .with_code(ErrorCode::AmbiguousImport);
                let mut note = format!("`{name}` is provided by module `{}`", uniq[0]);
                for p in &uniq[1..] {
                    note.push_str(&format!(" and module `{p}`"));
                }
                diag = diag.note(note);
                let mut suggestion = String::new();
                for (i, p) in uniq.iter().enumerate() {
                    if i > 0 {
                        suggestion.push_str(" / ");
                    }
                    suggestion.push_str(&format!(
                        "use `{p}.{name} as {name}{}`",
                        alias_suffix(i)
                    ));
                }
                diag = diag.note(suggestion);
                self.diags.emit(diag);
                Some(qualified_expr(span, &[name.to_string()]))
            }
        }
    }

    fn rewrite_item(&self, item: &mut Item) {
        match &mut item.kind {
            ItemKind::Fn(f) => {
                self.rename_decl(&mut f.name, item.span.file);
                self.rewrite_fn(f);
            }
            ItemKind::Test(f) => {
                self.rename_decl(&mut f.name, item.span.file);
                self.rewrite_fn(f);
            }
            ItemKind::Class(c) => {
                if let Some(te) = &mut c.extends {
                    self.rewrite_type(te);
                }
                for te in &mut c.implements {
                    self.rewrite_type(te);
                }
                for m in &mut c.members {
                    self.rewrite_class_member(m);
                }
            }
            ItemKind::Struct(s) => {
                for te in &mut s.implements {
                    self.rewrite_type(te);
                }
                for m in &mut s.members {
                    self.rewrite_class_member(m);
                }
            }
            ItemKind::Enum(e) => {
                for te in &mut e.implements {
                    self.rewrite_type(te);
                }
                for v in &mut e.variants {
                    for f in &mut v.fields {
                        self.rewrite_type(&mut f.ty);
                    }
                }
            }
            ItemKind::Interface(i) => {
                for te in &mut i.extends {
                    self.rewrite_type(te);
                }
                for m in &mut i.members {
                    match m {
                        InterfaceMember::Method(m) => self.rewrite_method(m),
                        InterfaceMember::Property { ty, .. } => self.rewrite_type(ty),
                        InterfaceMember::Const { value, .. } => self.rewrite_expr(value),
                    }
                }
            }
            ItemKind::Const(c) => {
                self.rename_decl(&mut c.name, item.span.file);
                if let Some(ty) = &mut c.ty {
                    self.rewrite_type(ty);
                }
                self.rewrite_expr(&mut c.value);
            }
        }
    }

    /// A collided top-level function, test, or const declared by a module file
    /// is flattened under its module-qualified name so the merged program
    /// keeps distinct table entries and emitted symbols per provider.
    fn rename_decl(&self, name: &mut String, fid: FileId) {
        let Some(provider) = self.ctx_for(fid).and_then(|c| c.own_module.clone()) else {
            return;
        };
        if self.collided.contains(name.as_str()) {
            *name = format!("{provider}.{name}");
        }
    }

    fn rewrite_class_member(&self, m: &mut ClassMember) {
        match m {
            ClassMember::Field {
                ty,
                init,
                attrs,
                ..
            } => {
                if let Some(ty) = ty {
                    self.rewrite_type(ty);
                }
                if let Some(init) = init {
                    self.rewrite_expr(init);
                }
                for a in attrs {
                    for e in &mut a.args {
                        self.rewrite_expr(e);
                    }
                }
            }
            ClassMember::Method(m) => self.rewrite_method(m),
            ClassMember::Constructor(ctor) => {
                for p in &mut ctor.params {
                    self.rewrite_param(p);
                }
                self.rewrite_block(&mut ctor.body);
            }
            ClassMember::Property(p) => {
                if let Some(ty) = &mut p.ty {
                    self.rewrite_type(ty);
                }
                if let Some(get) = &mut p.get {
                    self.rewrite_accessor(get);
                }
                if let Some(set) = &mut p.set {
                    self.rewrite_accessor(set);
                }
            }
            ClassMember::Init(b) => self.rewrite_block(b),
            ClassMember::Deinit(b) => self.rewrite_block(b),
            ClassMember::Const { ty, value, .. } => {
                if let Some(ty) = ty {
                    self.rewrite_type(ty);
                }
                self.rewrite_expr(value);
            }
        }
    }

    fn rewrite_accessor(&self, acc: &mut PropertyAccessor) {
        match acc {
            PropertyAccessor::Expr(e) => self.rewrite_expr(e),
            PropertyAccessor::Block(b) => self.rewrite_block(b),
        }
    }

    fn rewrite_fn(&self, f: &mut FnDecl) {
        for p in &mut f.params {
            self.rewrite_param(p);
        }
        if let Some(ty) = &mut f.return_ty {
            self.rewrite_type(ty);
        }
        if let Some(body) = &mut f.body {
            self.rewrite_body(body);
        }
    }

    fn rewrite_method(&self, m: &mut MethodDecl) {
        for p in &mut m.params {
            self.rewrite_param(p);
        }
        if let Some(ty) = &mut m.return_ty {
            self.rewrite_type(ty);
        }
        if let Some(body) = &mut m.body {
            self.rewrite_body(body);
        }
    }

    fn rewrite_param(&self, p: &mut Param) {
        if let Some(ty) = &mut p.ty {
            self.rewrite_type(ty);
        }
        if let Some(def) = &mut p.default {
            self.rewrite_expr(def);
        }
        for a in &mut p.attrs {
            for e in &mut a.args {
                self.rewrite_expr(e);
            }
        }
    }

    fn rewrite_body(&self, body: &mut FnBody) {
        match body {
            FnBody::Block(b) => self.rewrite_block(b),
            FnBody::Expr(e) => self.rewrite_expr(e),
        }
    }

    fn rewrite_block(&self, block: &mut Block) {
        for s in &mut block.stmts {
            self.rewrite_stmt(s);
        }
        if let Some(e) = &mut block.expr {
            self.rewrite_expr(e);
        }
    }

    fn rewrite_stmt(&self, stmt: &mut Stmt) {
        let fid = stmt_span(stmt).file;
        match stmt {
            Stmt::Let {
                pattern,
                ty,
                init,
                attrs,
                ..
            } => {
                self.rewrite_pattern(pattern, fid);
                if let Some(ty) = ty {
                    self.rewrite_type(ty);
                }
                if let Some(init) = init {
                    self.rewrite_expr(init);
                }
                for a in attrs {
                    for e in &mut a.args {
                        self.rewrite_expr(e);
                    }
                }
            }
            Stmt::Const { ty, value, .. } => {
                if let Some(ty) = ty {
                    self.rewrite_type(ty);
                }
                self.rewrite_expr(value);
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.rewrite_expr(v);
                }
            }
            Stmt::While { cond, body, .. } => {
                self.rewrite_expr(cond);
                self.rewrite_block(body);
            }
            Stmt::For { header, body, .. } => {
                match header {
                    ForHeader::In { pattern, sequence } => {
                        self.rewrite_pattern(pattern, fid);
                        self.rewrite_expr(sequence);
                    }
                    ForHeader::Range { init, cond, step } => {
                        self.rewrite_stmt(init);
                        self.rewrite_expr(cond);
                        self.rewrite_expr(step);
                    }
                }
                self.rewrite_block(body);
            }
            Stmt::Expr(e) => self.rewrite_expr(e),
            _ => {}
        }
    }

    fn rewrite_expr(&self, e: &mut Expr) {
        let ctx = self.ctx_for(e.span.file);
        let fid = e.span.file;
        if let Some(segs) = flatten_member_spine(e) {
            if let Some(rewritten) = self.match_chain(&segs, e.span, ctx) {
                *e = rewritten;
                return;
            }
        }
        match &mut e.kind {
            ExprKind::Call { callee, args } => {
                self.rewrite_expr(callee);
                for a in args {
                    self.rewrite_expr(&mut a.value);
                }
            }
            ExprKind::Member { object, .. } => self.rewrite_expr(object),
            ExprKind::OptAccess { object, .. } => self.rewrite_expr(object),
            ExprKind::Index { object, index } => {
                self.rewrite_expr(object);
                self.rewrite_expr(index);
            }
            ExprKind::Binary { lhs, rhs, .. } => {
                self.rewrite_expr(lhs);
                self.rewrite_expr(rhs);
            }
            ExprKind::Unary { operand, .. } => self.rewrite_expr(operand),
            ExprKind::Assign {
                target, value, ..
            } => {
                self.rewrite_expr(target);
                self.rewrite_expr(value);
            }
            ExprKind::Lambda {
                params,
                return_ty,
                body,
                ..
            } => {
                for p in params {
                    self.rewrite_param(p);
                }
                if let Some(ty) = return_ty {
                    self.rewrite_type(ty);
                }
                self.rewrite_body(body);
            }
            ExprKind::If {
                cond,
                then,
                else_else,
            } => {
                match cond {
                    IfCond::Cond(e) => self.rewrite_expr(e),
                    IfCond::Binding { pattern, value } => {
                        self.rewrite_pattern(pattern, fid);
                        self.rewrite_expr(value);
                    }
                }
                self.rewrite_block(then);
                if let Some(e) = else_else {
                    self.rewrite_expr(e);
                }
            }
            ExprKind::Match { scrutinee, arms } => {
                self.rewrite_expr(scrutinee);
                for arm in arms {
                    self.rewrite_pattern(&mut arm.pattern, arm.span.file);
                    if let Some(g) = &mut arm.guard {
                        self.rewrite_expr(g);
                    }
                    self.rewrite_expr(&mut arm.body);
                }
            }
            ExprKind::Await(e) => self.rewrite_expr(e),
            ExprKind::GenericCall { type_args, .. } => {
                for t in type_args {
                    self.rewrite_type(t);
                }
            }
            ExprKind::Cast { expr, ty, .. } => {
                self.rewrite_expr(expr);
                self.rewrite_type(ty);
            }
            ExprKind::Unsafe(b) => self.rewrite_block(b),
            ExprKind::Block(b) => self.rewrite_block(b),
            ExprKind::Tuple(es) | ExprKind::Array(es) => {
                for x in es {
                    self.rewrite_expr(x);
                }
            }
            ExprKind::Map(kvs) => {
                for (k, v) in kvs {
                    self.rewrite_expr(k);
                    self.rewrite_expr(v);
                }
            }
            ExprKind::OptUnwrap(e) => self.rewrite_expr(e),
            ExprKind::Range { start, end, .. } => {
                self.rewrite_expr(start);
                self.rewrite_expr(end);
            }
            _ => {}
        }
    }

    fn rewrite_type(&self, te: &mut TypeExpr) {
        let ctx = self.ctx_for(te.span.file);
        match &mut te.kind {
            TypeExprKind::Path(parts) => {
                if let Some(mut new) = self.match_type_path(parts, ctx) {
                    if new.len() == 1 {
                        if let Some(c) = ctx {
                            if let Some(item) = c.renames.get(&new[0]) {
                                new[0] = item.0.clone();
                            }
                        }
                    }
                    *parts = new;
                }
            }
            TypeExprKind::Generic(inner, args) => {
                self.rewrite_type(inner);
                for a in args {
                    self.rewrite_type(a);
                }
            }
            TypeExprKind::Pointer(t) | TypeExprKind::Ref(t) | TypeExprKind::Option(t) => {
                self.rewrite_type(t);
            }
            TypeExprKind::Tuple(ts) => {
                for t in ts {
                    self.rewrite_type(t);
                }
            }
            TypeExprKind::Fn { params, ret, .. } => {
                for t in params {
                    self.rewrite_type(t);
                }
                if let Some(r) = ret {
                    self.rewrite_type(r);
                }
            }
            _ => {}
        }
    }

    fn rewrite_pattern(&self, p: &mut Pattern, fid: FileId) {
        let ctx = self.ctx_for(fid);
        match p {
            Pattern::Binding { ty, .. } => {
                if let Some(ty) = ty {
                    self.rewrite_type(ty);
                }
            }
            Pattern::Tuple(ps) => {
                for q in ps {
                    self.rewrite_pattern(q, fid);
                }
            }
            Pattern::Variant { path, payloads } => {
                if let Some(c) = ctx {
                    if path.len() >= 2 {
                        if let Some(k) = longest_prefix(path, c) {
                            *path = path[k..].to_vec();
                        } else if c.aliases.contains_key(&path[0]) {
                            *path = path[1..].to_vec();
                        }
                    }
                    if path.len() >= 1 {
                        if let Some(item) = c.renames.get(&path[0]) {
                            path[0] = item.0.clone();
                        }
                    }
                }
                for q in payloads {
                    self.rewrite_pattern(q, fid);
                }
            }
            Pattern::Or(ps) => {
                for q in ps {
                    self.rewrite_pattern(q, fid);
                }
            }
            _ => {}
        }
    }

    /// Resolve a dotted or aliased chain to a plain name when the prefix is a
    /// module path (or module alias) in scope, and the next segment is one of
    /// that module's exports. Returns `None` when the chain is not a module
    /// reference, in which case it is left to the checker as an ordinary
    /// member access.
    fn match_chain(&self, segs: &[String], span: Span, ctx: Option<&FileImports>) -> Option<Expr> {
        let ctx = ctx?;
        if segs.len() == 1 {
            let name = &segs[0];
            if let Some((item, provider)) = ctx.renames.get(name) {
                return Some(qualified_expr(
                    span,
                    &[self.canon(item, &render_path(provider))],
                ));
            }
            if let Some(providers) = ctx.bindings.get(name) {
                return self.canonical_or_ambiguous(name, providers, span);
            }
            return None;
        }
        if let Some(k) = longest_prefix(segs, ctx) {
            let prefix = &segs[..k];
            let remaining = &segs[k..];
            if ctx.exports_of(prefix)?.contains(&remaining[0]) {
                if remaining.len() == 1 && self.collided.contains(&remaining[0]) {
                    return Some(qualified_expr(
                        span,
                        &[self.canon(&remaining[0], &render_path(prefix))],
                    ));
                }
                return Some(qualified_expr(span, remaining));
            }
            return None;
        }
        if let Some(mod_path) = ctx.aliases.get(&segs[0]) {
            let remaining = &segs[1..];
            if !remaining.is_empty() {
                if ctx.exports_of(mod_path)?.contains(&remaining[0]) {
                    if remaining.len() == 1 && self.collided.contains(&remaining[0]) {
                        return Some(qualified_expr(
                            span,
                            &[self.canon(&remaining[0], &render_path(mod_path))],
                        ));
                    }
                    return Some(qualified_expr(span, remaining));
                }
            }
            return None;
        }
        None
    }

    /// For `TypeExprKind::Path`: drop a matched module prefix, or return the
    /// path unchanged for a bare single segment. Renames for single segments
    /// are applied by the caller.
    fn match_type_path(&self, parts: &[String], ctx: Option<&FileImports>) -> Option<Vec<String>> {
        if parts.len() <= 1 {
            return None;
        }
        let ctx = ctx?;
        if let Some(k) = longest_prefix(parts, ctx) {
            return Some(parts[k..].to_vec());
        }
        if ctx.aliases.contains_key(&parts[0]) {
            return Some(parts[1..].to_vec());
        }
        None
    }
}

/// Longest `k` (1 <= k < segs.len()) with `segs[..k]` an importable module path.
fn longest_prefix(segs: &[String], ctx: &FileImports) -> Option<usize> {
    let mut k = 0;
    for i in 1..segs.len() {
        if ctx.modules.contains_key(&segs[..i]) {
            k = i;
        }
    }
    (k > 0).then_some(k)
}

fn flatten_member_spine(e: &Expr) -> Option<Vec<String>> {
    match &e.kind {
        ExprKind::Ident(n) => Some(vec![n.clone()]),
        ExprKind::Member { object, name } => {
            let mut segs = flatten_member_spine(object)?;
            segs.push(name.clone());
            Some(segs)
        }
        _ => None,
    }
}

fn qualified_expr(span: Span, remaining: &[String]) -> Expr {
    let mut e = Expr {
        span,
        kind: ExprKind::Ident(remaining[0].clone()),
    };
    for seg in &remaining[1..] {
        e = Expr {
            span,
            kind: ExprKind::Member {
                object: Box::new(e),
                name: seg.clone(),
            },
        };
    }
    e
}