type REG = usize;
pub enum IR {
    STOREREG(REG, REG),
    STOREIMM(REG, i128),
    ADDREG(REG, REG),
    ADDIMM(REG, i128),
    SUBREG(REG, REG),
    SUBIMM(REG, i128),
    MULREG(REG, REG),
    MULIMM(REG, i128),
    DIVREG(REG, REG),
    DIVIMM(REG, i128),
    RETURNREG(REG),
    RETURNIMM(i128),
    LABEL(String),
    JMP(String),
}
