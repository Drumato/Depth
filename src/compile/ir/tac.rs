type Virtual = usize;
type Physical = usize;
pub enum Lvalue {
    REG(Virtual, Physical),
    ID(String),
}
impl Lvalue {
    pub fn string(&self) -> String {
        match self {
            Self::REG(virt, _phys) => format!("t{}", virt),
            Self::ID(name) => name.to_string(),
        }
    }
}
pub enum Operand {
    INTLIT(i128),
    REG(Virtual, Physical),
    ID(String),
}
impl Operand {
    pub fn string(&self) -> String {
        match self {
            Self::INTLIT(value) => format!("{}", value),
            Self::REG(virt, _phys) => format!("t{}", virt),
            Self::ID(name) => name.to_string(),
        }
    }
}
pub enum Tac {
    FUNC(String),
    EX(Lvalue, String, Operand, Operand),
    UNEX(Lvalue, String, Operand),
    RET(Operand),
}
impl Tac {
    pub fn string(&self) -> String {
        match self {
            Self::FUNC(name) => format!("{}:", name),
            Self::EX(lv, op, lop, rop) => format!(
                "{} <- {} {} {}",
                lv.string(),
                lop.string(),
                op,
                rop.string()
            ),
            Self::UNEX(lv, op, lop) => format!("{} <- {}{}", lv.string(), op, lop.string(),),
            Self::RET(op) => format!("ret {}", op.string()),
        }
    }
}
