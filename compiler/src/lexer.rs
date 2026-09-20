use crate::diag::{Diagnostic, DiagnosticSink, FileId, Span};
use crate::token::{LexedToken, StrLit, StrSeg, Tok};

pub fn lex(file: FileId, src: &str, diags: &DiagnosticSink) -> Vec<LexedToken> {
    Lexer::new(file, src, diags).run()
}

struct Lexer<'a> {
    file: FileId,
    chars: Vec<char>,
    bytes: Vec<usize>, // byte offset of each char index
    pos: usize,
    nesting: usize,

    /// Doc comment text (`///` lines and `/** */` blocks) collected since the
    /// last real token; attached to the next non-trivia token.
    pending_doc: Option<String>,

    /// Paren/bracket nesting depth at the point each `{` was opened. A newline
    /// is significant (a statement boundary) at the top level of a block even
    /// when the block itself sits inside parentheses (e.g. the body of
    /// `test("...", { ... })`), while newlines inside parentheses/brackets
    /// *within* the block stay suppressed for line-continuation.
    block_frames: Vec<usize>,
// `src` is retained for diagnostics when reporting invalid characters.
    #[allow(dead_code)]
    src: &'a str,
    diags: &'a DiagnosticSink,
}

impl<'a> Lexer<'a> {
    fn new(file: FileId, src: &'a str, diags: &'a DiagnosticSink) -> Lexer<'a> {
        let chars: Vec<char> = src.chars().collect();
        let mut bytes = Vec::with_capacity(chars.len() + 1);
        let mut off = 0usize;
        for c in &chars {
            bytes.push(off);
            off += c.len_utf8();
        }
        bytes.push(src.len());
        Lexer {
            file,
            chars,
            bytes,
            pos: 0,
            nesting: 0,
            pending_doc: None,
            block_frames: Vec::new(),
            src,
            diags,
        }
    }

    fn peek(&self, ahead: usize) -> Option<char> {
        self.chars.get(self.pos + ahead).copied()
    }

    fn at(&self) -> Option<char> {
        self.peek(0)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.at();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn byte_of(&self, idx: usize) -> usize {
        self.bytes[idx.min(self.bytes.len() - 1)]
    }

    fn span(&self, start: usize, end: usize) -> Span {
        Span::new(self.file, self.byte_of(start), self.byte_of(end))
    }

    fn err(&self, span: Span, msg: impl Into<String>) {
        self.diags
            .emit(Diagnostic::error_at(span, msg).with_code(crate::error::ErrorCode::Lex));
    }

    fn append_doc(&mut self, text: String) {
        if text.is_empty() {
            return;
        }
        match &mut self.pending_doc {
            Some(d) => {
                d.push('\n');
                d.push_str(&text);
            }
            None => self.pending_doc = Some(text),
        }
    }

    fn run(mut self) -> Vec<LexedToken> {
        let mut out = Vec::new();
        loop {
            let t = self.next_token();
            let eof = matches!(t.token.kind, Tok::Eof);
            out.push(t);
            if eof {
                break;
            }
        }
        out
    }

    fn skip_ws_and_comments(&mut self) -> Span {
        // Returns a span covering skipped trivia start (for newline tokens we
        // emit the position of the newline itself).
        loop {
            match self.at() {
                Some(' ') | Some('\t') | Some('\r') => {
                    self.bump();
                }
                Some('\n') => return self.span(self.pos, self.pos),
                Some('/') if self.peek(1) == Some('/') => {
                    if self.peek(2) == Some('/') {
                        // `///` doc line: keep the text after the marker.
                        self.bump();
                        self.bump();
                        self.bump();
                        let mut text = String::new();
                        while let Some(c) = self.at() {
                            if c == '\n' {
                                break;
                            }
                            text.push(c);
                            self.bump();
                        }
                        let clean = text
                            .strip_prefix(' ')
                            .unwrap_or(&text)
                            .trim_end()
                            .to_string();
                        self.append_doc(clean);
                    } else {
                        while let Some(c) = self.at() {
                            if c == '\n' {
                                break;
                            }
                            self.bump();
                        }
                    }
                }
                Some('/') if self.peek(1) == Some('*') => {
                    let start = self.pos;
                    let is_doc = self.peek(2) == Some('*');
                    self.bump();
                    self.bump();
                    let mut depth = 1i32;
                    let mut text = String::new();
                    while depth > 0 {
                        match self.at() {
                            None => {
                                self.err(self.span(start, self.pos), "unterminated block comment");
                                break;
                            }
                            Some('/') if self.peek(1) == Some('*') => {
                                depth += 1;
                                self.bump();
                                self.bump();
                            }
                            Some('*') if self.peek(1) == Some('/') => {
                                depth -= 1;
                                self.bump();
                                self.bump();
                            }
                            Some(c) => {
                                text.push(c);
                                self.bump();
                            }
                        }
                    }
                    if is_doc {
                        if let Some(stripped) = text.strip_prefix('*') {
                            let body = stripped
                                .strip_prefix(' ')
                                .unwrap_or(stripped)
                                .trim_end()
                                .to_string();
                            self.append_doc(body);
                        }
                    }
                }
                _ => return self.span(self.pos, self.pos),
            }
        }
    }

    fn next_token(&mut self) -> LexedToken {
        self.skip_ws_and_comments();

        if self.at() == Some('\n') {
            // Newlines are significant tokens (statement boundaries) unless we
            // are inside parentheses/brackets. A block brace re-opens a
            // statement context, so a newline is suppressed only when an open
            // paren/bracket sits *within* the innermost block (or there is no
            // block at all).
            let start = self.pos;
            self.bump();
            let base = self.block_frames.last().copied().unwrap_or(0);
            if self.nesting <= base {
                return LexedToken::new(Tok::Newline, self.span(start, self.pos));
            }
            return self.next_token();
        }

        if self.at().is_none() {
            return LexedToken::new(Tok::Eof, self.span(self.pos, self.pos));
        }

        let mut t = self.lex_tok();
        if !matches!(t.token.kind, Tok::Newline | Tok::Eof) {
            t.data.doc = self.pending_doc.take();
        }
        t
    }

    fn lex_tok(&mut self) -> LexedToken {
        let start = self.pos;
        let c = self.bump().unwrap();

        // Raw string r"..." : no escapes, no interpolation.
        if c == 'r' && self.at() == Some('"') {
            return self.lex_raw_string(start);
        }

        // Identifier or keyword
        if c.is_ascii_alphabetic() || c == '_' {
            while let Some(ch) = self.at() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    self.bump();
                } else {
                    break;
                }
            }
            let span = self.span(start, self.pos);
            let word = &self.src[self.byte_of(start)..self.byte_of(self.pos)];
            let kind = Tok::keyword(word).unwrap_or_else(|| Tok::Ident(word.to_string()));
            return LexedToken::new(kind, span);
        }

        // Numbers
        if c.is_ascii_digit() {
            self.pos -= 1;
            return self.lex_number();
        }

        // Character literal
        if c == '\'' {
            self.pos -= 1;
            let t = self.lex_char();
            return t;
        }

        // String literals: `"..."` or `"""..."""`.
        if c == '"' {
            if self.at() == Some('"') && self.peek(1) == Some('"') {
                self.pos -= 1;
                return self.lex_multiline_string();
            }
            self.pos -= 1;
            return self.lex_string();
        }

        match c {
            '(' => {
                self.nesting += 1;
                LexedToken::new(Tok::LParen, self.span(start, start + 1))
            }
            ')' => {
                self.nesting = self.nesting.saturating_sub(1);
                LexedToken::new(Tok::RParen, self.span(start, start + 1))
            }
            '[' => {
                self.nesting += 1;
                LexedToken::new(Tok::LBracket, self.span(start, start + 1))
            }
            ']' => {
                self.nesting = self.nesting.saturating_sub(1);
                LexedToken::new(Tok::RBracket, self.span(start, start + 1))
            }
            '{' => {
                self.block_frames.push(self.nesting);
                LexedToken::new(Tok::LBrace, self.span(start, start + 1))
            }
            '}' => {
                self.block_frames.pop();
                LexedToken::new(Tok::RBrace, self.span(start, start + 1))
            }
            ',' => LexedToken::new(Tok::Comma, self.span(start, start + 1)),
            '.' => {
                if self.at() == Some('.') {
                    self.bump();
                    if self.at() == Some('.') {
                        self.bump();
                        return LexedToken::new(Tok::Ellipsis, self.span(start, self.pos));
                    }
                    if self.at() == Some('=') {
                        self.bump();
                        return LexedToken::new(Tok::RangeIncl, self.span(start, self.pos));
                    }
                    return LexedToken::new(Tok::Range, self.span(start, self.pos));
                }
                if matches!(self.at(), Some(d) if d.is_ascii_digit()) {
                    // leading-dot float, e.g. `.5`
                    let mut text = String::new();
                    text.push('.');
                    while let Some(c) = self.at() {
                        if c.is_ascii_digit() || c == '_' {
                            text.push(self.bump().unwrap());
                        } else {
                            break;
                        }
                    }
                    LexedToken::number(text, self.span(start, self.pos))
                } else {
                    LexedToken::new(Tok::Dot, self.span(start, start + 1))
                }
            }
            ':' => {
                if self.at() == Some(':') {
                    self.bump();
                    return LexedToken::new(Tok::ColonColon, self.span(start, self.pos));
                }
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::Assign, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Colon, self.span(start, start + 1))
            }
            ';' => LexedToken::new(Tok::Semicolon, self.span(start, start + 1)),
            '=' => {
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::EqEq, self.span(start, self.pos));
                }
                if self.at() == Some('>') {
                    self.bump();
                    return LexedToken::new(Tok::FatArrow, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Assign, self.span(start, start + 1))
            }
            '-' => {
                if self.at() == Some('>') {
                    self.bump();
                    return LexedToken::new(Tok::Arrow, self.span(start, self.pos));
                }
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::MinusEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Minus, self.span(start, start + 1))
            }
            '+' => {
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::PlusEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Plus, self.span(start, start + 1))
            }
            '*' => {
                if self.at() == Some('*') {
                    self.bump();
                    return LexedToken::new(Tok::StarStar, self.span(start, self.pos));
                }
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::StarEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Star, self.span(start, start + 1))
            }
            '/' => {
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::SlashEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Slash, self.span(start, start + 1))
            }
            '%' => {
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::PercentEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Percent, self.span(start, start + 1))
            }
            '!' => {
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::NotEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Bang, self.span(start, start + 1))
            }
            '~' => LexedToken::new(Tok::Tilde, self.span(start, start + 1)),
            '#' => LexedToken::new(Tok::Hash, self.span(start, start + 1)),
            '&' => {
                if self.at() == Some('&') {
                    self.bump();
                    return LexedToken::new(Tok::AndAnd, self.span(start, self.pos));
                }
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::AndEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Amp, self.span(start, start + 1))
            }
            '|' => {
                if self.at() == Some('|') {
                    self.bump();
                    return LexedToken::new(Tok::OrOr, self.span(start, self.pos));
                }
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::OrEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Pipe, self.span(start, start + 1))
            }
            '^' => {
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::XorEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Caret, self.span(start, start + 1))
            }
            '<' => {
                if self.at() == Some('<') {
                    self.bump();
                    if self.at() == Some('=') {
                        self.bump();
                        return LexedToken::new(Tok::ShlEq, self.span(start, self.pos));
                    }
                    return LexedToken::new(Tok::Shl, self.span(start, self.pos));
                }
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::Le, self.span(start, self.pos));
                }
                if self.at() == Some('-') {
                    self.bump();
                    return LexedToken::new(Tok::SendOp, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Lt, self.span(start, start + 1))
            }
            '>' => {
                if self.at() == Some('>') {
                    self.bump();
                    if self.at() == Some('=') {
                        self.bump();
                        return LexedToken::new(Tok::ShrEq, self.span(start, self.pos));
                    }
                    return LexedToken::new(Tok::Shr, self.span(start, self.pos));
                }
                if self.at() == Some('=') {
                    self.bump();
                    return LexedToken::new(Tok::Ge, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Gt, self.span(start, start + 1))
            }
            '?' => {
                if self.at() == Some('?') {
                    self.bump();
                    return LexedToken::new(Tok::QuestionQuestion, self.span(start, self.pos));
                }
                if self.at() == Some('.') {
                    self.bump();
                    return LexedToken::new(Tok::QuestionDot, self.span(start, self.pos));
                }
                if self.at() == Some(':') {
                    self.bump();
                    return LexedToken::new(Tok::QuestionColon, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Question, self.span(start, start + 1))
            }
            '@' => LexedToken::new(Tok::At, self.span(start, start + 1)),
            _ => {
                // Report the invalid character, skip it, and keep scanning so a
                // file with a stray character still yields its other tokens and
                // more than one diagnostic.
                let span = self.span(start, start + 1);
                self.err(span, format!("unexpected character `{c}`"));
                self.next_token()
            }
        }
    }

    fn lex_number(&mut self) -> LexedToken {
        let start = self.pos;
        let mut text = String::new();

        // Radix prefixes
        let mut radix = 10u32;
        if self.at() == Some('0')
            && matches!(
                self.peek(1),
                Some('x') | Some('X') | Some('b') | Some('B') | Some('o') | Some('O')
            )
        {
            let p = self.peek(1).unwrap();
            self.bump();
            self.bump();
            radix = match p {
                'x' | 'X' => 16,
                'b' | 'B' => 2,
                _ => 8,
            };
            text.push('0');
            text.push(p);
        }

        let mut is_float = false;
        if radix == 10 {
            while let Some(c) = self.at() {
                if c != '.' && c != 'e' && c != 'E' && !c.is_ascii_digit() && c != '_' {
                    break;
                }
                if c == '.' {
                    // Don't treat `.` as part of number if already float or next is ident-ish
                    if is_float {
                        break;
                    }
                    if matches!(self.peek(1), Some(n) if n.is_ascii_digit()) {
                        is_float = true;
                        text.push(self.bump().unwrap());
                        continue;
                    }
                    break;
                }
                if c == 'e' || c == 'E' {
                    // exponent only if followed by digits or sign+digit
                    let n = self.peek(1);
                    let n2 = self.peek(2);
                    if matches!(n, Some(d) if d.is_ascii_digit())
                        || matches!(
                            (n, n2),
                            (Some('+') | Some('-'), Some(d)) if d.is_ascii_digit()
                        )
                    {
                        is_float = true;
                        text.push(self.bump().unwrap());
                        if self.at() == Some('+') || self.at() == Some('-') {
                            text.push(self.bump().unwrap());
                        }
                        continue;
                    }
                    break;
                }
                text.push(self.bump().unwrap());
            }
        } else {
            while let Some(c) = self.at() {
                if c.is_ascii_hexdigit() || c == '_' {
                    text.push(self.bump().unwrap());
                } else {
                    break;
                }
            }
        }

        // Suffix (e.g. `7u8`, `-2i32`, `2.5f32`). Alphanumeric so a full typed
        // suffix like `u8` or `i32` is captured in one token.
        let mut suffix = String::new();
        while let Some(c) = self.at() {
            if c.is_ascii_alphanumeric() {
                suffix.push(self.bump().unwrap());
            } else {
                break;
            }
        }
        let span = self.span(start, self.pos);
        let mut t = LexedToken::number(text.clone(), span);
        t.data.suffix = Some(suffix);
        t.data.is_float = is_float;
        t
    }

    fn lex_raw_string(&mut self, start: usize) -> LexedToken {
        self.bump(); // consume the opening "
        let mut text = String::new();
        loop {
            match self.at() {
                Some('"') => {
                    self.bump();
                    let lit = StrLit {
                        segments: vec![StrSeg::Text { text }],
                    };
                    return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                }
                Some(c) => {
                    text.push(c);
                    self.bump();
                }
                None => {
                    self.err(self.span(start, self.pos), "unterminated raw string literal");
                    let lit = StrLit {
                        segments: vec![StrSeg::Text { text }],
                    };
                    return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                }
            }
        }
    }

    fn lex_multiline_string(&mut self) -> LexedToken {
        let start = self.pos;
        self.bump();
        self.bump();
        self.bump(); // consume """
        let mut segments = Vec::new();
        loop {
            let mut text = String::new();
            loop {
                match self.at() {
                    Some('"') if self.peek(1) == Some('"') && self.peek(2) == Some('"') => {
                        self.bump();
                        self.bump();
                        self.bump();
                        if !text.is_empty() {
                            segments.push(StrSeg::Text { text });
                        }
                        let lit = StrLit { segments };
                        return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                    }
                    Some('{') if self.peek(1) != Some('{') => {
                        // interpolation
                        self.bump();
                        if !text.is_empty() {
                            segments.push(StrSeg::Text { text });
                        }
                        let tok_start = self.pos;
                        let expr = self.lex_interpolated_tokens(tok_start);
                        segments.push(StrSeg::Expr { tokens: expr });
                        break;
                    }
                    Some('{') => {
                        // escaped {{
                        self.bump();
                        self.bump();
                        text.push('{');
                    }
                    Some('}') if self.peek(1) == Some('}') => {
                        self.bump();
                        self.bump();
                        text.push('}');
                    }
                    Some('\\') => self.string_escape(&mut text),
                    Some(c) => {
                        text.push(c);
                        self.bump();
                    }
                    None => {
                        self.err(self.span(start, self.pos), "unterminated multi-line string literal");
                        segments.push(StrSeg::Text { text });
                        let lit = StrLit { segments };
                        return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                    }
                }
            }
        }
    }

    /// Consumes `\X` (or `\u{...}`) and appends the decoded character to
    /// `text`, reporting a diagnostic for unknown or malformed escapes.
    fn string_escape(&mut self, text: &mut String) {
        self.bump(); // consume '\'
        let esc_start = self.pos;
        match self.bump() {
            Some('n') => text.push('\n'),
            Some('t') => text.push('\t'),
            Some('r') => text.push('\r'),
            Some('\\') => text.push('\\'),
            Some('"') => text.push('"'),
            Some('{') => text.push('{'),
            Some('}') => text.push('}'),
            Some('0') => text.push('\0'),
            Some('u') => {
                if self.at() == Some('{') {
                    self.bump();
                    let mut hex = String::new();
                    let hex_start = self.pos;
                    while let Some(h) = self.at() {
                        if h.is_ascii_hexdigit() {
                            hex.push(h);
                            self.bump();
                        } else {
                            break;
                        }
                    }
                    let esc_span = self.span(hex_start, self.pos);
                    if self.at() == Some('}') {
                        self.bump();
                        match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                            Some(ch) => text.push(ch),
                            None => self.err(esc_span, format!("invalid unicode escape '\\u{{{hex}}}'")),
                        }
                    } else {
                        self.err(esc_span, "unterminated unicode escape".to_string());
                    }
                } else {
                    self.err(
                        self.span(esc_start, self.pos),
                        "malformed unicode escape, expected \\u{...}".to_string(),
                    );
                }
            }
            Some(other) => {
                self.err(
                    self.span(esc_start, self.pos),
                    format!("unknown string escape `\\{other}`"),
                );
            }
            None => {
                // A lone trailing backslash: the enclosing string loop reports
                // the unterminated literal.
            }
        }
    }

    fn lex_char(&mut self) -> LexedToken {
        let start = self.pos;
        self.bump(); // consume '
        let mut value = None;
        let mut err = None;
        match self.at() {
            Some('\'') | None => err = Some("empty character literal".to_string()),
            Some('\\') => {
                self.bump(); // consume backslash
                let esc_start = self.pos;
                match self.bump() {
                    Some('n') => value = Some('\n'),
                    Some('t') => value = Some('\t'),
                    Some('r') => value = Some('\r'),
                    Some('\\') => value = Some('\\'),
                    Some('\'') => value = Some('\''),
                    Some('"') => value = Some('"'),
                    Some('0') => value = Some('\0'),
                    Some('u') => {
                        if self.at() == Some('{') {
                            self.bump();
                            let mut hex = String::new();
                            while let Some(h) = self.at() {
                                if h.is_ascii_hexdigit() {
                                    hex.push(h);
                                    self.bump();
                                } else {
                                    break;
                                }
                            }
                            if self.at() == Some('}') {
                                self.bump();
                                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                                    Some(ch) => value = Some(ch),
                                    None => {
                                        err = Some(format!("invalid unicode escape '\\u{{{hex}}}'"))
                                    }
                                }
                            } else {
                                err = Some("unterminated unicode escape".to_string());
                            }
                        } else {
                            err = Some("malformed unicode escape, expected \\u{...}".to_string());
                        }
                    }
                    Some(other) => err = Some(format!("unknown escape `\\{other}`")),
                    None => err = Some("unterminated escape".to_string()),
                }
                let _ = esc_start;
            }
            Some(c) => {
                value = Some(c);
                self.bump();
            }
        }
        if self.at() != Some('\'') {
            err = Some("unterminated character literal".to_string());
        } else {
            self.bump();
        }
        let close = self.span(start, self.pos);
        if let Some(e) = err {
            self.err(close, e);
        }
        let mut t = LexedToken::new(Tok::Char(value.unwrap_or('\0')), close);
        t.data.int = value.map(|c| c as u32 as i128);
        t
    }

    fn lex_string(&mut self) -> LexedToken {
        let start = self.pos;
        self.bump(); // consume opening "
        let mut segments = Vec::new();
        loop {
            let mut text = String::new();
            loop {
                match self.at() {
                    Some('"') => {
                        self.bump();
                        if !text.is_empty() {
                            segments.push(StrSeg::Text { text });
                        }
                        let lit = StrLit { segments };
                        return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                    }
                    Some('{') if self.peek(1) != Some('{') => {
                        // interpolation
                        self.bump();
                        if !text.is_empty() {
                            segments.push(StrSeg::Text { text });
                        }
                        let tok_start = self.pos;
                        let expr = self.lex_interpolated_tokens(tok_start);
                        segments.push(StrSeg::Expr { tokens: expr });
                        break;
                    }
                    Some('{') => {
                        // escaped {{
                        self.bump();
                        self.bump();
                        text.push('{');
                    }
                    Some('}') if self.peek(1) == Some('}') => {
                        self.bump();
                        self.bump();
                        text.push('}');
                    }
                    Some('\\') => self.string_escape(&mut text),
                    Some(c) => {
                        text.push(c);
                        self.bump();
                    }
                    None => {
                        self.err(self.span(start, self.pos), "unterminated string literal");
                        segments.push(StrSeg::Text { text });
                        let lit = StrLit { segments };
                        return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                    }
                }
            }
        }
    }

    /// Tokenize an interpolated expression `{ ... }` up to the matching `}`.
    /// Delegates to the main tokenizer so interpolation regions accept the full
    /// operator set (`<<`, `>>`, `..=`, `?:`, `<-`, `...`, ...), tracking the
    /// brace depth of nested `{`/`}` so only the interpolated expression's own
    /// closing brace is consumed here.
    fn lex_interpolated_tokens(&mut self, start: usize) -> Vec<LexedToken> {
        let mut out = Vec::new();
        let mut brace_depth = 1usize;
        let saved_nesting = self.nesting;
        let saved_frames_len = self.block_frames.len();
        let saved_doc = self.pending_doc.take();
        self.nesting = 0;
        loop {
            self.skip_ws_and_comments();
            if self.at() == Some('}') && brace_depth == 1 {
                self.bump();
                break;
            }
            if self.at().is_none() {
                self.err(self.span(start, self.pos), "unterminated interpolation expression");
                break;
            }
            let t = self.next_token();
            match t.token.kind {
                Tok::LBrace => brace_depth += 1,
                Tok::RBrace => brace_depth -= 1,
                Tok::Newline => continue,
                Tok::Eof => {
                    self.err(self.span(start, self.pos), "unterminated interpolation expression");
                    break;
                }
                _ => {}
            }
            out.push(t);
        }
        self.nesting = saved_nesting;
        while self.block_frames.len() > saved_frames_len {
            self.block_frames.pop();
        }
        self.pending_doc = saved_doc;
        out.push(LexedToken::new(Tok::Eof, self.span(self.pos, self.pos)));
        out
    }
}