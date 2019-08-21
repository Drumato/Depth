use super::super::ir::hi::HIR;
use super::manager::Manager;
const FREE_REG: [&str; 6] = ["r10", "r11", "r12", "r13", "r14", "r15"];
fn gr(num: &usize) -> String {
    FREE_REG[*num].to_string()
}
impl Manager {
    pub fn genx64(&self) {
        for ir in self.hirs.iter() {
            match ir {
                HIR::ADD(lr, rr) => println!("  add {}, {}", gr(lr), gr(rr)),
                HIR::SUB(lr, rr) => println!("  sub {}, {}", gr(lr), gr(rr)),
                HIR::MUL(lr, rr) => {
                    println!("  mov rax, {}", gr(lr));
                    println!("  imul {}", gr(rr));
                    println!("  mov {}, rax", gr(lr));
                }
                HIR::DIV(lr, rr) => {
                    self.division(lr, rr);
                    println!("  mov {}, rax", gr(lr));
                }
                HIR::MOD(lr, rr) => {
                    self.division(lr, rr);
                    println!("  mov {}, rdx", gr(lr));
                }
                HIR::LSHIFT(lr, rr) => {
                    println!("  mov rcx, {}", gr(rr));
                    println!("  sal {}, cl", gr(lr))
                }
                HIR::RSHIFT(lr, rr) => {
                    println!("  mov rcx, {}", gr(rr));
                    println!("  sar {}, cl", gr(lr));
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
                HIR::LOAD(reg, val) => println!("  mov {}, {}", gr(reg), val),
                HIR::NEGATIVE(reg) => {
                    println!("  neg {}", gr(reg));
                }
                HIR::RETURN(reg) => {
                    println!("  mov rax, {}", gr(reg));
                    println!("  call .Lend");
                }
                HIR::PROLOGUE => {
                    println!("  push rbp");
                    println!("  mov rbp, rsp");
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
                    println!("  cmp {}, 0", gr(reg));
                    println!("  je .L{}", label);
                }
                HIR::STORE(offset, reg) => {
                    println!("  mov -{}[rbp], {}", offset, gr(reg));
                }
            }
        }
    }
    fn division(&self, lr: &usize, rr: &usize) {
        println!("  mov rax, {}", gr(lr));
        println!("  cqo");
        println!("  idiv {}", gr(rr));
    }
    fn compare(&self, lr: &usize, rr: &usize, inst: &str) {
        println!("  cmp {}, {}", gr(lr), gr(rr));
        println!("  {} al", inst);
        println!("  movzx {}, al", gr(lr));
    }
}
