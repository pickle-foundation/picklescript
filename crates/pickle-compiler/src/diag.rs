use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub file: FileId,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(file: FileId, start: usize, end: usize) -> Span {
        Span { file, start, end }
    }

    pub fn join(self, other: Span) -> Span {
        debug_assert!(self.file == other.file);
        Span {
            file: self.file,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn to(self, other: Span) -> Span {
        self.join(other)
    }

    pub fn point(self) -> Span {
        Span {
            file: self.file,
            start: self.start,
            end: self.start,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}-{}", self.file.0, self.start, self.end)
    }
}

pub struct SourceFile {
    pub name: String,
    pub text: Rc<str>,
    line_starts: Vec<usize>,
}

impl SourceFile {
    fn new(name: String, text: String) -> SourceFile {
        let mut line_starts = vec![0];
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        let text: Rc<str> = Rc::from(text);
        SourceFile {
            name,
            text,
            line_starts,
        }
    }

    pub fn line_of(&self, byte: usize) -> usize {
        match self.line_starts.binary_search(&byte) {
            Ok(i) => i,
            Err(i) => i - 1,
        }
    }

    /// Byte offset of the start of a line (0-based line index).
    pub fn line_start(&self, line: usize) -> usize {
        self.line_starts[line]
    }

    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    pub fn line_text(&self, line: usize) -> &str {
        let start = self.line_starts[line];
        let end = if line + 1 < self.line_starts.len() {
            self.line_starts[line + 1]
        } else {
            self.text.len()
        };
        self.text[start..end].trim_end_matches(|c| c == '\n' || c == '\r').trim_end()
    }
}

#[derive(Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn add(&mut self, name: String, text: String) -> FileId {
        let id = FileId(self.files.len());
        self.files.push(SourceFile::new(name, text));
        id
    }

    pub fn get(&self, id: FileId) -> &SourceFile {
        &self.files[id.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<(Option<Span>, String)>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            message: message.into(),
            span: None,
            notes: Vec::new(),
        }
    }

    pub fn error_at(span: Span, message: impl Into<String>) -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            message: message.into(),
            span: Some(span),
            notes: Vec::new(),
        }
    }

    pub fn warn_at(span: Span, message: impl Into<String>) -> Diagnostic {
        Diagnostic {
            severity: Severity::Warning,
            message: message.into(),
            span: Some(span),
            notes: Vec::new(),
        }
    }

    pub fn note(mut self, message: impl Into<String>) -> Diagnostic {
        self.notes.push((None, message.into()));
        self
    }

    pub fn note_at(mut self, span: Span, message: impl Into<String>) -> Diagnostic {
        self.notes.push((Some(span), message.into()));
        self
    }
}

/// A session-wide error sink. Collects diagnostics for one compile.
#[derive(Default)]
pub struct DiagnosticSink {
    pub diagnostics: Rc<RefCell<Vec<Diagnostic>>>,
    pub errors_abort: usize,
}

impl DiagnosticSink {
    pub fn new() -> DiagnosticSink {
        DiagnosticSink {
            diagnostics: Rc::new(RefCell::new(Vec::new())),
            errors_abort: 50,
        }
    }

    pub fn emit(&self, d: Diagnostic) {
        let abort = d.severity == Severity::Error;
        self.diagnostics.borrow_mut().push(d);
        if abort && self.any_error() {
            // count handled by has_errors
        }
        let _ = abort;
    }

    pub fn any_error(&self) -> bool {
        self.diagnostics
            .borrow()
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics
            .borrow()
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count()
    }

    pub fn too_many_errors(&self) -> bool {
        self.error_count() >= self.errors_abort
    }

    pub fn render_all(&self, map: &SourceMap, colored: bool) -> String {
        let mut out = String::new();
        for d in self.diagnostics.borrow().iter() {
            render_diagnostic(d, map, colored, &mut out);
            out.push('\n');
        }
        out
    }
}

fn render_diagnostic(d: &Diagnostic, map: &SourceMap, colored: bool, out: &mut String) {
    use std::fmt::Write;
    let (severity, sg, eg) = match d.severity {
        Severity::Error => ("error", "\x1b[1;31m", "\x1b[0m"),
        Severity::Warning => ("warning", "\x1b[1;33m", "\x1b[0m"),
    };

    match d.span {
        Some(span) => {
            let file = map.get(span.file);
            let line = file.line_of(span.start);
            let lline = line + 1;
            let col = span.start - file.line_start(line);
            if colored {
                let _ = write!(out, "{sg}{severity}{eg}");
            } else {
                out.push_str(severity);
            }
            let _ = write!(out, "{}{}:{}:{}: ", file.name, lline, col + 1, " ");
            let _ = write!(out, "{}\n", d.message);

            let text = file.line_text(line);
            let _ = write!(out, "  {lline} | {text}\n");
            let gutter = format!("    | ");
            out.push_str(&gutter);
            for _ in 0..col {
                out.push(' ');
            }
            let width = span
                .end
                .saturating_sub(span.start)
                .min(80)
                .max(1);
            for _ in 0..width {
                out.push('^');
            }
            for (nspan, note) in &d.notes {
                match nspan {
                    Some(nspan) => {
                        let line = file.line_of(nspan.start);
                        let col = nspan.start - file.line_start(line);
                        let ntext = file.line_text(line);
                        let _ = write!(out, "\n  {} | {ntext}", line + 1);
                        out.push('\n');
                        out.push_str(&gutter);
                        for _ in 0..col {
                            out.push(' ');
                        }
                        let _ = write!(out, "^ note: {note}");
                    }
                    None => {
                        let _ = write!(out, "\n  = note: {note}");
                    }
                }
            }
        }
        None => {
            if colored {
                let _ = write!(out, "{sg}{severity}{eg}");
            } else {
                out.push_str(severity);
            }
            let _ = write!(out, ": {}", d.message);
            for (_, note) in &d.notes {
                let _ = write!(out, "\n  = note: {note}");
            }
        }
    }
}