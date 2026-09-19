use std::fmt;

/// A resolved/runtime type after semantic analysis and name resolution.
///
/// Generic types carry their concrete argument list (`Class("List", [Ty::Int])`
/// etc.). `Var` denotes an unresolved generic parameter inside a generic
/// function/class body; it is only valid during monomorphization.
#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Bool,
    Char,
    Int,
    Float,
    String,
    /// The type of the `none` literal in isolation.
    None,
    /// `T?`
    Option(Box<Ty>),
    /// A class type with type arguments.
    Class(String, Vec<Ty>),
    /// A struct type.
    Struct(String, Vec<Ty>),
    /// An enum type.
    Enum(String, Vec<Ty>),
    /// An interface type (usable as a static type).
    Interface(String, Vec<Ty>),
    /// `List<T>`
    List(Box<Ty>),
    /// `Map<K, V>`
    Map(Box<Ty>, Box<Ty>),
    /// `(T1, T2, ...)`
    Tuple(Vec<Ty>),
    /// `(T1, T2) -> R`
    Fn(Vec<Ty>, Box<Ty>),
    /// No value (statements, void return).
    Empty,
    /// Unresolved/errored type produced during recovery.
    Unknown,
    /// A generic parameter name.
    Var(String),
    /// `Range<T>` produced by `a..b`.
    Range(Box<Ty>),
}

impl Ty {
    pub fn is_primitive(&self) -> bool {
        matches!(self, Ty::Bool | Ty::Char | Ty::Int | Ty::Float | Ty::String)
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Ty::Int | Ty::Float)
    }

    /// `T?` of this type (flattening nested options).
    pub fn opt_of(&self) -> Ty {
        match self {
            Ty::Option(_) => self.clone(),
            other => Ty::Option(Box::new(other.clone())),
        }
    }

    pub fn is_option(&self) -> bool {
        matches!(self, Ty::Option(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Ty::None)
    }

    /// Inner type of an option, or None if not an option.
    pub fn inner_option(&self) -> Option<Ty> {
        match self {
            Ty::Option(inner) => Some((**inner).clone()),
            _ => None,
        }
    }

    pub fn is_never(&self) -> bool {
        matches!(self, Ty::Empty)
    }

    /// The "kind name" used in diagnostics (e.g. `class`, `struct`).
    pub fn kind_name(&self) -> &'static str {
        match self {
            Ty::Class(..) => "class",
            Ty::Struct(..) => "struct",
            Ty::Enum(..) => "enum",
            Ty::Interface(..) => "interface",
            Ty::List(..) => "list",
            Ty::Map(..) => "map",
            Ty::Tuple(..) => "tuple",
            Ty::Fn(..) => "function",
            Ty::Option(..) => "optional",
            _ => "type",
        }
    }

    /// Render the type with its type arguments omitted (e.g. `List<T>` -> `List`).
    pub fn bare_name(&self) -> String {
        match self {
            Ty::Bool => "bool".into(),
            Ty::Char => "char".into(),
            Ty::Int => "int".into(),
            Ty::Float => "float".into(),
            Ty::String => "string".into(),
            Ty::None => "none".into(),
            Ty::Option(inner) => format!("{}?", inner.bare_name()),
            Ty::Class(n, args) | Ty::Struct(n, args) | Ty::Enum(n, args) | Ty::Interface(n, args) => {
                if args.is_empty() {
                    n.clone()
                } else {
                    let inner = args.iter().map(Ty::bare_name).collect::<Vec<_>>().join(", ");
                    format!("{n}<{inner}>")
                }
            }
            Ty::List(inner) => format!("List<{}>", inner.bare_name()),
            Ty::Map(k, v) => format!("Map<{}, {}>", k.bare_name(), v.bare_name()),
            Ty::Tuple(items) => {
                let inner = items.iter().map(Ty::bare_name).collect::<Vec<_>>().join(", ");
                format!("({inner})")
            }
            Ty::Fn(params, ret) => {
                let ps = params.iter().map(Ty::bare_name).collect::<Vec<_>>().join(", ");
                format!("({ps}) -> {}", ret.bare_name())
            }
            Ty::Empty => "void".into(),
            Ty::Unknown => "?".into(),
            Ty::Var(n) => n.clone(),
            Ty::Range(inner) => format!("Range<{}>", inner.bare_name()),
        }
    }

    /// Return the underlying named type this is built on (unwraps Var and None
    /// suppression), used to look up method tables while checking.
    pub fn named(&self) -> Option<&str> {
        match self {
            Ty::Class(n, _) | Ty::Struct(n, _) | Ty::Enum(n, _) | Ty::Interface(n, _) => Some(n),
            Ty::String => Some("string"),
            Ty::List(_) => Some("list"),
            Ty::Map(..) => Some("map"),
            _ => None,
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bare_name())
    }
}