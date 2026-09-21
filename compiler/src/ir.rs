//! PickleIR: the typed, lowering-oriented intermediate representation produced
//! from the checked AST (Milestone 3, first slice).
//!
//! Model: each function is a small CFG of basic blocks over
//! procedure-local *slots* (memory, like `alloca` slots / Wasm locals) and
//! *temps* (fresh values, valid only inside the block that defined them).
//! Values that must survive a branch go through slots, so no phis are needed;
//! this maps 1:1 onto Cranelift stack locals and block-local virtual values.

use std::collections::HashMap;
use std::fmt;

/// The IR type lattice. Ptr is any heap reference (string, list, map,
/// class/struct instance, option). Strings are heap objects in this model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrTy {
    Bool,
    Char,
    Int,
    Float,
    Str,
    Unit,
    Ptr,
}

impl IrTy {
    pub fn is_unit(self) -> bool {
        self == IrTy::Unit
    }
}

impl fmt::Display for IrTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IrTy::Bool => "bool",
            IrTy::Char => "char",
            IrTy::Int => "int64",
            IrTy::Float => "float64",
            IrTy::Str => "string",
            IrTy::Unit => "unit",
            IrTy::Ptr => "ptr",
        };
        f.write_str(s)
    }
}

/// A value produced by an instruction; only valid in its defining block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temp(pub u32);

/// A procedure-local storage cell; valid function-wide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Slot(pub u32);

/// A basic block id. `func.entry` is the entry block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

/// A module-level string constant (into `IrModule::strings`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrConst {
    Int(i64),
    Float(u64),
    Bool(bool),
    Char(u32),
    Null,
    Str(StrId),
    /// Tag `Int`: raw address of a `pkl_strdata_` block (module string data).
    /// Never used as a managed pointer; used to hand string bytes to the
    /// runtime (e.g. `pickle_class_register`). Typed `Int` so the value never
    /// enters the GC trace frame.
    StrAddr(StrId),
    /// Tag `Int`: the link-time address of a module function's symbol. Used to
    /// hand a finalizer (`pkl_<T>_deinit`) to `pickle_class_register`. Typed
    /// `Int` so the raw code pointer never enters the GC trace frame.
    FuncAddr(FuncId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone)]
pub struct IrParam {
    pub name: String,
    pub ty: IrTy,
}

#[derive(Debug, Clone)]
pub struct IrFunc {
    /// Unmangled source name.
    pub name: String,
    /// Linkable symbol (compiler-owned namespace, `pkl_*`).
    pub symbol: String,
    pub params: Vec<IrParam>,
    pub ret: IrTy,
    /// Slot index -> type (params occupy slots 0..params.len()).
    pub slots: Vec<IrTy>,
    pub entry: BlockId,
    pub blocks: Vec<IrBlock>,
    /// `true` for `main`.
    pub is_main: bool,
    /// `true` for tests.
    pub is_test: bool,
    /// `#[tag("…")]` tags carried by test items (empty for non-tests).
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IrBlock {
    pub id: BlockId,
    pub instrs: Vec<IrInstr>,
    pub term: IrTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Callee {
    Func(FuncId),
    Extern(ExternId),
}

/// A reference to a runtime `pickle_*` symbol the codegen/link phase binds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrExtern {
    pub symbol: String,
    pub params: Vec<IrTy>,
    pub ret: IrTy,
}

#[derive(Debug, Clone)]
pub enum IrInstr {
    Const { dst: Temp, c: IrConst },
    /// Integer or float negation; logical/bool `not`; bitwise `not`.
    UnOp { dst: Temp, op: UnOp, v: Temp },
    /// Arith/shift/bitwise/comparison. Comparison results are `bool`.
    BinOp { dst: Temp, op: BinOp, a: Temp, b: Temp },
    Copy { dst: Temp, v: Temp },
    /// Signed int -> float conversion (numeric promotion at mixed operands).
    Itof { dst: Temp, v: Temp },
    /// Float -> int conversion (saturating cast).
    Ftoi { dst: Temp, v: Temp },
    /// Char (code point, i32) -> int (i64): scalar reinterpreting cast.
    Chartoi { dst: Temp, v: Temp },
    /// Int (i64) -> char (i32): scalar reinterpreting cast (narrowing).
    Itochar { dst: Temp, v: Temp },
    LoadSlot { dst: Temp, slot: Slot },
    StoreSlot { slot: Slot, v: Temp },
    /// Raw address (`Int`) of a scalar local's stack storage. Only valid for
    /// non-managed slots; the address is never GC-tracked.
    LocalAddr { dst: Temp, slot: Slot },
    /// Load a scalar of `ty` through a raw address (`Int`) temp.
    LoadRaw { dst: Temp, addr: Temp, ty: IrTy },
    /// Store a scalar of `ty` through a raw address (`Int`) temp.
    StoreRaw { addr: Temp, v: Temp, ty: IrTy },
    /// Call a user function or runtime symbol. `dst` absent for void calls.
    Call { dst: Option<Temp>, callee: Callee, args: Vec<Temp> },
    /// Call a closure value: a function pointer (`Int`) over an explicit
    /// signature. `fn_addr` is the raw code address of the closure's body
    /// entry or of a top-level function used as a value.
    CallInd {
        dst: Option<Temp>,
        fn_addr: Temp,
        params: Vec<IrTy>,
        ret: IrTy,
        args: Vec<Temp>,
    },
}

#[derive(Debug, Clone)]
pub enum IrTerm {
    Branch { target: BlockId },
    BranchIf { cond: Temp, then: BlockId, else_: BlockId },
    Return { v: Option<Temp> },
    Unreachable,
}

#[derive(Debug, Clone, Default)]
pub struct IrModule {
    pub funcs: Vec<IrFunc>,
    pub externs: Vec<IrExtern>,
    /// Source function name -> id (slice-1: at most one non-generic entry).
    pub funcs_by_name: HashMap<String, FuncId>,
    /// String literal data.
    pub strings: Vec<Vec<u8>>,
}

impl IrModule {
    pub fn extern_id(&mut self, ex: IrExtern) -> ExternId {
        for (i, e) in self.externs.iter().enumerate() {
            if *e == ex {
                return ExternId(i);
            }
        }
        self.externs.push(ex);
        ExternId(self.externs.len() - 1)
    }
}

impl fmt::Display for IrModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for func in &self.funcs {
            writeln!(f, "{func}")?;
        }
        Ok(())
    }
}

impl fmt::Display for IrFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = if self.is_main {
            "main "
        } else if self.is_test {
            "test "
        } else {
            ""
        };
        write!(
            f,
            "{kind}fn {}({}) -> {}{}",
            self.name,
            self.params
                .iter()
                .map(|p| format!("{}: {}", p.name, p.ty))
                .collect::<Vec<_>>()
                .join(", "),
            self.ret,
            if self.symbol.is_empty() {
                String::new()
            } else {
                format!(" @{}", self.symbol)
            }
        )?;
        for block in &self.blocks {
            writeln!(f)?;
            write!(f, "  {block}")?;
        }
        Ok(())
    }
}

impl fmt::Display for IrBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = self.id;
        if self.id.0 == 0 {
            writeln!(f, "@entry:")?;
        } else {
            writeln!(f, "@{}:", self.id.0)?;
        }
        for instr in &self.instrs {
            writeln!(f, "    {instr}")?;
        }
        writeln!(f, "    {}", self.term)
    }
}

fn temp_name(t: Temp) -> String {
    format!("t{}", t.0)
}

impl fmt::Display for IrInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrInstr::Const { dst, c } => write!(f, "{} = const {}", temp_name(*dst), irconst_repr(*c)),
            IrInstr::UnOp { dst, op, v } => {
                let op = match op {
                    UnOp::Neg => "neg",
                    UnOp::Not => "not",
                    UnOp::BitNot => "bitnot",
                };
                write!(f, "{} = {} {}", temp_name(*dst), op, temp_name(*v))
            }
            IrInstr::BinOp { dst, op, a, b } => write!(
                f,
                "{} = binop.{} {}, {}",
                temp_name(*dst),
                binop_repr(*op),
                temp_name(*a),
                temp_name(*b)
            ),
            IrInstr::Copy { dst, v } => write!(f, "{} = copy {}", temp_name(*dst), temp_name(*v)),
            IrInstr::Itof { dst, v } => write!(f, "{} = itof {}", temp_name(*dst), temp_name(*v)),
            IrInstr::Ftoi { dst, v } => write!(f, "{} = ftoi {}", temp_name(*dst), temp_name(*v)),
            IrInstr::Chartoi { dst, v } => write!(f, "{} = chartoi {}", temp_name(*dst), temp_name(*v)),
            IrInstr::Itochar { dst, v } => write!(f, "{} = itochar {}", temp_name(*dst), temp_name(*v)),
            IrInstr::LoadSlot { dst, slot } => {
                write!(f, "{} = load slot s{}", temp_name(*dst), slot.0)
            }
            IrInstr::StoreSlot { slot, v } => {
                write!(f, "store {} -> slot s{}", temp_name(*v), slot.0)
            }
            IrInstr::LocalAddr { dst, slot } => {
                write!(f, "{} = addr slot s{}", temp_name(*dst), slot.0)
            }
            IrInstr::LoadRaw { dst, addr, ty } => {
                write!(f, "{} = loadraw.{} [{}]", temp_name(*dst), ty, temp_name(*addr))
            }
            IrInstr::StoreRaw { addr, v, ty } => {
                write!(f, "storeraw.{} {} -> [{}]", ty, temp_name(*v), temp_name(*addr))
            }
            IrInstr::Call { dst, callee, args } => {
                let callee = match callee {
                    Callee::Func(id) => format!("fn#{}", id.0),
                    Callee::Extern(id) => format!("extern#{}", id.0),
                };
                let args = args.iter().map(|a| temp_name(*a)).collect::<Vec<_>>().join(", ");
                match dst {
                    Some(d) => write!(f, "{} = call {}({})", temp_name(*d), callee, args),
                    None => write!(f, "call {}({})", callee, args),
                }
            }
            IrInstr::CallInd { dst, fn_addr, params, ret, args } => {
                let sig = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let args = args.iter().map(|a| temp_name(*a)).collect::<Vec<_>>().join(", ");
                match dst {
                    Some(d) => write!(
                        f,
                        "{} = callind [{}] -> {} ({}) via {}",
                        temp_name(*d),
                        sig,
                        ret,
                        args,
                        temp_name(*fn_addr)
                    ),
                    None => write!(
                        f,
                        "callind [{}] -> {} ({}) via {}",
                        sig,
                        ret,
                        args,
                        temp_name(*fn_addr)
                    ),
                }
            }
        }
    }
}

impl fmt::Display for IrTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrTerm::Branch { target } => write!(f, "branch @{}", target.0),
            IrTerm::BranchIf { cond, then, else_ } => write!(
                f,
                "branchif {} @{} @{}",
                temp_name(*cond),
                then.0,
                else_.0
            ),
            IrTerm::Return { v } => match v {
                Some(t) => write!(f, "return {}", temp_name(*t)),
                None => write!(f, "return"),
            },
            IrTerm::Unreachable => write!(f, "unreachable"),
        }
    }
}

fn binop_repr(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "add",
        BinOp::Sub => "sub",
        BinOp::Mul => "mul",
        BinOp::Div => "div",
        BinOp::Mod => "mod",
        BinOp::Shl => "shl",
        BinOp::Shr => "shr",
        BinOp::BitAnd => "band",
        BinOp::BitOr => "bor",
        BinOp::BitXor => "bxor",
        BinOp::Eq => "eq",
        BinOp::Ne => "ne",
        BinOp::Lt => "lt",
        BinOp::Le => "le",
        BinOp::Gt => "gt",
        BinOp::Ge => "ge",
    }
}

pub fn irconst_repr(c: IrConst) -> String {
    match c {
        IrConst::Int(v) => v.to_string(),
        IrConst::Float(bits) => format!("{}f", f64::from_bits(bits)),
        IrConst::Bool(v) => v.to_string(),
        IrConst::Char(v) => format!("'{}'", char::from_u32(v).unwrap_or('\u{FFFD}')),
        IrConst::Null => "null".to_string(),
        IrConst::Str(id) => format!("str#{}", id.0),
        IrConst::StrAddr(id) => format!("addrof str#{}", id.0),
        IrConst::FuncAddr(id) => format!("addrof fn#{}", id.0),
    }
}