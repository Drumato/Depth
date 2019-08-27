type Offset = usize;
type Reg = usize;
pub enum HIR {
    PROLOGUE(usize),
    EPILOGUE,
    IMM(Reg, i128),
    IMMCHAR(Reg, char),
    ADD(Reg, Reg),
    SUB(Reg, Reg),
    MUL(Reg, Reg),
    DIV(Reg, Reg),
    MOD(Reg, Reg),
    LSHIFT(Reg, Reg),
    RSHIFT(Reg, Reg),
    LT(Reg, Reg),
    LTEQ(Reg, Reg),
    GT(Reg, Reg),
    GTEQ(Reg, Reg),
    EQ(Reg, Reg),
    NTEQ(Reg, Reg),
    NEGATIVE(Reg),
    DEREFERENCE(Reg, Offset),
    ADDRESS(Reg, Offset),
    RETREG(Reg),
    RETURN,
    SYMBOL(String),
    LABEL(usize),
    CMP(Reg, usize),
    JUMP(usize),
    STORE(Offset, Reg, usize),        //offset,reg,size
    LOAD(Reg, Offset, usize),         //reg,offset,size
    INDEXLOAD(Reg, Reg, i128, usize), //reg,reg,index,size
    CALL(String, Vec<Reg>),
}

impl HIR {
    pub fn string(&self) -> String {
        match self {
            HIR::PROLOGUE(size) => format!("function prologue<allocate {}>", size),
            HIR::EPILOGUE => format!("function epilogue"),
            HIR::IMM(reg, val) => format!("immediate {} to {}", val, reg),
            HIR::IMMCHAR(reg, char_val) => format!("immediate-char {} to {}", char_val, reg),
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
            HIR::ADDRESS(reg, offset) => format!("address of {} from {}", reg, offset),
            HIR::DEREFERENCE(reg, offset) => format!("dereference of {} from {}", reg, offset),
            HIR::RETREG(reg) => format!("mov retreg, {}", reg),
            HIR::RETURN => "return".to_string(),
            HIR::SYMBOL(name) => format!("symbol '{}'", name),
            HIR::LABEL(num) => format!("LABEL {}", num),
            HIR::CMP(reg, label) => format!("compare between {} and 0, then jump {}", reg, label),
            HIR::JUMP(num) => format!("JUMP {}", num),
            HIR::CALL(func, _) => format!("CALL {}", func),
            HIR::STORE(offset, reg, size) => {
                format!("STORE<{}> into -{}[rbp] from {}", size, offset, reg)
            }
            HIR::INDEXLOAD(reg1, reg2, index, size) => format!(
                "INDEXLOAD into {} from [{} + {} * {}]",
                reg1, reg2, index, size
            ),
            HIR::LOAD(reg, offset, size) => {
                format!("LOAD<{}> into {} from -{}[rbp]", size, reg, offset)
            }
        }
    }
}
