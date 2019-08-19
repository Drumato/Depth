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
                HIR::LOAD(reg, val) => println!("  mov {}, {}", gr(reg), val),
                HIR::NEGATIVE(reg) => {
                    println!("  neg {}", gr(reg));
                }
                HIR::RETURN(reg) => {
                    println!("  mov rax, {}", gr(reg));
                    println!("  ret");
                }
            }
        }
    }
    fn division(&self, lr: &usize, rr: &usize) {
        println!("  mov rax, {}", gr(lr));
        println!("  cqo");
        println!("  idiv {}", gr(rr));
    }
}
