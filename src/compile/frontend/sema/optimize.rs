use super::super::frontmanager::frontmanager::FrontManager;
use super::super::parse::node::{Func, Node};
impl FrontManager {
    pub fn constant_folding(&mut self) {
        let func_num: usize = self.functions.len();
        let mut idx: usize = 0;
        loop {
            if idx == func_num {
                break;
            }
            let f: Func = self.functions[idx].clone();
            self.cur_env = f.env.clone();
            for (i, n) in f.stmts.iter().enumerate() {
                self.fold_stmt(n, idx, i);
            }
            self.functions[idx].env = self.cur_env.clone();
            idx += 1;
        }
    }
    fn fold_stmt(&mut self, n: &Node, func_idx: usize, i: usize) {
        match n {
            Node::LET(ident_name, bexpr) => {
                let onode: Option<Node> = self.fold_expr(*bexpr.clone());
                if let Some(Node::INTEGER(val)) = onode {
                    self.functions[func_idx].stmts[i] =
                        Node::LET(ident_name.clone(), Box::new(Node::INTEGER(val)));
                }
            }
            Node::ASSIGN(ident, bexpr) => {
                let onode: Option<Node> = self.fold_expr(*bexpr.clone());
                if let Some(Node::INTEGER(val)) = onode {
                    self.functions[func_idx].stmts[i] =
                        Node::ASSIGN(ident.clone(), Box::new(Node::INTEGER(val)));
                }
            }
            Node::RETURN(bexpr) => {
                let onode: Option<Node> = self.fold_expr(*bexpr.clone());
                if let Some(Node::INTEGER(val)) = onode {
                    self.functions[func_idx].stmts[i] = Node::RETURN(Box::new(Node::INTEGER(val)));
                }
            }
            _ => (),
        }
    }
    fn fold_expr(&mut self, n: Node) -> Option<Node> {
        match n {
            Node::INTEGER(val) => Some(Node::INTEGER(val)),
            Node::ADD(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER(lval + rval));
                }
                None
            }
            Node::SUB(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER(lval - rval));
                }
                None
            }
            Node::MUL(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER(lval * rval));
                }
                None
            }
            Node::DIV(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER(lval / rval));
                }
                None
            }
            Node::MOD(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER(lval % rval));
                }
                None
            }
            Node::EQ(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER((lval == rval) as i128));
                }
                None
            }
            Node::NTEQ(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER((lval != rval) as i128));
                }
                None
            }
            Node::LT(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER((lval < rval) as i128));
                }
                None
            }
            Node::GT(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER((lval > rval) as i128));
                }
                None
            }
            Node::LTEQ(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER((lval <= rval) as i128));
                }
                None
            }
            Node::GTEQ(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER((lval >= rval) as i128));
                }
                None
            }
            Node::LSHIFT(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER((lval << rval) as i128));
                }
                None
            }
            Node::RSHIFT(lch, rch) => {
                if let Some((lval, rval)) = self.check_valid(*lch, *rch) {
                    return Some(Node::INTEGER(lval >> rval));
                }
                None
            }
            Node::MINUS(lch) => {
                if let Some(Node::INTEGER(lval)) = self.fold_expr(*lch) {
                    return Some(Node::INTEGER(-lval));
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }
    fn check_valid(&mut self, lhs: Node, rhs: Node) -> Option<(i128, i128)> {
        if let Some(Node::INTEGER(lval)) = self.fold_expr(lhs) {
            if let Some(Node::INTEGER(rval)) = self.fold_expr(rhs) {
                return Some((lval, rval));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
