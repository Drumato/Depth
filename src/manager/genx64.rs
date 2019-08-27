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
    pub fn genx64(&self) {
        for ir in self.hirs.iter() {
            match ir {
                HIR::ADD(lr, rr) => println!("  add {}, {}", gr(lr, 64), gr(rr, 64)),
                HIR::SUB(lr, rr) => println!("  sub {}, {}", gr(lr, 64), gr(rr, 64)),
                HIR::MUL(lr, rr) => {
                    println!("  mov rax, {}", gr(lr, 8));
                    println!("  imul {}", gr(rr, 8));
                    println!("  mov {}, rax", gr(lr, 8));
                }
                HIR::DIV(lr, rr) => {
                    self.division(lr, rr);
                    println!("  mov {}, rax", gr(lr, 8));
                }
                HIR::MOD(lr, rr) => {
                    self.division(lr, rr);
                    println!("  mov {}, rdx", gr(lr, 8));
                }
                HIR::LSHIFT(lr, rr) => {
                    println!("  mov rcx, {}", gr(rr, 8));
                    println!("  sal {}, cl", gr(lr, 8))
                }
                HIR::RSHIFT(lr, rr) => {
                    println!("  mov rcx, {}", gr(rr, 8));
                    println!("  sar {}, cl", gr(lr, 8));
                }
                HIR::LT(lr, rr) => {
                    self.compare(lr, rr, "setl");
                }
                HIR::LTEQ(lr, rr) => {
                    self.compare(lr, rr, "setle");
                }
                HIR::GT(lr, rr) => {
                    self.compare(lr, rr, "setg");
                }
                HIR::GTEQ(lr, rr) => {
                    self.compare(lr, rr, "setge");
                }
                HIR::EQ(lr, rr) => {
                    self.compare(lr, rr, "sete");
                }
                HIR::NTEQ(lr, rr) => {
                    self.compare(lr, rr, "setne");
                }
                HIR::IMM(reg, val) => println!("  mov {}, {}", gr(reg, 8), val),
                HIR::IMMCHAR(reg, char_val) => {
                    println!("  mov {}, {}", gr(reg, 4), *char_val as u32)
                }
                HIR::NEGATIVE(reg) => {
                    println!("  neg {}", gr(reg, 8));
                }
                HIR::ADDRESS(reg, offset) => {
                    println!("  lea {}, -{}[rbp]", gr(reg, 8), offset);
                }
                HIR::DEREFERENCE(reg, offset) => {
                    println!("  mov {}, -{}[rbp]", gr(reg, 8), offset);
                    println!("  mov {}, [{}]", gr(reg, 8), gr(reg, 8));
                }
                HIR::RETREG(reg) => {
                    println!("  mov rax, {}", gr(reg, 8));
                }
                HIR::RETURN => {
                    println!("  call .Lend");
                }
                HIR::PROLOGUE(size) => {
                    println!("  push rbp");
                    println!("  mov rbp, rsp");
                    if size != &0 {
                        println!("  sub rsp, {}", !7 & size + 7);
                    }
                }
                HIR::EPILOGUE => {
                    println!(".Lend:");
                    println!("  mov rsp, rbp");
                    println!("  pop rbp");
                    println!("  ret");
                }
                HIR::SYMBOL(name) => {
                    println!("{}:", name);
                }
                HIR::JUMP(label) => {
                    println!("  jmp .L{}", label);
                }
                HIR::LABEL(label) => {
                    println!(".L{}:", label);
                }
                HIR::CMP(reg, label) => {
                    println!("  cmp {}, 0", gr(reg, 8));
                    println!("  je .L{}", label);
                }
                HIR::STORE(offset, reg, size) => {
                    println!("  mov -{}[rbp], {}", offset, gr(reg, *size));
                }
                HIR::LOAD(reg, offset, size) => {
                    println!("  mov {}, -{}[rbp]", gr(reg, *size), offset);
                }
                HIR::CALL(func_name, regs) => {
                    for (idx, reg) in regs.iter().enumerate() {
                        println!("  mov {}, {}", argr(idx, 8), gr(reg, 8))
                    }
                    println!("  call {}", func_name);
                }
                HIR::PUSHARG(reg) => println!("  push {}", argr(*reg, 8)),
                HIR::INDEXLOAD(reg1, reg2, index, size) => {
                    println!(
                        "  mov {}, [{} + {}]",
                        gr(reg1, *size),
                        gr(reg2, *size),
                        index * (*size as i128)
                    );
                }
            }
        }
    }
    fn division(&self, lr: &usize, rr: &usize) {
        println!("  mov rax, {}", gr(lr, 8));
        println!("  cqo");
        println!("  idiv {}", gr(rr, 8));
    }
    fn compare(&self, lr: &usize, rr: &usize, inst: &str) {
        println!("  cmp {}, {}", gr(lr, 8), gr(rr, 8));
        println!("  {} al", inst);
        println!("  movzx {}, al", gr(lr, 8));
    }
}
