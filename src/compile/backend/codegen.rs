use super::super::ir::lir::x64;
use super::super::ir::tac::{Operand, Tac};
static X64_REGS: [&str; 9] = ["rax", "rdx", "rcx", "rdi", "rsi", "r8", "r9", "r10", "r11"];
fn gr(n: &usize) -> &str {
    X64_REGS[*n]
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
                    if let Operand::REG(_virt, phys) = lv {
                        self.ex_reg_id(phys, op, lop, rop);
                    } else if let Operand::ID(_name, phys) = lv {
                        self.ex_reg_id(phys, op, lop, rop);
                    }
                }
                Tac::RET(op) => {
                    if let Operand::REG(_virt, phys) = op {
                        self.lirs.push(x64::IR::RETURNREG(*phys));
                    } else if let Operand::INTLIT(value) = op {
                        self.lirs.push(x64::IR::RETURNIMM(*value));
                    } else if let Operand::ID(_name, phys) = op {
                        self.lirs.push(x64::IR::RETURNREG(*phys));
                    }
                }
                Tac::LET(lv, op) => {
                    if let Operand::ID(_name, phys) = lv {
                        if let Operand::REG(_virt, p) = op {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        } else if let Operand::INTLIT(v) = op {
                            self.lirs.push(x64::IR::STOREIMM(*phys, *v));
                        } else if let Operand::ID(_name, p) = op {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                Tac::LABEL(name) => {
                    self.lirs.push(x64::IR::LABEL(name.to_string()));
                }
                _ => (),
            }
        }
    }
    fn ex_reg_id(&mut self, phys: &usize, op: &String, lop: &Operand, rop: &Operand) {
        if let Operand::REG(_virt, p) = lop {
            match op.as_str() {
                "+" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::ADDREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::ADDIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::ADDREG(*p, *p2));
                    }
                }
                "-" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::SUBREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::SUBIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::SUBREG(*p, *p2));
                    }
                }
                "*" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::MULREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::MULIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::MULREG(*p, *p2));
                    }
                }
                "/" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::DIVREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::DIVIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::DIVREG(*p, *p2));
                    }
                }
                "%" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::MODREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::MODIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::MODREG(*p, *p2));
                    }
                }
                "<<" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::LSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*p, *p2));
                    }
                }
                ">>" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::RSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*p, *p2));
                    }
                }
                "<" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::LTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LTREG(*p, *p2));
                    }
                }
                "<=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::LTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*p, *p2));
                    }
                }
                ">" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::GTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::GTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::GTREG(*p, *p2));
                    }
                }
                ">=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::GTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*p, *p2));
                    }
                }
                "==" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::EQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::EQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::EQREG(*p, *p2));
                    }
                }
                "!=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::NTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*p, *p2));
                    }
                }
                _ => (),
            }
        } else if let Operand::ID(_name, p) = lop {
            match op.as_str() {
                "+" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::ADDREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::ADDIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::ADDREG(*p, *p2));
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                "-" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::SUBREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::SUBIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::SUBREG(*p, *p2));
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                "*" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::MULREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::MULIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::MULREG(*p, *p2));
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                "/" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::DIVREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::DIVIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::DIVREG(*p, *p2));
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                "%" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::MODREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::MODIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::MODREG(*p, *p2));
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                "<<" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::LSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*p, *p2));
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                ">>" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::RSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*p, *p2));
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                    }
                }
                "<" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::LTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LTREG(*p, *p2));
                    }
                }
                "<=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::LTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*p, *p2));
                    }
                }
                ">" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::GTREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::GTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::GTREG(*p, *p2));
                    }
                }
                ">=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::GTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*p, *p2));
                    }
                }
                "==" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::EQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::EQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::EQREG(*p, *p2));
                    }
                }
                "!=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*p, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        if *phys != *p {
                            self.lirs.push(x64::IR::STOREREG(*phys, *p));
                        }
                        self.lirs.push(x64::IR::NTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*p, *p2));
                    }
                }
                _ => (),
            }
        } else if let Operand::INTLIT(value) = lop {
            self.lirs.push(x64::IR::STOREIMM(*phys, *value));
            match op.as_str() {
                "+" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::ADDREG(*phys, *p2));
                    } else if let Operand::INTLIT(v2) = rop {
                        self.lirs.push(x64::IR::ADDIMM(*phys, *v2));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::ADDREG(*phys, *p2));
                    }
                }
                "-" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::SUBREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::SUBIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::SUBREG(*phys, *p2));
                    }
                }
                "*" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::MULREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MULIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::MULREG(*phys, *p2));
                    }
                }
                "/" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::DIVREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::DIVIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::DIVREG(*phys, *p2));
                    }
                }
                "%" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::MODREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::MODIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::MODREG(*phys, *p2));
                    }
                }
                "<<" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LSHIFTREG(*phys, *p2));
                    }
                }
                ">>" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::RSHIFTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::RSHIFTREG(*phys, *p2));
                    }
                }
                "<" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LTREG(*phys, *p2));
                    }
                }
                ">" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::GTREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::GTREG(*phys, *p2));
                    }
                }
                "<=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::LTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::LTEQREG(*phys, *p2));
                    }
                }
                ">=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::GTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::GTEQREG(*phys, *p2));
                    }
                }
                "==" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::EQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::EQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::EQREG(*phys, *p2));
                    }
                }
                "!=" => {
                    if let Operand::REG(_virt, p2) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*phys, *p2));
                    } else if let Operand::INTLIT(value) = rop {
                        self.lirs.push(x64::IR::NTEQIMM(*phys, *value));
                    } else if let Operand::ID(_name, p2) = rop {
                        self.lirs.push(x64::IR::NTEQREG(*phys, *p2));
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
                    out += &(format!("  mov {}, {}\n", gr(dst), gr(src)).as_str());
                }
                x64::IR::STOREIMM(dst, value) => {
                    out += &(format!("  mov {}, {}\n", gr(dst), value).as_str());
                }
                x64::IR::ADDREG(dst, src) => {
                    out += &(format!("  add {}, {}\n", gr(dst), gr(src)).as_str());
                }
                x64::IR::ADDIMM(dst, value) => {
                    out += &(format!("  add {}, {}\n", gr(dst), value).as_str());
                }
                x64::IR::SUBREG(dst, src) => {
                    out += &(format!("  sub {}, {}\n", gr(dst), gr(src)).as_str());
                }
                x64::IR::SUBIMM(dst, value) => {
                    out += &(format!("  sub {}, {}\n", gr(dst), value).as_str());
                }
                x64::IR::MULREG(dst, src) => {
                    out += &(format!("  imul {}, {}\n", gr(dst), gr(src)).as_str());
                }
                x64::IR::MULIMM(dst, value) => {
                    out += &(format!("  imul {}, {}\n", gr(dst), value).as_str());
                }
                x64::IR::DIVREG(dst, src) => {
                    out += "  push rax\n";
                    out += "  push rdx\n";
                    out += &(format!("  mov rax, {}\n", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  idiv {}\n", gr(src)).as_str());
                    out += &(format!("  mov {}, rax\n", gr(dst)).as_str());
                    out += "  pop rdx\n";
                    out += "  pop rax\n";
                }
                x64::IR::DIVIMM(dst, value) => {
                    out += "  push rax\n";
                    out += "  push rdx\n";
                    out += &(format!("  mov rax, {}\n", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  mov r12, {}\n", value).as_str());
                    out += "  idiv r12\n";
                    out += &(format!("  mov {}, rax\n", gr(dst)).as_str());
                    out += "  pop rdx\n";
                    out += "  pop rax\n";
                }
                x64::IR::MODREG(dst, src) => {
                    out += "  push rax\n";
                    out += "  push rdx\n";
                    out += &(format!("  mov rax, {}\n", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  idiv {}\n", gr(src)).as_str());
                    out += &(format!("  mov {}, rdx\n", gr(dst)).as_str());
                    out += "  pop rdx\n";
                    out += "  pop rax\n";
                }
                x64::IR::MODIMM(dst, value) => {
                    out += "  push rax\n";
                    out += "  push rdx\n";
                    out += &(format!("  mov rax, {}\n", gr(dst)).as_str());
                    out += "  cqo\n";
                    out += &(format!("  mov r12, {}\n", value).as_str());
                    out += "  idiv r12\n";
                    out += &(format!("  mov {}, rdx\n", gr(dst)).as_str());
                    out += "  pop rdx\n";
                    out += "  pop rax\n";
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
                x64::IR::RSHIFTREG(dst, src) => {
                    out += "  push rcx\n";
                    out += &(format!("  mov rcx, {}\n", gr(src)).as_str());
                    out += &(format!("  sar {}, cl\n", gr(dst)).as_str());
                    out += "  pop rcx\n";
                }
                x64::IR::RSHIFTIMM(dst, value) => {
                    out += &(format!("  sar {}, {}\n", gr(dst), value));
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
                x64::IR::LABEL(name) => {
                    out += &(format!("{}:\n", name).as_str());
                }
                x64::IR::JMP(label) => {
                    out += &(format!("  {}\n", label).as_str());
                }
                x64::IR::RETURNREG(r) => {
                    if *r != 0 {
                        out += &(format!("  mov rax, {}\n", gr(r)).as_str());
                    }
                    out += "  ret\n";
                }
                x64::IR::RETURNIMM(value) => {
                    out += &(format!("  mov rax, {}\n", value).as_str());
                    out += "  ret\n";
                }
            }
        }
        out
    }
}
