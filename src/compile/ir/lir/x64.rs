type REG = usize;
pub enum IR {
    STOREREG(REG, REG),
    STOREIMM(REG, i128),
    ADDREG(REG, REG),
    ADDIMM(REG, i128),
    RETURNREG(REG),
    RETURNIMM(i128),
    LABEL(String),
    JMP(String),
}
