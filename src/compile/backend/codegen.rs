use super::super::ir::lir::x64;
use super::super::ir::tac::{Operand, Tac};
static X64_REGS: [&str; 9] = [
    "r10", "r11", "r12", "r13", "r14", "r15", "rax", "rdx", "rcx",
];
static X64_ARGREGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
static mut ARGREG: usize = 0;
static RETURN_REG: usize = 6;
fn gr(n: &usize) -> &str {
    X64_REGS[*n]
}
fn argr(r: usize) -> &'static str {
    X64_ARGREGS[r]
}
pub fn genx64(tacs: Vec<Tac>) -> String {
    let mut generator = Generator::new(tacs);
    generator.gen_ir();
    generator.emit()
}
struct Generator {
    tacs: Vec<Tac>,
    lirs: Vec<x64::IR>,
}
impl Generator {
    fn new(tacs: Vec<Tac>) -> Self {
        Self {
            tacs,
            lirs: Vec::new(),
        }
    }
    fn gen_ir(&mut self) {
        let tacs = self.tacs.clone();
        for t in tacs.iter() {
            match t {
                Tac::EX(lv, op, lop, rop) => {
                    if let Operand::REG(_virt, phys, _oind) = lv {
                        self.ex_reg(phys, op, lop, rop);
                    }
                }
                Tac::UNEX(lv, op, lop) => {
                    if let Operand::REG(_virt, phys, _oind) = lv {
                        self.unex_reg(phys, op, lop);
                    }
                }
                Tac::RET(op) => {
                    if let Operand::REG(_virt, phys, _oind) = op {
                        self.lirs.push(x64::IR::RETURNREG(*phys));
                    } else if let Operand::INTLIT(value) = op {
                        self.lirs.push(x64::IR::RETURNIMM(*value));
                    } else if let Operand::ID(_name, offset, _oind) = op {
                        self.lirs.push(x64::IR::RETURNMEM(*offset));
                    } else if let Operand::CALL(name, _length) = op {
                        self.lirs.push(x64::IR::RETURNCALL(name.to_owned()));
                    }
                }
                Tac::LET(lv, op) => {
                    if let Operand::ID(_name, offset, oind) = lv {
                        if let Some(bop) = oind {
                            let ind_op: Operand = *bop.clone();
                            if let Operand::INTLIT(idx) = ind_op {
                                if let Operand::REG(_virt, p, _oind) = op {
                                    self.lirs
                                        .push(x64::IR::STOREREG(*offset - (idx as usize) * 8, *p));
                                } else if let Operand::INTLIT(v) = op {
                                    self.lirs
                                        .push(x64::IR::STOREIMM(*offset - (idx as usize) * 8, *v));
                                } else if let Operand::ID(_name, off, _oind) = op {
                                    self.lirs.push(x64::IR::STOREMEM(
                                        *offset - (idx as usize) * 8,
                                        *off,
                                    ));
                                } else if let Operand::CALL(name, _length) = op {
                                    self.lirs.push(x64::IR::STORECALL(
                                        *offset - (idx as usize) * 8,
                                        name.to_owned(),
                                    ));
                                }
                            }
                        } else {
                            if let Operand::REG(_virt, p, _oind) = op {
                                self.lirs.push(x64::IR::STOREREG(*offset, *p));
                            } else if let Operand::INTLIT(v) = op {
                                self.lirs.push(x64::IR::STOREIMM(*offset, *v));
                            } else if let Operand::ID(n, off, _oind) = op {
                                if !n.contains("Array") {
                                    self.lirs.push(x64::IR::STOREMEM(*offset, *off));
                                }
                            } else if let Operand::CALL(name, _length) = op {
                                self.lirs.push(x64::IR::STORECALL(*offset, name.to_owned()));
                            }
                        }
                    }
                }
                Tac::LABEL(name) => {
                    self.lirs.push(x64::IR::LABEL(name.to_string()));
                }
                Tac::FUNCNAME(name) => {
                    self.lirs.push(x64::IR::LABEL(name.to_string()));
                }
                Tac::PROLOGUE(stack_offset) => {
                    self.lirs.push(x64::IR::PROLOGUE(*stack_offset));
                }
                Tac::PUSHARG(reg, arg) => {
                    self.lirs.push(x64::IR::PUSHARG(*reg, *arg));
                }
                Tac::PARAM(reg, op) => {
                    if let Operand::REG(_virt, p, _oind) = op {
                        self.lirs.push(x64::IR::ARGREG(*reg, *p));
                    } else if let Operand::INTLIT(v) = op {
                        self.lirs.push(x64::IR::ARGIMM(*reg, *v));
                    } else if let Operand::ID(_name, offset, _oind) = op {
                        self.lirs.push(x64::IR::ARGMEM(*reg, *offset));
                    } else if let Operand::CALL(name, _length) = op {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::ARGREG(*reg, 0));
                    }
                }
                Tac::IFF(op, label) => {
                    if let Operand::REG(_virt, p, _oind) = op {
                        self.lirs.push(x64::IR::CMPREG(*p));
                    } else if let Operand::ID(_name, offset, _oind) = op {
                        self.lirs.push(x64::IR::CMPMEM(*offset));
                    } else if let Operand::CALL(name, _length) = op {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::CMPREG(0));
                    }
                    self.lirs.push(x64::IR::JZ(label.to_owned()));
                }
                Tac::GOTO(label) => {
                    self.lirs.push(x64::IR::JMP(label.to_owned()));
                }
            }
            unsafe {
                ARGREG = 0;
            };
        }
    }
    fn unex_reg(&mut self, phys: &usize, op: &String, lop: &Operand) {
        if let Operand::REG(_virs, p, _oind) = lop {
            match op.as_str() {
                "-" => {
                    self.lirs.push(x64::IR::NEGREG(*p));
                }
                "*" => {
                    self.lirs.push(x64::IR::DEREFREG(*p));
                }
                _ => (),
            }
            self.lirs.push(x64::IR::LOADREG(*phys, *p));
        } else if let Operand::INTLIT(value) = lop {
            self.lirs.push(x64::IR::REGIMM(*phys, *value));
            match op.as_str() {
                "-" => {
                    self.lirs.push(x64::IR::NEGREG(*phys));
                }
                _ => (),
            }
        } else if let Operand::ID(_virt, offset, _oind) = lop {
            match op.as_str() {
                "-" => {
                    self.lirs.push(x64::IR::LOADMEM(*phys, *offset));
                    self.lirs.push(x64::IR::NEGREG(*phys));
                }
                "&" => {
                    self.lirs.push(x64::IR::ADDRESSMEM(*phys, *offset));
                }
                "*" => {
                    self.lirs.push(x64::IR::LOADMEM(*phys, *offset));
                    self.lirs.push(x64::IR::DEREFREG(*phys));
                }
                _ => (),
            }
        }
    }
    fn ex_reg(&mut self, phys: &usize, op: &String, lop: &Operand, rop: &Operand) {
        if let Operand::REG(_virt, p, _oind) = lop {
            match op.as_str() {
                "+" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::ADDREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::ADDIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::ADDMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::ADDREG(*p, 0));
                    }
                }
                "-" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::SUBREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::SUBIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::SUBMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::SUBREG(*p, 0));
                    }
                }
                "*" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MULREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MULIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MULMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::MULREG(*p, 0));
                    }
                }
                "/" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::DIVREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::DIVIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::DIVMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::DIVREG(*p, 0));
                    }
                }
                "%" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MODREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MODIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MODMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::MODREG(*p, 0));
                    }
                }
                "<<" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LSHIFTIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LSHIFTMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LSHIFTREG(*p, 0));
                    }
                }
                ">>" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::RSHIFTIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::RSHIFTMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::RSHIFTREG(*p, 0));
                    }
                }
                "<" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LTREG(*p, 0));
                    }
                }
                "<=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTEQIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTEQMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LTEQREG(*p, 0));
                    }
                }
                ">" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::GTREG(*p, 0));
                    }
                }
                ">=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTEQIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTEQMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::GTEQREG(*p, 0));
                    }
                }
                "==" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::EQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::EQIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::EQMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::EQREG(*p, 0));
                    }
                }
                "!=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::NTEQIMM(*p, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::NTEQMEM(*p, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::NTEQREG(*p, 0));
                    }
                }
                _ => (),
            }
            self.lirs.push(x64::IR::LOADREG(*phys, *p));
        } else if let Operand::ID(_name, offset, _oind) = lop {
            self.lirs.push(x64::IR::LOADMEM(*phys, *offset));
            match op.as_str() {
                "+" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::ADDREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::ADDIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::ADDMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::ADDREG(*phys, 0));
                    }
                }
                "-" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::SUBREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::SUBIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::SUBMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::SUBREG(*phys, 0));
                    }
                }
                "*" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MULREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MULIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MULMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::MULREG(*phys, 0));
                    }
                }
                "/" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::DIVREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::DIVIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::DIVMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::DIVREG(*phys, 0));
                    }
                }
                "%" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MODREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MODIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MODMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::MODREG(*phys, 0));
                    }
                }
                "<<" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LSHIFTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LSHIFTREG(*phys, 0));
                    }
                }
                ">>" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::RSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::RSHIFTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::RSHIFTREG(*phys, 0));
                    }
                }
                "<" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LTREG(*phys, 0));
                    }
                }
                "<=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTEQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LTEQREG(*phys, 0));
                    }
                }
                ">" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::GTREG(*phys, 0));
                    }
                }
                ">=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTEQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::GTREG(*phys, 0));
                    }
                }
                "==" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::EQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::EQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::EQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::STOREREG(*offset, 0));
                    }
                }
                "!=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::NTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::NTEQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::STOREREG(*offset, 0));
                    }
                }
                _ => (),
            }
        } else if let Operand::INTLIT(value) = lop {
            self.lirs.push(x64::IR::REGIMM(*phys, *value));
            match op.as_str() {
                "+" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::ADDREG(*phys, *p2));
                    } else if let Operand::INTLIT(v2) = rop {
                        self.lirs.push(x64::IR::ADDIMM(*phys, *v2));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::ADDMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::ADDREG(*phys, 0));
                    }
                }
                "-" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::SUBREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::SUBIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::SUBMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::SUBREG(*phys, 0));
                    }
                }
                "*" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MULREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MULIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MULMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::MULREG(*phys, 0));
                    }
                }
                "/" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::DIVREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::DIVIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::DIVMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::DIVREG(*phys, 0));
                    }
                }
                "%" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MODREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MODIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::MODMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::MODREG(*phys, 0));
                    }
                }
                "<<" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LSHIFTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LSHIFTREG(*phys, 0));
                    }
                }
                ">>" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::RSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::RSHIFTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::RSHIFTREG(*phys, 0));
                    }
                }
                "<" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LTREG(*phys, 0));
                    }
                }
                ">" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::GTREG(*phys, 0));
                    }
                }
                "<=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::LTEQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::LTEQREG(*phys, 0));
                    }
                }
                ">=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::GTEQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::GTEQREG(*phys, 0));
                    }
                }
                "==" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::EQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::EQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::EQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::EQREG(*phys, 0));
                    }
                }
                "!=" => {
                    if let Operand::REG(_virt, p2, _oind) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::NTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2, _oind) = rop {
                        self.lirs.push(x64::IR::NTEQMEM(*phys, *p2));
                    } else if let Operand::CALL(name, _length) = rop {
                        self.lirs.push(x64::IR::CALL(name.to_owned()));
                        self.lirs.push(x64::IR::NTEQREG(*phys, 0));
                    }
                }
                _ => (),
            }
        }
    }
    fn emit(&self) -> String {
        let mut out: String = String::new();
        for i in self.lirs.iter() {
            match i {
                x64::IR::STOREREG(dst, src) => {
                    out += &(format!("  mov -{}[rbp], {}\n", dst, gr(src)).as_str());
                }
                x64::IR::STOREIMM(dst, value) => {
                    out += &(format!("  mov -{}[rbp], {}\n", dst, value).as_str());
                }
                x64::IR::STOREMEM(dst, offset) => {
                    out += &(format!("  mov r12, -{}[rbp]\n", offset).as_str());
                    out += &(format!("  mov -{}[rbp], r12\n", dst).as_str());
                }
                x64::IR::STORECALL(dst, symbol) => {
                    out += &(format!("  call {}\n", symbol).as_str());
                    out += &(format!("  mov -{}[rbp], rax\n", dst).as_str());
                }
                x64::IR::ADDREG(dst, src) => {
                    out += &(format!("  add {}, {}\n", gr(dst), gr(src)).as_str());
                }
                x64::IR::ADDIMM(dst, value) => {
                    out += &(format!("  add {}, {}\n", gr(dst), value).as_str());
                }
                x64::IR::ADDMEM(dst, offset) => {
                    out += &(format!("  add {}, -{}[rbp]\n", gr(dst), offset).as_str());
                }
                x64::IR::SUBREG(dst, src) => {
                    out += &(format!("  sub {}, {}\n", gr(dst), gr(src)).as_str());
                }
                x64::IR::SUBIMM(dst, value) => {
                    out += &(format!("  sub {}, {}\n", gr(dst), value).as_str());
                }
                x64::IR::SUBMEM(dst, offset) => {
                    out += &(format!("  sub {}, -{}[rbp]\n", gr(dst), offset).as_str());
                }
                x64::IR::MULREG(dst, src) => {
                    out += &(format!("  imul {}, {}\n", gr(dst), gr(src)).as_str());
                }
                x64::IR::MULIMM(dst, value) => {
                    out += &(format!("  imul {}, {}\n", gr(dst), value).as_str());
                }
                x64::IR::MULMEM(dst, offset) => {
                    out += &(format!("  imul {}, -{}[rbp]\n", gr(dst), offset).as_str());
                }
                x64::IR::DIVREG(dst, src) => {
                    out += &(format!("  mov rax, {}", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  idiv {}", gr(src)).as_str());
                    out += &(format!("  mov {}, rax", gr(dst)).as_str());
                }
                x64::IR::DIVIMM(dst, value) => {
                    out += &(format!("  mov rax, {}", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  idiv {}", value).as_str());
                    out += &(format!("  mov {}, rax", gr(dst)).as_str());
                }
                x64::IR::DIVMEM(dst, offset) => {
                    out += &(format!("  mov rax, {}", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  idiv -{}[rbp]", offset).as_str());
                    out += &(format!("  mov {}, rax", gr(dst)).as_str());
                }
                x64::IR::MODREG(dst, src) => {
                    out += "  push rax\n";
                    out += "  push rdx\n";
                    out += &(format!("  mov rax, {}\n", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  idiv {}\n", gr(src)).as_str());
                    out += "  mov r12, rdx\n";
                    out += "  pop r13\n";
                    out += "  pop r13\n";
                    out += &(format!("  mov {}, r12\n", gr(dst)).as_str());
                }
                x64::IR::MODIMM(dst, value) => {
                    out += "  push rax\n";
                    out += "  push rdx\n";
                    out += &(format!("  mov rax, {}\n", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  mov r12, {}\n", value).as_str());
                    out += "  idiv r12\n";
                    out += "  mov r12, rdx\n";
                    out += "  pop r13\n";
                    out += "  pop r13\n";
                    out += &(format!("  mov {}, r12\n", gr(dst)).as_str());
                }
                x64::IR::MODMEM(dst, offset) => {
                    out += "  push rax\n";
                    out += "  push rdx\n";
                    out += &(format!("  mov rax, {}\n", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  idiv -{}[rbp]\n", offset).as_str());
                    out += "  mov r12, rdx\n";
                    out += "  pop r13\n";
                    out += "  pop r13\n";
                    out += &(format!("  mov {}, r12\n", gr(dst)).as_str());
                }
                x64::IR::LSHIFTREG(dst, src) => {
                    out += "  push rcx\n";
                    out += &(format!("  mov rcx, {}\n", gr(src)).as_str());
                    out += &(format!("  sal {}, cl\n", gr(dst)).as_str());
                    out += "  pop rcx\n";
                }
                x64::IR::LSHIFTIMM(dst, value) => {
                    out += &(format!("  sal {}, {}\n", gr(dst), value));
                }
                x64::IR::LSHIFTMEM(dst, offset) => {
                    out += "  push rcx\n";
                    out += &(format!("  mov rcx, -{}[rbp]\n", offset).as_str());
                    out += &(format!("  sal {}, cl\n", gr(dst)).as_str());
                    out += "  pop rcx\n";
                }
                x64::IR::RSHIFTREG(dst, src) => {
                    out += "  push rcx\n";
                    out += &(format!("  mov rcx, {}\n", gr(src)).as_str());
                    out += &(format!("  sar {}, cl\n", gr(dst)).as_str());
                    out += "  pop rcx\n";
                }
                x64::IR::RSHIFTIMM(dst, value) => {
                    out += &(format!("  sar {}, {}\n", gr(dst), value));
                }
                x64::IR::RSHIFTMEM(dst, offset) => {
                    out += "  push rcx\n";
                    out += &(format!("  mov rcx, -{}[rbp]\n", offset).as_str());
                    out += &(format!("  sar {}, cl\n", gr(dst)).as_str());
                    out += "  pop rcx\n";
                }
                x64::IR::LTREG(dst, src) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), gr(src)).as_str());
                    out += "  setl al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::LTIMM(dst, value) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), value).as_str());
                    out += "  setl al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::LTMEM(dst, offset) => {
                    out += &(format!("  cmp {}, -{}[rbp]\n", gr(dst), offset).as_str());
                    out += "  setl al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::GTREG(dst, src) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), gr(src)).as_str());
                    out += "  setg al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::GTIMM(dst, value) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), value).as_str());
                    out += "  setg al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::GTMEM(dst, offset) => {
                    out += &(format!("  cmp {}, -{}[rbp]\n", gr(dst), offset).as_str());
                    out += "  setg al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::LTEQREG(dst, src) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), gr(src)).as_str());
                    out += "  setle al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::LTEQIMM(dst, value) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), value).as_str());
                    out += "  setle al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::LTEQMEM(dst, offset) => {
                    out += &(format!("  cmp {}, -{}[rbp]\n", gr(dst), offset).as_str());
                    out += "  setle al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::GTEQREG(dst, src) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), gr(src)).as_str());
                    out += "  setge al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::GTEQIMM(dst, value) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), value).as_str());
                    out += "  setge al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::GTEQMEM(dst, offset) => {
                    out += &(format!("  cmp {}, -{}[rbp]\n", gr(dst), offset).as_str());
                    out += "  setge al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::EQREG(dst, src) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), gr(src)).as_str());
                    out += "  sete al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::EQIMM(dst, value) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), value).as_str());
                    out += "  sete al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::EQMEM(dst, offset) => {
                    out += &(format!("  cmp {}, -{}[rbp]\n", gr(dst), offset).as_str());
                    out += "  sete al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::NTEQREG(dst, src) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), gr(src)).as_str());
                    out += "  setne al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::NTEQIMM(dst, value) => {
                    out += &(format!("  cmp {}, {}\n", gr(dst), value).as_str());
                    out += "  setne al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::NTEQMEM(dst, offset) => {
                    out += &(format!("  cmp {}, -{}[rbp]\n", gr(dst), offset).as_str());
                    out += "  setne al\n";
                    out += &(format!("  movzx {}, al\n", gr(dst)).as_str());
                }
                x64::IR::NEGREG(r) => {
                    out += &(format!("  neg {}\n", gr(r)));
                }
                x64::IR::ADDRESSMEM(r, offset) => {
                    out += &(format!("  lea {}, -{}[rbp]\n", gr(r), offset).as_str());
                }
                x64::IR::DEREFREG(r) => {
                    out += &(format!("  mov {}, [{}]\n", gr(r), gr(r)).as_str());
                }
                x64::IR::CALL(name) => {
                    out += &(format!("  call {}\n", name).as_str());
                    unsafe {
                        ARGREG = 0;
                    };
                }
                x64::IR::LABEL(name) => {
                    out += &(format!("{}:\n", name).as_str());
                }
                x64::IR::JMP(label) => {
                    out += &(format!("  jmp {}\n", label).as_str());
                }
                x64::IR::RETURNREG(r) => {
                    if *r != RETURN_REG {
                        out += &(format!("  mov rax, {}\n", gr(r)).as_str());
                    }
                    out += "  mov rsp, rbp\n";
                    out += "  pop rbp\n";
                    out += "  ret\n";
                }
                x64::IR::RETURNMEM(offset) => {
                    out += &(format!("  mov rax, -{}[rbp]\n", offset).as_str());
                    out += "  mov rsp, rbp\n";
                    out += "  pop rbp\n";
                    out += "  ret\n";
                }
                x64::IR::RETURNIMM(value) => {
                    out += &(format!("  mov rax, {}\n", value).as_str());
                    out += "  mov rsp, rbp\n";
                    out += "  pop rbp\n";
                    out += "  ret\n";
                }
                x64::IR::RETURNCALL(symbol) => {
                    out += &(format!("  call {}\n", symbol).as_str());
                    out += "  mov rsp, rbp\n";
                    out += "  pop rbp\n";
                    out += "  ret\n";
                }
                x64::IR::LOADMEM(r, offset) => {
                    out += &(format!("  mov {}, -{}[rbp]\n", gr(r), offset).as_str());
                }
                x64::IR::LOADREG(r, r2) => {
                    out += &(format!("  mov {}, {}\n", gr(r), gr(r2)).as_str());
                }
                x64::IR::REGIMM(r, value) => {
                    out += &(format!("  mov {}, {}\n", gr(r), value).as_str());
                }
                x64::IR::PUSHARG(r, offset) => {
                    out += &(format!("  mov -{}[rbp], {}\n", offset, argr(*r)).as_str());
                }
                x64::IR::ARGREG(r, r2) => {
                    out += &(format!("  mov {}, {}\n", argr(*r), gr(r2)));
                }
                x64::IR::ARGMEM(r, offset) => {
                    out += &(format!("  mov {}, -{}[rbp]\n", argr(*r), offset));
                }
                x64::IR::ARGIMM(r, v) => {
                    out += &(format!("  mov {}, {}\n", argr(*r), v));
                }
                x64::IR::PROLOGUE(offset) => {
                    out += "  push rbp\n";
                    out += "  mov rbp, rsp\n";
                    if *offset != 0 {
                        out += &(format!("  sub rsp, {}\n", !7 & offset + 7));
                    }
                }
                x64::IR::CMPREG(r) => {
                    out += &(format!("  cmp {}, 0\n", gr(r)).as_str());
                }
                x64::IR::CMPMEM(offset) => {
                    out += &(format!("  cmp -{}[rbp], 0\n", offset).as_str());
                }
                x64::IR::JZ(label) => {
                    out += &(format!("  jz {}\n", label).as_str());
                }
            }
        }
        out
    }
}
