use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pickle_compiler::diag::DiagnosticSink;
use pickle_compiler::front::{frontend, frontend_checked};
use pickle_compiler::diag::SourceMap;

use std::io::IsTerminal;

mod build;
mod history_cli;
mod jit;

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
        /// Emit machine-readable JSON for every diagnostic on stdout
        #[arg(long)]
        json: bool,
        /// Group diagnostics under their error code
        #[arg(long)]
        group: bool,
    },
    /// Print the parsed AST for a module
    Ast {
        /// Source file to parse
        file: PathBuf,
    },
    /// Lower a checked module to PickleIR and print it
    Ir {
        /// Source file to lower
        file: PathBuf,
    },
    /// Explain a stable error code (e.g. `E0308`); with no code, list every
    /// code in the catalogue
    Explain {
        /// Error code to explain
        code: Option<String>,
        /// Also show, oldest first, every committed version of a `*.pkl`
        /// file that produced this code
        #[arg(long)]
        history: bool,
    },
    /// Show how public types and dependencies evolved across git history
    History {
        /// Only show history for this file
        #[arg(long)]
        file: Option<String>,
        /// Number of versions to show per file
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Show, newest first, which committed versions compiled and which
    /// failed, with the error codes of the failures
    Builds {
        /// Number of builds to show overall
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Compile a module with the JIT and run it
    Run {
        /// Source file to run
        file: PathBuf,
    },
    /// Compile a module to a native executable, linking the runtime
    Build {
        /// Source file to compile
        file: PathBuf,
        /// Output executable path (defaults to next to the source)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn read_source(path: &PathBuf) -> Result<String> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    Ok(text)
}

/// Default output path for `pickle build`: the source's stem plus the
/// platform executable suffix, next to the source file.
fn default_output(file: &Path) -> PathBuf {
    let dir = file
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let stem = file
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "pickle_program".to_string());
    let name = if cfg!(windows) { format!("{stem}.exe") } else { stem };
    dir.join(name)
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
        Command::Check { file, json, group } => {
            let (source, mut map, diags) = load(file)?;
            let ok = frontend_checked(&file.display().to_string(), &source, &mut map, &diags);
            if *json {
                // Machine mode: the JSON itself goes to stdout, human
                // diagnostics are suppressed. The consumer decides what to
                // show and the exit code carries pass/fail.
                print!("{}", diags.render_all_json(&map));
            } else if *group {
                let rendered = diags.render_all_grouped(&map, colored);
                if !rendered.is_empty() {
                    eprint!("{rendered}");
                }
            } else {
                let rendered = diags.render_all(&map, colored);
                if !rendered.is_empty() {
                    eprint!("{rendered}");
                }
            }
            if diags.any_error() || ok.is_none() {
                std::process::exit(1);
            }
        }
        Command::Explain { code, history } => match code {
            Some(code) => {
                let id = format!(
                    "E{}",
                    code.trim().to_uppercase().trim_start_matches('E')
                );
                let entry = pickle_compiler::error::explain(&id);
                if *history {
                    if let Some(entry) = entry {
                        println!("error[{}] -- {}", entry.code.id(), entry.title);
                        println!();
                        println!("{}", entry.rule);
                        println!();
                        println!("example:");
                        for line in entry.example.lines() {
                            println!("    {line}");
                        }
                        println!();
                    } else {
                        println!("(not in the error catalogue -- checking its history anyway)");
                        println!();
                    }
                    let root = history_cli::git_root(&std::env::current_dir()?)?;
                    let all = history_cli::history_for(&root)?;
                    history_cli::render_explain_history(&all, &id);
                } else {
                    match entry {
                        Some(entry) => {
                            println!("error[{}] -- {}", entry.code.id(), entry.title);
                            println!();
                            println!("{}", entry.rule);
                            println!();
                            println!("example:");
                            for line in entry.example.lines() {
                                println!("    {line}");
                            }
                        }
                        None => {
                            anyhow::bail!("unknown error code `{id}`");
                        }
                    }
                }
            }
            None => {
                for entry in pickle_compiler::error::CATALOGUE {
                    println!(
                        "error[{}] -- {}",
                        entry.code.id(),
                        entry.title
                    );
                }
            }
        },
        Command::History { file, limit } => {
            let root = history_cli::git_root(&std::env::current_dir()?)?;
            let all = history_cli::history_for(&root)?;
            if all.is_empty() {
                println!("no `*.pkl` files in git history yet");
            } else {
                history_cli::render_history(&all, file.as_deref(), *limit);
            }
        }
        Command::Builds { limit } => {
            let root = history_cli::git_root(&std::env::current_dir()?)?;
            let all = history_cli::history_for(&root)?;
            if all.is_empty() {
                println!("no `*.pkl` files in git history yet");
            } else {
                history_cli::render_builds(&all, *limit);
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
        Command::Ir { file } => {
            let (source, mut map, diags) = load(file)?;
            let module = frontend(&file.display().to_string(), &source, &mut map, &diags)
                .and_then(|out| pickle_compiler::emit::emit_ir(&out.program, &out.resolved, &diags));
            let rendered = diags.render_all(&map, colored);
            if !rendered.is_empty() {
                eprint!("{rendered}");
            }
            match module {
                Some(m) => println!("{m}"),
                None => std::process::exit(1),
            }
        }
    Command::Run { file } => {
            let (source, mut map, diags) = load(file)?;
            let module = frontend(&file.display().to_string(), &source, &mut map, &diags)
                .and_then(|out| pickle_compiler::emit::emit_ir(&out.program, &out.resolved, &diags));
            let rendered = diags.render_all(&map, colored);
            if !rendered.is_empty() {
                eprint!("{rendered}");
            }
            let module = module.context("frontend failed")?;
            let runner = jit::Jit::new().with_context(|| "while setting up the JIT")?;
            // Compiled pickle_* calls need the heap etc. alive while we run.
            pickle_runtime::abi::pickle_runtime_init();
            let program = runner.compile(&module).with_context(|| "while JIT-compiling")?;
            // SAFETY: runtime is initialised and pickle_main is a no-arg
            // void-returning host-convention function.
            unsafe {
                program.run();
            }
            pickle_runtime::abi::pickle_runtime_shutdown();
        }
        Command::Build { file, output } => {
            let (source, mut map, diags) = load(file)?;
            let module = frontend(&file.display().to_string(), &source, &mut map, &diags)
                .and_then(|out| pickle_compiler::emit::emit_ir(&out.program, &out.resolved, &diags));
            let rendered = diags.render_all(&map, colored);
            if !rendered.is_empty() {
                eprint!("{rendered}");
            }
            let module = module.context("frontend failed")?;
            if module.funcs.iter().all(|f| !f.is_main) {
                anyhow::bail!("no `main` in this module; nothing to build");
            }
            let object = build::emit_object(&module).with_context(|| "while compiling to machine code")?;
            let out = output
                .clone()
                .unwrap_or_else(|| default_output(file));
            let target_dir = std::env::temp_dir().join("pickle-aot");
            let rlib = build::build_runtime_rlib(&target_dir)
                .with_context(|| "while building the runtime for AOT")?;
            build::link_object(&object, &target_dir, &rlib, &out)
                .with_context(|| format!("while linking {}", out.display()))?;
            println!("built {}", out.display());
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