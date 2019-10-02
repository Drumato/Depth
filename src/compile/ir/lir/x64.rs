type REG = usize;
type OFFSET = usize;
type SYMBOL = String;
pub enum IR {
    PROLOGUE(OFFSET),
    REGIMM(REG, i128),
    STOREREG(REG, REG),
    STOREIMM(REG, i128),
    STOREMEM(REG, OFFSET),
    STORECALL(REG, SYMBOL),
    ADDREG(REG, REG),
    ADDIMM(REG, i128),
    ADDMEM(REG, OFFSET),
    SUBREG(REG, REG),
    SUBIMM(REG, i128),
    SUBMEM(REG, OFFSET),
    MULREG(REG, REG),
    MULIMM(REG, i128),
    MULMEM(REG, OFFSET),
    DIVREG(REG, REG),
    DIVIMM(REG, i128),
    DIVMEM(REG, OFFSET),
    MODREG(REG, REG),
    MODIMM(REG, i128),
    MODMEM(REG, OFFSET),
    LSHIFTREG(REG, REG),
    LSHIFTIMM(REG, i128),
    LSHIFTMEM(REG, OFFSET),
    RSHIFTREG(REG, REG),
    RSHIFTIMM(REG, i128),
    RSHIFTMEM(REG, OFFSET),
    LTREG(REG, REG),
    LTIMM(REG, i128),
    LTMEM(REG, OFFSET),
    GTREG(REG, REG),
    GTIMM(REG, i128),
    GTMEM(REG, OFFSET),
    LTEQREG(REG, REG),
    LTEQIMM(REG, i128),
    LTEQMEM(REG, OFFSET),
    GTEQREG(REG, REG),
    GTEQIMM(REG, i128),
    GTEQMEM(REG, OFFSET),
    EQREG(REG, REG),
    EQIMM(REG, i128),
    EQMEM(REG, OFFSET),
    NTEQREG(REG, REG),
    NTEQIMM(REG, i128),
    NTEQMEM(REG, OFFSET),
    NEGREG(REG),
    ADDRESSMEM(REG, OFFSET),
    DEREFREG(REG),
    LOADMEM(REG, OFFSET),
    LOADREG(REG, REG),
    RETURNREG(REG),
    RETURNIMM(i128),
    RETURNMEM(OFFSET),
    RETURNCALL(SYMBOL),
    CALL(SYMBOL),
    LABEL(SYMBOL),
    JMP(String),
}
