pub enum HIR {
    PROLOGUE,
    EPILOGUE,
    IMM(usize, i128),
    ADD(usize, usize),
    SUB(usize, usize),
    MUL(usize, usize),
    DIV(usize, usize),
    MOD(usize, usize),
    LSHIFT(usize, usize),
    RSHIFT(usize, usize),
    LT(usize, usize),
    LTEQ(usize, usize),
    GT(usize, usize),
    GTEQ(usize, usize),
    EQ(usize, usize),
    NTEQ(usize, usize),
    NEGATIVE(usize),
    RETURN(usize),
    SYMBOL(String),
    LABEL(usize),
    CMP(usize, usize),
    JUMP(usize),
    STORE(usize, usize, usize), //offset,reg,size
    LOAD(usize, usize, usize),  //reg,offset,size
}

impl HIR {
    pub fn string(&self) -> String {
        match self {
            HIR::PROLOGUE => format!("function prologue"),
            HIR::EPILOGUE => format!("function epilogue"),
            HIR::IMM(reg, val) => format!("immediate {} to {}", val, reg),
            HIR::ADD(lr, rr) => format!("{} plus {}", lr, rr),
            HIR::SUB(lr, rr) => format!("{} minus {}", lr, rr),
            HIR::MUL(lr, rr) => format!("{} multiply {}", lr, rr),
            HIR::DIV(lr, rr) => format!("{} divided by {}", lr, rr),
            HIR::MOD(lr, rr) => format!("reminder of '{} divided by {}'", lr, rr),
            HIR::LSHIFT(lr, rr) => format!("lshift {} {} times", lr, rr),
            HIR::RSHIFT(lr, rr) => format!("rshift {} {} times", lr, rr),
            HIR::LT(lr, rr) => format!("{} less than {}", lr, rr),
            HIR::LTEQ(lr, rr) => format!("{} less than or equal {}", lr, rr),
            HIR::GT(lr, rr) => format!("{} greater than {}", lr, rr),
            HIR::GTEQ(lr, rr) => format!("{} greater than or equal {}", lr, rr),
            HIR::EQ(lr, rr) => format!("{} equal {}", lr, rr),
            HIR::NTEQ(lr, rr) => format!("{} not equal {}", lr, rr),
            HIR::NEGATIVE(reg) => format!("negative {} ", reg),
            HIR::RETURN(reg) => format!("return {}", reg),
            HIR::SYMBOL(name) => format!("symbol '{}'", name),
            HIR::LABEL(num) => format!("LABEL {}", num),
            HIR::CMP(reg, label) => format!("compare between {} and 0, then jump {}", reg, label),
            HIR::JUMP(num) => format!("JUMP {}", num),
            HIR::STORE(offset, reg, size) => {
                format!("STORE<{}> into -{}[rbp] from {}", size, offset, reg)
            }
            HIR::LOAD(reg, offset, size) => {
                format!("LOAD<{}> into {} from -{}[rbp]", size, reg, offset)
            }
        }
    }
}
