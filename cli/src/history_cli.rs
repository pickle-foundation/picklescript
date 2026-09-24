//! `pickle history`, `pickle builds`, and `pickle explain --history`.
//!
//! Versions are the git history of `*.pkl` files. For every commit that
//! touches a `.pkl` file we compile the file as it existed then and store a
//! `Snapshot` (public surface, dependency lines, build status, and error
//! codes) in the cache at `.pickle/history/snapshots.db`, keyed by the file
//! content hash. Later runs reuse cached snapshots, so history stays fast
//! and the shapes stay comparable across runs.
//!
//! Cache layout note: a single `snapshots.db` rather than
//! `symbols.db`/`dependencies.db`/`builds.db` — a snapshot carries all three
//! dimensions, and any content change invalidates the whole variant anyway,
//! so splitting would duplicate every line with no extra granularity.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use pickle_compiler::diag::{DiagnosticSink, SourceMap};
use pickle_compiler::history::{deps_of, diff_surfaces, extract_surface, fnv1a64};
use pickle_compiler::history::{Snapshot, SurfaceChange};

/// A commit together with the `*.pkl` files it touched, in commit order.
struct CommitEntry {
    commit: String,
    short: String,
    date: String,
    message: String,
    files: Vec<String>,
}

fn run_git(root: &Path, args: &[&str]) -> Result<String> {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .with_context(|| "spawning `git`")?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!("git {} failed: {stderr}", args.join(" ")));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Absolute path to the git work tree root, found by walking up from `cwd`.
pub fn git_root(cwd: &Path) -> Result<PathBuf> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(cwd)
        .output()
        .with_context(|| "spawning `git`")?;
    if !out.status.success() {
        return Err(anyhow!(
            "`{}` is not inside a git repository; compiler history is derived from git commits",
            cwd.display()
        ));
    }
    let root = String::from_utf8_lossy(&out.stdout).trim().to_string();
    Ok(PathBuf::from(root))
}

/// Every commit that touched a `*.pkl` file, oldest first, with the touched
/// files. Git's `--name-only` gives exactly the changed paths, so unchanged
/// files at a commit are not recomputed.
fn pkl_history(root: &Path) -> Result<Vec<CommitEntry>> {
    let out = run_git(
        root,
        &[
            "log",
            "--name-only",
            "--format=%x1f%H%x1f%ci%x1f%s",
            "--",
            "*.pkl",
        ],
    )?;
    let mut entries: Vec<CommitEntry> = Vec::new();
    for line in out.lines() {
        if let Some(rest) = line.strip_prefix('\u{1f}') {
            let mut fields = rest.split('\u{1f}');
            let commit = fields.next().unwrap_or_default().to_string();
            let date = fields.next().unwrap_or_default().to_string();
            let message = fields.next().unwrap_or_default().to_string();
            if commit.is_empty() {
                continue;
            }
            entries.push(CommitEntry {
                short: commit.chars().take(7).collect(),
                commit,
                date,
                message,
                files: Vec::new(),
            });
        } else if !line.trim().is_empty() && line.trim_end().ends_with(".pkl") {
            if let Some(last) = entries.last_mut() {
                last.files.push(line.trim_end().to_string());
            }
        }
    }
    entries.reverse();
    Ok(entries)
}

fn git_show(root: &Path, commit: &str, file: &str) -> Result<String> {
    run_git(root, &["show", &format!("{commit}:{file}")])
}

fn cache_path(root: &Path) -> PathBuf {
    root.join(".pickle").join("history").join("snapshots.db")
}

fn read_cache(root: &Path) -> Vec<Snapshot> {
    let path = cache_path(root);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    text.lines().filter_map(Snapshot::from_json).collect()
}

fn write_cache(root: &Path, snapshots: Vec<Snapshot>) -> Result<()> {
    let path = cache_path(root);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut body = snapshots;
    body.sort_by(|a, b| {
        (a.commit.as_str(), a.file.as_str()).cmp(&(b.commit.as_str(), b.file.as_str()))
    });
    let mut text = String::new();
    for s in &body {
        text.push_str(&s.to_json());
        text.push('\n');
    }
    std::fs::write(&path, text).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

/// Compile a file as it existed at a commit into a `Snapshot`. The frontend
/// covers lex/parse/resolve/check; codegen diagnostics (e.g. "not lowered
/// yet") come from `emit_ir`.
fn build_snapshot(e: &CommitEntry, file: &str, content: &str) -> Snapshot {
    let mut map = SourceMap::default();
    let diags = DiagnosticSink::new();
    let out = pickle_compiler::front::frontend_checked(file, content, &mut map, &diags);
    if let (Some(prog), Some(res)) = (
        out.as_ref().map(|o| &o.program),
        out.as_ref().map(|o| &o.resolved),
    ) {
        let _ = pickle_compiler::emit::emit_ir(prog, res, &diags);
    }
    let codes = {
        let ds = diags.diagnostics.borrow();
        let mut codes: Vec<String> = ds
            .iter()
            .filter_map(|d| d.code)
            .map(|c| c.id().to_string())
            .collect();
        codes.sort();
        codes.dedup();
        codes
    };
    let (items, deps) = match &out {
        Some(o) => (extract_surface(&o.program), deps_of(&o.program)),
        None => (Vec::new(), Vec::new()),
    };
    Snapshot {
        commit: e.commit.clone(),
        short: e.short.clone(),
        date: e.date.clone(),
        message: e.message.clone(),
        file: file.to_string(),
        hash: fnv1a64(content.as_bytes()),
        ok: !diags.any_error(),
        codes,
        deps,
        items,
    }
}

/// Walk the pkl history, ensuring a cached snapshot exists for every touched
/// file, and return everything we know, oldest first.
fn collect_snapshots(root: &Path) -> Result<Vec<Snapshot>> {
    let history = pkl_history(root)?;
    let mut cache: HashMap<(String, String), Snapshot> = read_cache(root)
        .into_iter()
        .map(|s| ((s.commit.clone(), s.file.clone()), s))
        .collect();
    let mut fresh: Vec<Snapshot> = Vec::new();
    for e in &history {
        for file in &e.files {
            let content = match git_show(root, &e.commit, file) {
                Ok(c) => c,
                // A path that never resolves should not abort the whole run.
                Err(_) => continue,
            };
            let hash = fnv1a64(content.as_bytes());
            let cached = cache.get(&(e.commit.clone(), file.clone())).cloned();
            let snapshot = if cached.as_ref().is_some_and(|s| s.hash == hash) {
                cached.expect("checked above")
            } else {
                let s = build_snapshot(e, file, &content);
                cache.insert((s.commit.clone(), s.file.clone()), s.clone());
                s
            };
            fresh.push(snapshot);
        }
    }
    if !fresh.is_empty() {
        let mut all: Vec<Snapshot> = cache.into_values().collect();
        all.extend(fresh.clone());
        write_cache(root, all)?;
    }
    // pkl_history() is oldest-first; fresh follows that order.
    Ok(fresh)
}

/// Compile-and-cache the whole pkl history, then return the snapshots.
pub fn history_for(root: &Path) -> Result<Vec<Snapshot>> {
    collect_snapshots(root)
}

fn fmt_commit(short: &str, date: &str, message: &str) -> String {
    format!("{short}  {date}  {}", message.lines().next().unwrap_or(""))
}

/// `pickle history [--file F] [--limit N]`: per file, the last `limit`
/// versions with the surface evolution between consecutive ones.
pub fn render_history(snapshots: &[Snapshot], file_filter: Option<&str>, limit: usize) {
    let mut files: BTreeSet<String> = snapshots.iter().map(|s| s.file.clone()).collect();
    if let Some(f) = file_filter {
        files.retain(|x| x == f);
    }
    let limit = limit.max(1);
    for file in files {
        let seq: Vec<&Snapshot> = snapshots.iter().filter(|s| s.file == file).collect();
        let shown: Vec<&Snapshot> = {
            let mut t: Vec<&Snapshot> = seq.iter().rev().take(limit).copied().collect();
            t.reverse();
            t
        };
        println!("history of {file} ({} version(s))", shown.len());
        let mut last: Option<&Snapshot> = None;
        for s in shown {
            println!("  {}", fmt_commit(&s.short, &s.date, &s.message));
            if let Some(prev) = last {
                if prev.deps != s.deps {
                    println!("  deps  {} -> {}", prev.deps.join(", "), s.deps.join(", "));
                }
                let mut lines = Vec::new();
                for change in diff_surfaces(&prev.items, &s.items) {
                    push_change(&mut lines, &change);
                }
                for line in lines {
                    println!("{line}");
                }
            }
            last = Some(s);
        }
        println!();
    }
}

/// One change row for a file's history:
/// `changed-type  User.name    name: string -> name: string?  (made nullable)`.
fn push_change(out: &mut Vec<String>, change: &SurfaceChange) {
    let sig = match (change.before.as_deref(), change.after.as_deref()) {
        (Some(b), Some(a)) => format!("{b} -> {a}"),
        (Some(b), None) | (None, Some(b)) => b.to_string(),
        (None, None) => String::new(),
    };
    let label = pickle_compiler::history::transit_label(
        change.before.as_deref().unwrap_or(""),
        change.after.as_deref().unwrap_or(""),
    )
    .map(|l| format!("  ({l})"))
    .unwrap_or_default();
    let status = match change.status {
        "+" => "added-type",
        "-" => "removed-type",
        _ => "changed-type",
    };
    out.push(format!("  {status:13} {}  {sig}{label}", change.name));
}

/// `pickle builds [--limit N]`: the most recent `limit` build rows (newest
/// first) with their status and error codes.
pub fn render_builds(snapshots: &[Snapshot], limit: usize) {
    let limit = limit.max(1);
    let mut rows: Vec<(&str, &str, &str, bool, &[String])> = snapshots
        .iter()
        .map(|s| {
            (
                s.date.as_str(),
                s.short.as_str(),
                s.file.as_str(),
                s.ok,
                s.codes.as_slice(),
            )
        })
        .collect();
    rows.sort_by(|a, b| (b.0, &b.2).cmp(&(a.0, &a.2)));
    rows.truncate(limit);
    println!("      commit  file                       ok    codes");
    let mut last_commit: Option<&str> = None;
    for (_, short, file, ok, codes) in rows {
        let mark = if ok { "ok" } else { "ERR" };
        let c = codes.join(" ");
        if last_commit == Some(short) && !short.is_empty() {
            println!("      {:<28} {:>4}  {c}", "", mark);
        } else {
            println!("  {}  {:<28} {:>4}  {c}", short, file, mark);
        }
        last_commit = Some(short);
    }
}

/// `pickle explain <CODE> --history`: every version (oldest first) where the
/// code fired.
pub fn render_explain_history(snapshots: &[Snapshot], code: &str) {
    let hits: Vec<&Snapshot> = snapshots
        .iter()
        .filter(|s| s.codes.iter().any(|c| c == code))
        .collect();
    if hits.is_empty() {
        println!(
            "error[{code}] has never been produced by any committed version of a `*.pkl` file"
        );
        return;
    }
    println!("error[{code}] -- produced by committed versions (oldest first):");
    for s in &hits {
        println!(
            "  {}  {}  {}",
            s.short,
            s.date,
            s.message.lines().next().unwrap_or("")
        );
    }
    println!("first seen: {} ({})", hits[0].short, hits[0].file);
}
