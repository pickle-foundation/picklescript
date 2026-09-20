use pickle_compiler::diag::{DiagnosticSink, FileId, SourceMap};
use pickle_compiler::lexer::lex;
use pickle_compiler::token::{LexedToken, StrSeg, Tok};

struct Lex {
    tokens: Vec<LexedToken>,
    msgs: Vec<String>,
}

fn lex_str(src: &str) -> Lex {
    let diags = DiagnosticSink::new();
    let mut map = SourceMap::default();
    let file: FileId = map.add("test.pk".to_string(), src.to_string());
    let tokens = lex(file, src, &diags);
    let msgs = diags
        .diagnostics
        .borrow()
        .iter()
        .map(|d| d.message.clone())
        .collect();
    Lex { tokens, msgs }
}

fn find(tokens: &[LexedToken], k: &Tok) -> bool {
    tokens.iter().any(|t| &t.token.kind == k)
}

fn find_ref(tokens: &[&LexedToken], k: &Tok) -> bool {
    tokens.iter().any(|t| &t.token.kind == k)
}

fn find_after(tokens: &[LexedToken], pivot: usize, k: &Tok) -> bool {
    tokens.iter().skip(pivot).any(|t| &t.token.kind == k)
}

fn str_text(t: &LexedToken) -> Option<String> {
    if let Tok::Str(lit) = &t.token.kind {
        let mut out = String::new();
        for seg in &lit.segments {
            if let StrSeg::Text { text } = seg {
                out.push_str(text);
            }
        }
        Some(out)
    } else {
        None
    }
}

fn expr_tokens(t: &LexedToken) -> Vec<&LexedToken> {
    if let Tok::Str(lit) = &t.token.kind {
        return lit
            .segments
            .iter()
            .filter_map(|seg| match seg {
                StrSeg::Expr { tokens } => Some(tokens.as_slice()),
                StrSeg::Text { .. } => None,
            })
            .flatten()
            .collect();
    }
    Vec::new()
}

fn str_tokens(l: &Lex) -> Vec<&LexedToken> {
    l.tokens.iter().filter(|t| matches!(t.token.kind, Tok::Str(_))).collect()
}

#[test]
fn string_and_char_escapes_are_consumed() {
    let l = lex_str(
        r#"fn f() {
            let a = '\n'
            let b = '\u{41}'
            let s = "a\nb\t\"c\\d\{e\}f"
        }"#,
    );
    assert!(
        l.msgs.is_empty(),
        "unexpected lexer errors:\n{}",
        l.msgs.join("\n")
    );
    let strs = str_tokens(&l);
    assert_eq!(strs.len(), 1, "expected exactly one string token");
    assert_eq!(
        str_text(strs[0]).as_deref(),
        Some("a\nb\t\"c\\d{e}f"),
        "escaped string body must be reconstructed exactly"
    );
}

#[test]
fn unicode_escape_in_string_decodes() {
    let l = lex_str(r#"let s = "emoji \u{1F600} end""#);
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);
    let strs = str_tokens(&l);
    assert_eq!(str_text(strs[0]).as_deref(), Some("emoji \u{1F600} end"));
}

#[test]
fn typed_number_suffixes_lex_as_one_number_token() {
    let l = lex_str("let a = 7u8\nlet b = -2i32\nlet c = 2.5f32\n");
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);

    let num_suffix: Vec<Option<&String>> = l
        .tokens
        .iter()
        .filter(|t| matches!(t.token.kind, Tok::Number))
        .map(|t| t.data.suffix.as_ref())
        .collect();
    assert_eq!(
        num_suffix,
        vec![Some(&"u8".to_string()), Some(&"i32".to_string()), Some(&"f32".to_string())],
        "suffixes must be captured whole (u8, i32, f32), not one letter"
    );

    let last = &l.tokens[l.tokens.iter().position(|t| matches!(t.token.kind, Tok::Number)).unwrap() + 1];
    let f32_token = l
        .tokens
        .iter()
        .find(|t| matches!(t.token.kind, Tok::Number) && t.data.suffix.as_deref() == Some("f32"))
        .unwrap();
    assert!(f32_token.data.is_float, "2.5f32 must be lexed as a float");
    let _ = last;
}

#[test]
fn operators_inside_interpolation() {
    let l = lex_str(r#"let s = "{a << 2}>{c..=d}; {e ?: f}; {g <- h}; {0..10}" "#);
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);
    let strs = str_tokens(&l);
    let exprs = expr_tokens(strs[0]);
    assert!(
        find_ref(&exprs, &Tok::Shl),
        "interpolation must lex `<<` ({:?})",
        exprs.iter().map(|t| t.token.kind.clone()).collect::<Vec<_>>()
    );
    assert!(find_ref(&exprs, &Tok::RangeIncl), "interpolation must lex `..=`");
    assert!(find_ref(&exprs, &Tok::QuestionColon), "interpolation must lex `?:`");
    assert!(find_ref(&exprs, &Tok::SendOp), "interpolation must lex `<-`");
    assert!(find_ref(&exprs, &Tok::Range), "interpolation must lex `..`");
}

#[test]
fn insane_operators_lex_top_level() {
    let l = lex_str("a << b >> c ..= d ?: e\nch <- v\nx...y\n");
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);
    assert!(find(&l.tokens, &Tok::Shl));
    assert!(find(&l.tokens, &Tok::Shr));
    assert!(find(&l.tokens, &Tok::RangeIncl));
    assert!(find(&l.tokens, &Tok::QuestionColon));
    assert!(find(&l.tokens, &Tok::SendOp), "`<-` must lex at top level");
    assert!(find(&l.tokens, &Tok::Ellipsis), "`...` must lex as Ellipsis");
    assert!(
        !find(&l.tokens, &Tok::Range),
        "`x...y` must not lex as Range followed by Dot"
    );
}

#[test]
fn raw_string_is_literal_text_only() {
    let l = lex_str("let r = r\"raw {not inserted} \\n\\t\"\n");
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);
    let strs = str_tokens(&l);
    assert_eq!(strs.len(), 1);
    let t = strs[0];
    assert_eq!(expr_tokens(t).len(), 0, "raw strings must not interpolate");
    assert_eq!(
        str_text(t).as_deref(),
        Some("raw {not inserted} \\n\\t"),
        "backslash escapes must stay literal in raw strings"
    );
}

#[test]
fn multiline_string_with_interpolation_and_newline() {
    let l = lex_str("let m = \"\"\"hello\nworld {n}\"\"\"\n");
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);
    let strs = str_tokens(&l);
    assert_eq!(strs.len(), 1);
let t = strs[0];
    let exprs = expr_tokens(t);
    let expr_count = if let Tok::Str(lit) = &t.token.kind {
        lit.segments
            .iter()
            .filter(|s| matches!(s, StrSeg::Expr { .. }))
            .count()
    } else {
        0
    };
    assert_eq!(expr_count, 1, "expected one interpolation inside triple string");
    assert!(matches!(exprs[0].token.kind, Tok::Ident(_)));
    if let Tok::Str(lit) = &t.token.kind {
        let text = lit.segments.iter().find_map(|s| match s {
            StrSeg::Text { text } => Some(text.clone()),
            StrSeg::Expr { .. } => None,
        });
        assert_eq!(text.as_deref(), Some("hello\nworld "));
    } else {
        panic!("expected string token");
    }
}

#[test]
fn nested_braces_in_interpolation() {
    let l = lex_str("let s = \"{ {1, 2} }\"\n");
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);
    let strs = str_tokens(&l);
    let exprs = expr_tokens(strs[0]);
    assert!(find_ref(&exprs, &Tok::LBrace), "nested `{{` must lex inside interpolation");
    assert!(find_ref(&exprs, &Tok::RBrace), "nested `}}` must lex inside interpolation");
    assert!(
        !find(&l.tokens, &Tok::RBrace),
        "the interpolation's own closing brace must not leak as a top-level token"
    );
}

#[test]
fn newline_inside_interpolation_parens_continues() {
    let l = lex_str("let s = \"{ (a +\n b) }\"\n");
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);
    let strs = str_tokens(&l);
    let exprs = expr_tokens(strs[0]);
    assert!(find_ref(&exprs, &Tok::Plus), "multi-line parenthesized expr must lex");
}

#[test]
fn doc_comments_attach_to_following_token() {
    let l = lex_str("/// docs for main\nfn main() {\n/** block docs */\nlet x = 1\n}\n");
    assert!(l.msgs.is_empty(), "unexpected errors: {:?}", l.msgs);

    let fn_idx = l
        .tokens
        .iter()
        .position(|t| matches!(t.token.kind, Tok::Fn))
        .expect("fn token");
    assert_eq!(
        l.tokens[fn_idx].data.doc.as_deref(),
        Some("docs for main"),
        "`///` doc line must attach to the following declaration"
    );

    let let_idx = l
        .tokens
        .iter()
        .position(|t| matches!(t.token.kind, Tok::Let))
        .expect("let token");
    assert_eq!(
        l.tokens[let_idx].data.doc.as_deref(),
        Some("block docs"),
        "`/** */` doc block must attach to the following declaration"
    );
}

#[test]
fn doc_newline_does_not_steal_doc() {
    let l = lex_str("/// doc\nlet y = 1\n");
    let let_idx = l
        .tokens
        .iter()
        .position(|t| matches!(t.token.kind, Tok::Let))
        .expect("let token");
    assert_eq!(l.tokens[let_idx].data.doc.as_deref(), Some("doc"));
}

#[test]
fn unterminated_block_comment_is_reported() {
    let l = lex_str("let x = 1 /* never closed\n");
    assert!(
        l.msgs.iter().any(|m| m.contains("unterminated block comment")),
        "expected unterminated block comment diagnostic, got: {:?}",
        l.msgs
    );
    assert!(find(&l.tokens, &Tok::Ident(String::from("x"))), "tokens before the comment survive");
}

#[test]
fn bad_characters_are_skipped_and_lexing_continues() {
    let l = lex_str("let x = 1 $\nlet y = 2 `\n");
    let errs: Vec<&String> = l
        .msgs
        .iter()
        .filter(|m| m.starts_with("unexpected character"))
        .collect();
assert_eq!(errs.len(), 2, "one diagnostic per bad character, got: {:?}", l.msgs);
    assert!(find(&l.tokens, &Tok::Ident(String::from("x"))));
    assert!(
        find_after(&l.tokens, 0, &Tok::Ident(String::from("y"))),
        "lexing must continue past a bad character"
    );
}

#[test]
fn real_nul_byte_is_an_error_but_does_not_truncate() {
    let l = lex_str("let a = 1 \u{0} let b = 2\n");
    assert!(
        l.msgs.iter().any(|m| m.contains("unexpected character")),
        "NUL is a real (invalid) character now, got: {:?}",
        l.msgs
    );
    assert!(
        find_after(&l.tokens, 0, &Tok::Ident(String::from("b"))),
        "the file must not be silently truncated at a NUL byte"
    );
}

#[test]
fn unterminated_interpolation_is_reported() {
    let l = lex_str("let s = \"{x\"\n");
    assert!(
        l.msgs.iter().any(|m| m.contains("unterminated interpolation")),
        "expected unterminated interpolation diagnostic, got: {:?}",
        l.msgs
    );
}

#[test]
fn unterminated_string_and_raw_string_are_reported() {
    let bad = lex_str("let s = \"abc\n");
    assert!(bad.msgs.iter().any(|m| m.contains("unterminated string")));

    let raw = lex_str("let s = r\"abc\n");
    assert!(raw.msgs.iter().any(|m| m.contains("unterminated raw string")));

    let multi = lex_str("let s = \"\"\"abc\n");
    assert!(multi.msgs.iter().any(|m| m.contains("unterminated multi-line string")));
}

#[test]
fn char_empty_and_unterminated_reported() {
    let l = lex_str("let a = ''\n");
    assert!(l.msgs.iter().any(|m| m.contains("empty character literal")));

    let u = lex_str("let a = 'x\n");
    assert!(u.msgs.iter().any(|m| m.contains("unterminated character literal")));
}

#[test]
fn keyword_lookup_is_complete() {
    // The match-based keyword table must keep covering every reserved word.
for s in [
        "module", "import", "use", "class", "struct", "enum", "interface", "fn",
        "constructor", "property", "get", "set", "static", "override", "let", "var",
        "const", "public", "private", "protected", "if", "else", "while", "for", "in",
        "break", "continue", "return", "match", "case", "true", "false", "none", "async",
        "await", "task", "channel", "unsafe", "extends", "implements", "is", "as", "this",
        "super", "init", "deinit", "operator",
    ] {
        assert!(
            Tok::keyword(s).is_some(),
            "expected `{s}` to be recognized as a keyword"
        );
        let l = lex_str(&format!("{s}\n"));
        assert!(
            !matches!(&l.tokens[0].token.kind, Tok::Ident(_)),
            "`{s}` must lex as a keyword, not an identifier"
        );
    }
}
