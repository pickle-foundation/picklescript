use crate::ast::Program;
use crate::check::check_program;
use crate::diag::{DiagnosticSink, SourceMap};
use crate::parser::parse;
use crate::resolve::{ResolvedProgram, Resolver};

pub struct FrontOutput {
    pub program: Program,
    pub resolved: ResolvedProgram,
}

/// Lex, parse, resolve, and type-check a single source module. The caller owns
/// the `SourceMap` and `DiagnosticSink` (checked after this returns).
pub fn frontend(
    file_name: &str,
    source: &str,
    map: &mut SourceMap,
    diags: &DiagnosticSink,
) -> Option<FrontOutput> {
    let fid = map.add(file_name.to_string(), source.to_string());
    let tokens = crate::lexer::lex(fid, source, diags);
    let Ok(program) = parse(tokens, diags) else {
        return None;
    };
    let resolved = Resolver::new(diags).resolve(&program);
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