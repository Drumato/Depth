use super::super::ir::hi::HIR;
use super::manager::Manager;
const FREE_REG: [&str; 6] = ["r10", "r11", "r12", "r13", "r14", "r15"];
const FREE_REG32: [&str; 6] = ["r10d", "r11d", "r12d", "r13d", "r14d", "r15d"];
const FREE_REG16: [&str; 6] = ["r10w", "r11w", "r12w", "r13w", "r14w", "r15w"];
const FREE_REG8: [&str; 6] = ["r10b", "r11b", "r12b", "r13b", "r14b", "r15b"];
const ARG_REG: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
fn argr(num: usize, size: usize) -> String {
    match size {
        8 => ARG_REG[num].to_string(),
        _ => ARG_REG[num].to_string(),
    }
}
fn gr(num: &usize, size: usize) -> String {
    match size {
        8 => FREE_REG[*num].to_string(),
        4 => FREE_REG32[*num].to_string(),
        2 => FREE_REG16[*num].to_string(),
        1 => FREE_REG8[*num].to_string(),
        _ => FREE_REG[*num].to_string(),
    }
}
impl Manager {
    pub fn genx64(&mut self) -> String {
        let mut out: String = String::with_capacity(4096);
        for ir in self.hirs.iter() {
            match ir {
                HIR::ADD(lr, rr) => {
                    out += format!("  add {}, {}\n", gr(lr, 64), gr(rr, 64)).as_str();
                }
                HIR::SUB(lr, rr) => {
                    out += format!("  sub {}, {}\n", gr(lr, 64), gr(rr, 64)).as_str();
                }
                HIR::MUL(lr, rr) => {
                    out += format!(
                        "  mov rax, {}\n  imul {}\n  mov {}, rax\n",
                        gr(lr, 64),
                        gr(rr, 64),
                        gr(lr, 8)
                    )
                    .as_str();
                }
                HIR::DIV(lr, rr) => {
                    out += self.division(lr, rr).as_str();
                }
                HIR::MOD(lr, rr) => {
                    out += self.division(lr, rr).as_str();
                    out += format!("  mov {}, rdx\n", gr(lr, 8)).as_str();
                }
                HIR::LSHIFT(lr, rr) => {
                    out += format!("  mov rcx, {}\n  sal {}, cl\n", gr(rr, 8), gr(lr, 8)).as_str();
                }
                HIR::RSHIFT(lr, rr) => {
                    out += format!("  mov rcx, {}\n  sar {}, cl\n", gr(rr, 8), gr(lr, 8)).as_str();
                }
                HIR::LT(lr, rr) => {
                    out += self.compare(lr, rr, "setl").as_str();
                }
                HIR::LTEQ(lr, rr) => {
                    out += self.compare(lr, rr, "setle").as_str();
                }
                HIR::GT(lr, rr) => {
                    out += self.compare(lr, rr, "setg").as_str();
                }
                HIR::GTEQ(lr, rr) => {
                    out += self.compare(lr, rr, "setge").as_str();
                }
                HIR::EQ(lr, rr) => {
                    out += self.compare(lr, rr, "sete").as_str();
                }
                HIR::NTEQ(lr, rr) => {
                    out += self.compare(lr, rr, "setne").as_str();
                }
                HIR::IMM(reg, val) => {
                    out += format!("  mov {}, {}\n", gr(reg, 8), val).as_str();
                }
                HIR::IMMCHAR(reg, char_val) => {
                    out += format!("  mov {}, {}\n", gr(reg, 4), *char_val as u32).as_str();
                }
                HIR::NEGATIVE(reg) => {
                    out += format!("  neg {}\n", gr(reg, 8)).as_str();
                }
                HIR::ADDRESS(reg, offset) => {
                    out += format!("  lea {}, -{}[rbp]\n", gr(reg, 8), offset).as_str();
                }
                HIR::DEREFERENCE(reg, _) => {
                    out += format!("  mov {}, [{}]\n", gr(reg, 8), gr(reg, 8)).as_str();
                }
                HIR::RETURN(reg) => {
                    out += format!("  mov rax, {}\n", gr(reg, 8)).as_str();
                    out += "  mov rsp, rbp\n";
                    out += "  pop r15\n";
                    out += "  pop r14\n";
                    out += "  pop r13\n";
                    out += "  pop r12\n";
                    out += "  pop rbp\n";
                    out += "  ret\n";
                }
                HIR::PROLOGUE(size) => {
                    out += "  push rbp\n";
                    out += "  push r12\n";
                    out += "  push r13\n";
                    out += "  push r14\n";
                    out += "  push r15\n";
                    out += "  mov rbp, rsp\n";
                    if size != &0 {
                        out += format!("  sub rsp, {}\n", !7 & size + 7).as_str();
                    }
                }
                HIR::SYMBOL(name) => {
                    out += format!("{}:\n", name).as_str();
                }
                HIR::JUMP(label) => {
                    out += format!("  jmp .L{}\n", label).as_str();
                }
                HIR::LABEL(label) => {
                    out += format!(".L{}:\n", label).as_str();
                }
                HIR::CMP(reg, label) => {
                    out += format!("  cmp {}, 0\n", gr(reg, 8)).as_str();
                    out += format!("  je .L{}\n", label).as_str();
                }
                HIR::STORE(offset, reg, size) => {
                    out += format!("  mov -{}[rbp], {}\n", offset, gr(reg, *size)).as_str();
                }
                HIR::LOAD(reg, offset, size) => {
                    out += format!("  mov {}, -{}[rbp]\n", gr(reg, *size), offset).as_str();
                }
                HIR::CALL(func_name, regs, retreg) => {
                    for (idx, reg) in regs.iter().enumerate() {
                        out += format!("  mov {}, {}\n", argr(idx, 8), gr(reg, 8)).as_str();
                    }
                    out += format!("  call {}\n", func_name).as_str();
                    if let Some(reg) = retreg {
                        out += format!("  mov {}, rax\n", gr(reg, 8)).as_str();
                    }
                }
                HIR::PUSHARG(reg) => {
                    out += format!("  push {}\n", argr(*reg, 8)).as_str();
                }
                HIR::INDEXLOAD(reg1, reg2, index, size) => {
                    out += format!(
                        "  mov {}, [{} + {}]\n",
                        gr(reg1, *size),
                        gr(reg2, *size),
                        index * (*size as i128)
                    )
                    .as_str();
                }
            }
        }
        out
    }
    fn division(&self, lr: &usize, rr: &usize) -> String {
        format!("  mov rax, {}\n  cqo\n  idiv {}\n", gr(lr, 8), gr(rr, 8))
    }
    fn compare(&self, lr: &usize, rr: &usize, inst: &str) -> String {
        format!(
            "  cmp {}, {}\n  {} al\n  movzx {}, al\n",
            gr(lr, 8),
            gr(rr, 8),
            inst,
            gr(lr, 8)
        )
    }
}
