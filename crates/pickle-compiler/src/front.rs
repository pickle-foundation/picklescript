use crate::ast::Program;
use crate::diag::{DiagnosticSink, SourceMap};
use crate::parser::parse;
use crate::resolve::{ResolvedProgram, Resolver};

pub struct FrontOutput {
    pub program: Program,
    pub resolved: ResolvedProgram,
}

/// Lex, parse, and resolve a single source module. The caller owns the
/// `SourceMap` and `DiagnosticSink` (checked after this returns).
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