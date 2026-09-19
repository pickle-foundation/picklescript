use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pickle_compiler::diag::DiagnosticSink;
use pickle_compiler::front::{frontend, frontend_checked};
use pickle_compiler::diag::SourceMap;

use std::io::IsTerminal;

#[derive(Parser)]
#[command(name = "pickle", version, about = "The PickleScript toolchain")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse, resolve, and type-check a module
    Check {
        /// Source file to check
        file: PathBuf,
    },
    /// Print the parsed AST for a module
    Ast {
        /// Source file to parse
        file: PathBuf,
    },
}

fn read_source(path: &PathBuf) -> Result<String> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    Ok(text)
}

fn load(file: &PathBuf) -> Result<(String, SourceMap, DiagnosticSink)> {
    let source = read_source(file)?;
    let map = SourceMap::default();
    let diags = DiagnosticSink::new();
    Ok((source, map, diags))
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let colored = std::io::stderr().is_terminal();

    match &cli.command {
        Command::Check { file } => {
            let (source, mut map, diags) = load(file)?;
            let ok = frontend_checked(&file.display().to_string(), &source, &mut map, &diags);
            let rendered = diags.render_all(&map, colored);
            if !rendered.is_empty() {
                eprint!("{rendered}");
            }
            if diags.any_error() || ok.is_none() {
                std::process::exit(1);
            }
        }
        Command::Ast { file } => {
            let (source, mut map, diags) = load(file)?;
            if let Some(out) = frontend(&file.display().to_string(), &source, &mut map, &diags) {
                println!("{:#?}", out.program);
            }
            let rendered = diags.render_all(&map, colored);
            if !rendered.is_empty() {
                eprint!("{rendered}");
            }
            if diags.any_error() {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}