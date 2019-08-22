use super::super::manager::semantics::Type;
use super::super::token::token::Token;
#[derive(Clone)]
pub enum Node {
    BINOP(Token, Box<Node>, Box<Node>, Option<Type>),
    UNARY(Token, Box<Node>, Option<Type>),
    NUMBER(Type),
    CHARLIT(char),
    IDENT(String),
    RETURN(Box<Node>),
    IF(Box<Node>, Box<Node>, Option<Box<Node>>), // condition, block,else
    BLOCK(Vec<Box<Node>>),
    LET(String, Token, Box<Node>), // ident_name,type,expression
    INVALID,
}
impl Node {
    pub fn string(&self) -> String {
        match self {
            Node::BINOP(op, lhs, rhs, _) => {
                format!("{}({}, {})", op.string(), lhs.string(), rhs.string())
            }
            Node::UNARY(op, inner, _) => format!("{}({})", op.string(), inner.string()),
            Node::NUMBER(ty) => match ty {
                Type::INTEGER(int_type) => format!("INT-Node<{}>", int_type.val.unwrap()),
                _ => format!("UNKNOWN"),
            },
            Node::RETURN(expr) => format!("RETURN({})", expr.string()),
            Node::IF(cond, stmt, alter) => match alter {
                Some(else_block) => format!(
                    "IF({}) ({}) ELSE({})",
                    cond.string(),
                    stmt.string(),
                    else_block.string(),
                ),
                None => format!("IF({}) ({}) ", cond.string(), stmt.string()),
            },
            Node::BLOCK(bstmts) => {
                let stmts: String = bstmts.into_iter().map(|st| st.string()).collect::<String>();
                format!("BLOCK({})", stmts)
            }
            Node::INVALID => "INVALID".to_string(),
            Node::LET(ident_name, ty, expr) => {
                format!("LET {} <- {} ({})", ident_name, ty.string(), expr.string())
            }
            Node::CHARLIT(char_val) => format!("CHARLIT<{}>", char_val),
            Node::IDENT(ident_name) => format!("IDENT<{}>", ident_name),
        }
    }
}

#[derive(Clone)]
pub struct Func {
    pub name: String,
    pub stmts: Vec<Node>,
}
