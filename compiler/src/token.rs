use crate::diag::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum StrSeg {
    Text { text: String },
    Expr { tokens: Vec<LexedToken> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrLit {
    pub segments: Vec<StrSeg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Ident(String),

    // Literals
    Number,
    Char(char),
    Str(StrLit),

    // Keywords
    Module,
    Import,
    Use,
    Class,
    Struct,
    Enum,
    Interface,
    Fn,
    Constructor,
    Property,
    Get,
    Set,
    Static,
    Override,
    Let,
    Var,
    Const,
    Public,
    Private,
    Protected,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    Return,
    Match,
    Case,
    True,
    False,
    None,
    Async,
    Await,
    Task,
    Channel,
    Unsafe,
    Extends,
    Implements,
    Is,
    As,
    This,
    Super,
    Init,
    Deinit,
    Operator,

    // Punctuation & operators
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    ColonColon,
    Semicolon,
    Arrow,
    FatArrow,
    Assign,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    ShlEq,
    ShrEq,
    AndEq,
    OrEq,
    XorEq,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    Bang,
    Tilde,
    Amp,
    Pipe,
    Caret,
    Shl,
    Shr,
    EqEq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    AndAnd,
    OrOr,
    Question,
    QuestionDot,
    QuestionQuestion,
    QuestionColon,
    Range,
    RangeIncl,
    SendOp,
    Ellipsis,
    At,
    Hash,

    Newline,
    Eof,
}

impl Tok {
    pub fn describe(&self) -> String {
        match self {
            Tok::Ident(s) => format!("identifier `{s}`"),
            Tok::Number => "number".to_string(),
            Tok::Char(c) => format!("character literal `{c}`"),
            Tok::Str(_) => "string literal".to_string(),
            Tok::Newline => "end of line".to_string(),
            Tok::Eof => "end of file".to_string(),
            other => format!("`{}`", other.punct()),
        }
    }

    pub fn punct(&self) -> &'static str {
        use Tok::*;
        match self {
            LParen => "(",
            RParen => ")",
            LBrace => "{",
            RBrace => "}",
            LBracket => "[",
            RBracket => "]",
            Comma => ",",
            Dot => ".",
            Colon => ":",
            ColonColon => "::",
            Arrow => "->",
            FatArrow => "=>",
            Assign => "=",
            PlusEq => "+=",
            MinusEq => "-=",
            StarEq => "*=",
            SlashEq => "/=",
            PercentEq => "%=",
            ShlEq => "<<=",
            ShrEq => ">>=",
            AndEq => "&=",
            OrEq => "|=",
            XorEq => "^=",
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            StarStar => "**",
            Bang => "!",
            Tilde => "~",
            Amp => "&",
            Pipe => "|",
            Caret => "^",
            Shl => "<<",
            Shr => ">>",
            EqEq => "==",
            NotEq => "!=",
            Lt => "<",
            Le => "<=",
            Gt => ">",
            Ge => ">=",
            AndAnd => "&&",
            OrOr => "||",
            Question => "?",
            QuestionDot => "?.",
            QuestionQuestion => "??",
            QuestionColon => "?:",
            Range => "..",
            RangeIncl => "..=",
            SendOp => "<-",
            Ellipsis => "...",
            At => "@",
            Hash => "#",
            _ => "?",
        }
    }

    /// Look up a keyword by its source text. Compiles to a jump table rather
    /// than a linear scan.
    pub fn keyword(s: &str) -> Option<Tok> {
        use Tok::*;
        match s {
            "module" => Some(Module),
            "import" => Some(Import),
            "use" => Some(Use),
            "class" => Some(Class),
            "struct" => Some(Struct),
            "enum" => Some(Enum),
            "interface" => Some(Interface),
            "fn" => Some(Fn),
            "constructor" => Some(Constructor),
            "property" => Some(Property),
            "get" => Some(Get),
            "set" => Some(Set),
            "static" => Some(Static),
            "override" => Some(Override),
            "let" => Some(Let),
            "var" => Some(Var),
            "const" => Some(Const),
            "public" => Some(Public),
            "private" => Some(Private),
            "protected" => Some(Protected),
            "if" => Some(If),
            "else" => Some(Else),
            "while" => Some(While),
            "for" => Some(For),
            "in" => Some(In),
            "break" => Some(Break),
            "continue" => Some(Continue),
            "return" => Some(Return),
            "match" => Some(Match),
            "case" => Some(Case),
            "true" => Some(True),
            "false" => Some(False),
            "none" => Some(None),
            "async" => Some(Async),
            "await" => Some(Await),
            "task" => Some(Task),
            "channel" => Some(Channel),
            "unsafe" => Some(Unsafe),
            "extends" => Some(Extends),
            "implements" => Some(Implements),
            "is" => Some(Is),
            "as" => Some(As),
            "this" => Some(This),
            "super" => Some(Super),
            "init" => Some(Init),
            "deinit" => Some(Deinit),
            "operator" => Some(Operator),
            _ => Option::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Tok,
    pub span: Span,
}

impl Token {
    pub fn new(kind: Tok, span: Span) -> Token {
        Token { kind, span }
    }
}

/// Value payloads carried on some tokens: number text/radix, char, ident kind.
#[derive(Debug, Clone, Default)]
pub struct TokenData {
    pub text: String,
    pub int: Option<i128>,
    pub float: Option<f64>,
    pub is_float: bool,
    pub suffix: Option<String>,
    pub doc: Option<String>,
}

impl PartialEq for TokenData {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.int == other.int
            && self.is_float == other.is_float
            && self.suffix == other.suffix
            && self.doc == other.doc
            && match (self.float, other.float) {
                (Some(a), Some(b)) => a.to_bits() == b.to_bits(),
                (None, None) => true,
                _ => false,
            }
    }
}

/// A token plus the payload needed by the parser.
#[derive(Debug, Clone, PartialEq)]
pub struct LexedToken {
    pub token: Token,
    pub data: TokenData,
}

impl LexedToken {
    pub fn new(kind: Tok, span: Span) -> LexedToken {
        LexedToken {
            token: Token::new(kind, span),
            data: TokenData::default(),
        }
    }

    pub fn number(text: String, span: Span) -> LexedToken {
        let mut t = LexedToken::new(Tok::Number, span);
        t.data.text = text.clone();
        t.data.int = parse_int_literal(&text);
        if t.data.int.is_none() {
            t.data.float = text.replace('_', "").parse::<f64>().ok();
            t.data.is_float = text.contains('.') || text.contains('e') || text.contains('E');
        }
        t
    }
}

fn parse_int_literal(text: &str) -> Option<i128> {
    let clean = text.replace('_', "");
    let (sign, body) = match clean.strip_prefix('-') {
        Some(_) => ("-", &clean[1..]),
        None => ("", clean.as_str()),
    };
    let (radix, digits) = if let Some(h) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
        (16, h)
    } else if let Some(h) = body.strip_prefix("0b").or_else(|| body.strip_prefix("0B")) {
        (2, h)
    } else if let Some(h) = body.strip_prefix("0o").or_else(|| body.strip_prefix("0O")) {
        (8, h)
    } else {
        (10, body)
    };
    if digits.is_empty() {
        return None;
    }
    let v = i128::from_str_radix(digits, radix).ok()?;
    if sign == "-" {
        Some(-v)
    } else {
        Some(v)
    }
}

