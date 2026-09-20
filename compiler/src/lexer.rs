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
            block_frames: Vec::new(),
            src,
            diags,
        }
    }

    fn peek(&self, ahead: usize) -> char {
        self.chars.get(self.pos + ahead).copied().unwrap_or('\0')
    }

    fn at(&self) -> char {
        self.peek(0)
    }

    fn bump(&mut self) -> char {
        let c = self.at();
        if c != '\0' {
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
                ' ' | '\t' | '\r' => {
                    self.bump();
                }
                '\n' => return self.span(self.pos, self.pos),
                '/' if self.peek(1) == '/' => {
                    while self.at() != '\n' && self.at() != '\0' {
                        self.bump();
                    }
                }
                '/' if self.peek(1) == '*' => {
                    let start = self.pos;
                    self.bump();
                    self.bump();
                    let mut depth = 1;
                    while depth > 0 && self.at() != '\0' {
                        if self.at() == '/' && self.peek(1) == '*' {
                            depth += 1;
                            self.bump();
                            self.bump();
                        } else if self.at() == '*' && self.peek(1) == '/' {
                            depth -= 1;
                            self.bump();
                            self.bump();
                        } else {
                            self.bump();
                        }
                    }
                    let _ = start;
                }
                _ => return self.span(self.pos, self.pos),
            }
        }
    }

    fn next_token(&mut self) -> LexedToken {
        self.skip_ws_and_comments();

        if self.at() == '\n' {
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

        if self.at() == '\0' {
            return LexedToken::new(Tok::Eof, self.span(self.pos, self.pos));
        }

        let start = self.pos;
        let c = self.bump();

        // Identifier or keyword
        if c.is_ascii_alphabetic() || c == '_' {
            let mut ident = String::new();
            ident.push(c);
            while self.at().is_ascii_alphanumeric() || self.at() == '_' {
                ident.push(self.bump());
            }
            let span = self.span(start, self.pos);
            let kind = match std::str::from_utf8(ident.as_bytes()).ok() {
                Some(s) => {
                    if let Some((_, kw)) = Tok::keyword().iter().find(|(k, _)| *k == s) {
                        kw.clone()
                    } else {
                        Tok::Ident(ident.clone())
                    }
                }
                None => Tok::Ident(ident.clone()),
            };
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

        // String literal
        if c == '"' {
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
                if self.at() == '.' {
                    self.bump();
                    if self.at() == '=' {
                        self.bump();
                        return LexedToken::new(Tok::RangeIncl, self.span(start, self.pos));
                    }
                    return LexedToken::new(Tok::Range, self.span(start, self.pos));
                }
                if self.at().is_ascii_digit() {
                    // leading-dot float, e.g. `.5`
                    self.pos -= 0;
                    let mut text = String::new();
                    text.push('.');
                    while self.at().is_ascii_digit() || self.at() == '_' {
                        text.push(self.bump());
                    }
                    LexedToken::number(text, self.span(start, self.pos))
                } else {
                    LexedToken::new(Tok::Dot, self.span(start, start + 1))
                }
            }
            ':' => {
                if self.at() == ':' {
                    self.bump();
                    return LexedToken::new(Tok::ColonColon, self.span(start, self.pos));
                }
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::Assign, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Colon, self.span(start, start + 1))
            }
            ';' => LexedToken::new(Tok::Semicolon, self.span(start, start + 1)),
            '=' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::EqEq, self.span(start, self.pos));
                }
                if self.at() == '>' {
                    self.bump();
                    return LexedToken::new(Tok::FatArrow, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Assign, self.span(start, start + 1))
            }
            '-' => {
                if self.at() == '>' {
                    self.bump();
                    return LexedToken::new(Tok::Arrow, self.span(start, self.pos));
                }
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::MinusEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Minus, self.span(start, start + 1))
            }
            '+' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::PlusEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Plus, self.span(start, start + 1))
            }
            '*' => {
                if self.at() == '*' {
                    self.bump();
                    return LexedToken::new(Tok::StarStar, self.span(start, self.pos));
                }
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::StarEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Star, self.span(start, start + 1))
            }
            '/' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::SlashEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Slash, self.span(start, start + 1))
            }
            '%' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::PercentEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Percent, self.span(start, start + 1))
            }
            '!' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::NotEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Bang, self.span(start, start + 1))
            }
            '~' => LexedToken::new(Tok::Tilde, self.span(start, start + 1)),
            '#' => LexedToken::new(Tok::Hash, self.span(start, start + 1)),
            '&' => {
                if self.at() == '&' {
                    self.bump();
                    return LexedToken::new(Tok::AndAnd, self.span(start, self.pos));
                }
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::AndEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Amp, self.span(start, start + 1))
            }
            '|' => {
                if self.at() == '|' {
                    self.bump();
                    return LexedToken::new(Tok::OrOr, self.span(start, self.pos));
                }
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::OrEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Pipe, self.span(start, start + 1))
            }
            '^' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::XorEq, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Caret, self.span(start, start + 1))
            }
            '<' => {
                if self.at() == '<' {
                    self.bump();
                    if self.at() == '=' {
                        self.bump();
                        return LexedToken::new(Tok::ShlEq, self.span(start, self.pos));
                    }
                    return LexedToken::new(Tok::Shl, self.span(start, self.pos));
                }
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::Le, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Lt, self.span(start, start + 1))
            }
            '>' => {
                if self.at() == '>' {
                    self.bump();
                    if self.at() == '=' {
                        self.bump();
                        return LexedToken::new(Tok::ShrEq, self.span(start, self.pos));
                    }
                    return LexedToken::new(Tok::Shr, self.span(start, self.pos));
                }
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::Ge, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Gt, self.span(start, start + 1))
            }
            '?' => {
                if self.at() == '?' {
                    self.bump();
                    return LexedToken::new(Tok::QuestionQuestion, self.span(start, self.pos));
                }
                if self.at() == '.' {
                    self.bump();
                    return LexedToken::new(Tok::QuestionDot, self.span(start, self.pos));
                }
                if self.at() == ':' {
                    self.bump();
                    return LexedToken::new(Tok::QuestionColon, self.span(start, self.pos));
                }
                LexedToken::new(Tok::Question, self.span(start, start + 1))
            }
            '@' => LexedToken::new(Tok::At, self.span(start, start + 1)),
            _ => {
                let span = self.span(start, start + 1);
                self.err(span, format!("unexpected character `{c}`"));
                LexedToken::new(Tok::Eof, span)
            }
        }
    }

    fn lex_number(&mut self) -> LexedToken {
        let start = self.pos;
        let mut text = String::new();

        // Radix prefixes
        let mut radix = 10u32;
        if self.at() == '0' && matches!(self.peek(1), 'x' | 'X' | 'b' | 'B' | 'o' | 'O') {
            let p = self.peek(1);
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
            while self.at().is_ascii_digit() || self.at() == '_' || self.at() == '.'
                || (matches!(self.at(), 'e' | 'E'))
            {
                let c = self.at();
                if c == '.' {
                    // Don't treat `.` as part of number if already float or next is ident-ish
                    if is_float {
                        break;
                    }
                    if self.peek(1).is_ascii_digit() {
                        is_float = true;
                        text.push(self.bump());
                        continue;
                    }
                    break;
                }
                if c == 'e' || c == 'E' {
                    // exponent only if followed by digits or sign+digit
                    let n = self.peek(1);
                    let n2 = self.peek(2);
                    if n.is_ascii_digit() || ((n == '+' || n == '-') && n2.is_ascii_digit()) {
                        is_float = true;
                        text.push(self.bump());
                        if self.at() == '+' || self.at() == '-' {
                            text.push(self.bump());
                        }
                        continue;
                    }
                    break;
                }
                text.push(self.bump());
            }
        } else {
            while self.at().is_ascii_hexdigit() || self.at() == '_' {
                text.push(self.bump());
            }
        }

        // Suffix
        let mut suffix = String::new();
        while self.at().is_ascii_alphabetic() {
            suffix.push(self.bump());
        }
        let span = self.span(start, self.pos);
        let mut t = LexedToken::number(text.clone(), span);
        t.data.suffix = Some(suffix);
        t.data.is_float = is_float;
        t
    }

    fn lex_char(&mut self) -> LexedToken {
        let start = self.pos;
        self.bump(); // consume '
        let mut value = None;
        let mut err = None;
        match self.at() {
            '\'' | '\0' => err = Some("empty character literal".to_string()),
            '\\' => {
                self.bump();
                let c = self.at();
                match c {
                    'n' => value = Some('\n'),
                    't' => value = Some('\t'),
                    'r' => value = Some('\r'),
                    '\\' => value = Some('\\'),
                    '\'' => value = Some('\''),
                    '"' => value = Some('"'),
                    '0' => value = Some('\0'),
                    'u' => {
                        if self.peek(1) == '{' {
                            self.bump();
                            self.bump();
                            let mut hex = String::new();
                            while self.at().is_ascii_hexdigit() {
                                hex.push(self.bump());
                            }
                            if self.at() == '}' {
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
                    other => err = Some(format!("unknown escape `\\{other}`")),
                }
            }
            c => {
                value = Some(c);
                self.bump();
            }
        }
        let _span = self.span(start, self.pos.max(start + 1));
        if self.at() != '\'' {
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

        // raw string r"..." : no escapes, no interpolation
        // handled by caller? We consumed `"` only; raw is r" prefix.
        loop {
            let mut text = String::new();
            loop {
                let c = self.at();
                match c {
                    '"' => {
                        self.bump();
                        if !text.is_empty() {
                            segments.push(StrSeg::Text { text });
                        }
                        let lit = StrLit { segments };
                        return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                    }
                    '{' if self.peek(1) != '{' => {
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
                    '{' => {
                        // escaped {{
                        self.bump();
                        self.bump();
                        text.push('{');
                    }
                    '}' if self.peek(1) == '}' => {
                        self.bump();
                        self.bump();
                        text.push('}');
                    }
                    '\\' => {
                        self.bump();
                        let mut ok = true;
                        let esc = self.at();
                        match esc {
                            'n' => text.push('\n'),
                            't' => text.push('\t'),
                            'r' => text.push('\r'),
                            '\\' => text.push('\\'),
                            '"' => text.push('"'),
                            '{' => text.push('{'),
                            '}' => text.push('}'),
                            '0' => text.push('\0'),
                            'u' => {
                                if self.peek(1) == '{' {
                                    self.bump();
                                    self.bump();
                                    let mut hex = String::new();
                                    let hex_start = self.pos;
                                    while self.at().is_ascii_hexdigit() {
                                        hex.push(self.bump());
                                    }
                                    let esc_span = self.span(hex_start, self.pos);
                                    if self.at() == '}' {
                                        self.bump();
                                        match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                                            Some(ch) => text.push(ch),
                                            None => self.err(esc_span, format!("invalid unicode escape '\\u{{{hex}}}'")),
                                        }
                                    } else {
                                        self.err(esc_span, "unterminated unicode escape".to_string());
                                    }
                                } else {
                                    ok = false;
                                }
                            }
                            _ => ok = false,
                        }
                        if !ok {
                            self.err(
                                self.span(self.pos, self.pos + 1),
                                format!("unknown string escape `\\{esc}`"),
                            );
                        }
                    }
                    '\0' => {
                        self.err(self.span(start, self.pos), "unterminated string literal");
                        segments.push(StrSeg::Text { text });
                        let lit = StrLit { segments };
                        return LexedToken::new(Tok::Str(lit), self.span(start, self.pos));
                    }
                    c => {
                        text.push(c);
                        self.bump();
                    }
                }
            }
        }
    }

    /// Tokenize an interpolated expression `{ ... }` up to the matching `}`.
    fn lex_interpolated_tokens(&mut self, start: usize) -> Vec<LexedToken> {
        let mut out = Vec::new();
        let mut brace_depth = 1usize;
        let saved_nesting = self.nesting;
        self.nesting = 0;
        loop {
            self.skip_ws_and_comments();
            let s = self.pos;
            let c = self.at();
            match c {
                '\n' => {
                    self.bump();
                    // expressions cannot span statements; skip newlines
                }
                '{' => {
                    brace_depth += 1;
                    self.bump();
                }
                '}' => {
                    self.bump();
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        break;
                    }
                }
                '\0' => {
                    self.nesting = saved_nesting;
                    self.err(self.span(start, self.pos), "unterminated interpolation expression");
                    return out;
                }
                _ => {
                    // Reuse main tokenizer but re-interpret: simplest path is to
                    // delegate a small sub-lex pass by scanning one token via the main
                    // machinery. To avoid code duplication we tokenize a whole
                    // buffer; instead, we inline a minimal token scan here by calling
                    // next_token() — but next_token skips ws and returns Eof handling
                    // braces incorrectly. So we special-case braces here and desk.
                    // We implement a small scanner for the common operators:
                    let _ = s;
                    out.push(self.lex_one_template_token());
                }
            }
        }
        self.nesting = saved_nesting;
        out.push(LexedToken::new(Tok::Eof, self.span(self.pos, self.pos)));
        out
    }

    /// Single-token scanner used for interpolation regions (no newlines/braces).
    fn lex_one_template_token(&mut self) -> LexedToken {
        let start = self.pos;
        let c = self.bump();
        if c.is_ascii_alphabetic() || c == '_' {
            let mut ident = String::new();
            ident.push(c);
            while self.at().is_ascii_alphanumeric() || self.at() == '_' {
                ident.push(self.bump());
            }
            let span = self.span(start, self.pos);
            if let Ok(s) = std::str::from_utf8(ident.as_bytes()) {
                if let Some((_, kw)) = Tok::keyword().iter().find(|(k, _)| *k == s) {
                    return LexedToken::new(kw.clone(), span);
                }
            }
            return LexedToken::new(Tok::Ident(ident), span);
        }
        if c.is_ascii_digit() {
            self.pos -= 1;
            return self.lex_number();
        }
        if c == '"' {
            self.pos -= 1;
            return self.lex_string();
        }
        if c == '\'' {
            self.pos -= 1;
            return self.lex_char();
        }
        let span = self.span(start, start + 1);
        let kind = match c {
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            '[' => Tok::LBracket,
            ']' => Tok::RBracket,
            ',' => Tok::Comma,
            '.' => {
                if self.at() == '.' {
                    self.bump();
                    return LexedToken::new(Tok::Range, self.span(start, self.pos));
                }
                Tok::Dot
            }
            ':' => {
                if self.at() == ':' {
                    self.bump();
                    return LexedToken::new(Tok::ColonColon, self.span(start, self.pos));
                }
                Tok::Colon
            }
            '=' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::EqEq, self.span(start, self.pos));
                }
                if self.at() == '>' {
                    self.bump();
                    return LexedToken::new(Tok::FatArrow, self.span(start, self.pos));
                }
                Tok::Assign
            }
            '-' => {
                if self.at() == '>' {
                    self.bump();
                    return LexedToken::new(Tok::Arrow, self.span(start, self.pos));
                }
                Tok::Minus
            }
            '+' => Tok::Plus,
            '*' => {
                if self.at() == '*' {
                    self.bump();
                    return LexedToken::new(Tok::StarStar, self.span(start, self.pos));
                }
                Tok::Star
            }
            '/' => Tok::Slash,
            '%' => Tok::Percent,
            '!' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::NotEq, self.span(start, self.pos));
                }
                Tok::Bang
            }
            '&' => {
                if self.at() == '&' {
                    self.bump();
                    return LexedToken::new(Tok::AndAnd, self.span(start, self.pos));
                }
                Tok::Amp
            }
            '|' => {
                if self.at() == '|' {
                    self.bump();
                    return LexedToken::new(Tok::OrOr, self.span(start, self.pos));
                }
                Tok::Pipe
            }
            '^' => Tok::Caret,
            '?' => {
                if self.at() == '?' {
                    self.bump();
                    return LexedToken::new(Tok::QuestionQuestion, self.span(start, self.pos));
                }
                if self.at() == '.' {
                    self.bump();
                    return LexedToken::new(Tok::QuestionDot, self.span(start, self.pos));
                }
                Tok::Question
            }
            '<' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::Le, self.span(start, self.pos));
                }
                if self.at() == '-' {
                    self.bump();
                    return LexedToken::new(Tok::SendOp, self.span(start, self.pos));
                }
                Tok::Lt
            }
            '>' => {
                if self.at() == '=' {
                    self.bump();
                    return LexedToken::new(Tok::Ge, self.span(start, self.pos));
                }
                Tok::Gt
            }
            '~' => Tok::Tilde,
            ';' => Tok::Newline, // tolerate stray semicolons as statement separators
            _ => {
                self.err(span, format!("unexpected character `{c}` in interpolation"));
                Tok::Eof
            }
        };
        LexedToken::new(kind, span)
    }
}