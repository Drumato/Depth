type Virtual = usize;
type Physical = usize;
type Offset = usize;
type Index = Option<usize>;
#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    INTLIT(i128),
    CHARLIT(char),
    REG(Virtual, Physical, Index),
    ID(String, Offset, Index),
    CALL(String, usize),
}
impl Operand {
    pub fn string(&self) -> String {
        match self {
            Self::CHARLIT(value) => format!("'{}'", value),
            Self::INTLIT(value) => format!("{}", value),
            Self::REG(virt, _phys, _oind) => format!("t{}", virt),
            Self::ID(name, _, _oind) => name.to_string(),
            Self::CALL(func, argc) => format!("call {}, {}", func, argc),
        }
    }
    fn dump_st(&self) -> String {
        match self {
            Self::CHARLIT(value) => format!("'{}'", value),
            Self::INTLIT(value) => format!("{}", value),
            Self::REG(virt, _phys, oind) => match oind {
                Some(index) => format!("t{}[{}]", virt, index),
                None => format!("t{}", virt),
            },
            Self::ID(name, _, oind) => match oind {
                Some(index) => format!("{}[{}]", name, index),
                None => name.to_string(),
            },
            Self::CALL(func, argc) => format!("call {}, {}", func, argc),
        }
    }
}
#[derive(Clone)]
pub enum Tac {
    EX(Operand, String, Operand, Operand),
    UNEX(Operand, String, Operand),
    RET(Operand),
    PARAM(usize, Operand),
    LET(Operand, Operand),
    IFF(Operand, String),
    GOTO(String),
    LABEL(String),
    FUNCNAME(String),
    PROLOGUE(usize),
    PUSHARG(usize, usize),
}
impl Tac {
    pub fn string(&self) -> String {
        match self {
            Self::LABEL(name) => format!("{}:", name),
            Self::FUNCNAME(name) => format!("{}:", name),
            Self::EX(lv, op, lop, rop) => format!(
                "{} <- {} {} {}",
                lv.dump_st(),
                lop.dump_st(),
                op,
                rop.dump_st()
            ),
            Self::UNEX(lv, op, lop) => format!("{} <- {}{}", lv.dump_st(), op, lop.dump_st(),),
            Self::RET(op) => format!("ret {}", op.dump_st()),
            Self::LET(lv, op) => format!("{} <- {}", lv.dump_st(), op.dump_st()),
            Self::IFF(cond, label) => format!("ifFalse {} goto {}", cond.dump_st(), label),
            Self::GOTO(label) => format!("goto {}", label),
            Self::PARAM(reg, arg) => format!("param {} {}", reg + 1, arg.dump_st()),
            Self::PROLOGUE(offset) => format!("prologue {}", offset),
            Self::PUSHARG(_reg, offset) => format!("pusharg {}", offset),
        }
    }
}
