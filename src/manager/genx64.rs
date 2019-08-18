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
                HIR::LOAD(reg, val) => println!("  mov {}, {}", gr(reg), val),
                HIR::RETURN(reg) => {
                    println!("  mov rax, {}", gr(reg));
                    println!("  ret");
                }
            }
        }
    }
}
