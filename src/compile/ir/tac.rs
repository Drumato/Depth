type Virtual = usize;
type Physical = usize;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lvalue {
    REG(Virtual, Physical),
    ID(String, Physical),
    INDEX(Operand, Operand),
}
impl Lvalue {
    pub fn string(&self) -> String {
        match self {
            Self::REG(virt, _phys) => format!("t{}", virt),
            Self::ID(name, _) => name.to_string(),
            Self::INDEX(lop, rop) => format!("{}[{}]", lop.string(), rop.string()),
        }
    }
    pub fn to_op(lv: Self) -> Operand {
        if let Lvalue::REG(virt, phys) = lv {
            return Operand::REG(virt, phys);
        } else if let Lvalue::ID(name, _) = lv {
            return Operand::ID(name.to_string(), 0);
        } else {
            eprintln!("can't convert {} to operand", lv.string());
            return Operand::ID("(inv)".to_string(), 0);
        }
    }
}
#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    INTLIT(i128),
    CHARLIT(char),
    REG(Virtual, Physical),
    ID(String, Physical),
    CALL(String, usize),
    INDEX(Box<Operand>, Box<Operand>),
}
impl Operand {
    pub fn string(&self) -> String {
        match self {
            Self::CHARLIT(value) => format!("'{}'", value),
            Self::INTLIT(value) => format!("{}", value),
            Self::REG(virt, _phys) => format!("t{}", virt),
            Self::ID(name, _) => name.to_string(),
            Self::INDEX(lop, rop) => format!("{}[{}]", lop.string(), rop.string()),
            Self::CALL(func, argc) => format!("call {}, {}", func, argc),
        }
    }
}
#[derive(Clone)]
pub enum Tac {
    EX(Lvalue, String, Operand, Operand),
    UNEX(Lvalue, String, Operand),
    RET(Operand),
    PARAM(Operand),
    LET(Lvalue, Operand),
    IFF(Operand, String),
    GOTO(String),
    LABEL(String),
}
impl Tac {
    pub fn string(&self) -> String {
        match self {
            Self::LABEL(name) => format!("{}:", name),
            Self::EX(lv, op, lop, rop) => format!(
                "{} <- {} {} {}",
                lv.string(),
                lop.string(),
                op,
                rop.string()
            ),
            Self::UNEX(lv, op, lop) => format!("{} <- {}{}", lv.string(), op, lop.string(),),
            Self::RET(op) => format!("ret {}", op.string()),
            Self::LET(lv, op) => format!("{} <- {}", lv.string(), op.string()),
            Self::IFF(cond, label) => format!("ifFalse {} goto {}", cond.string(), label),
            Self::GOTO(label) => format!("goto {}", label),
            Self::PARAM(arg) => format!("param {}", arg.string()),
        }
    }
}
