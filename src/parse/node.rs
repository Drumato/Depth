pub trait Node {
    fn dump(&self) -> String;
    fn ty(&self) -> NodeType;
}

pub enum NodeType {
    Term,
    Operator,
}

pub struct Term {
    pub name: String,
    pub ty: TermType,
    pub val: TermVal,
}

impl Node for Term {
    fn dump(&self) -> String {
        self.string()
    }
    fn ty(&self) -> NodeType {
        NodeType::Term
    }
}

impl Term {
    fn new(param: (String, TermType, TermVal)) -> Term {
        Term {
            name: param.0,
            ty: param.1,
            val: param.2,
        }
    }
    fn string(&self) -> String {
        format!(
            "type:{}\tname:{}\tval:{}",
            self.ty.string(),
            self.name,
            self.val.string()
        )
    }
}

pub struct Operator {
    pub ty: OperatorType,
    pub lchild: Node,
    pub rchild: Node,
}

impl Node for Operator {
    fn dump(&self) -> String {
        format! {
            "Type:{}\tlchild:{}\trchild:{}",
            self.ty.string(),
            self.lchild.dump(),
            self.rchild.dump(),
        }
    }
    fn ty(&self) -> NodeType {
        NodeType::Operator
    }
}

impl Operator {
    fn new(param: (OperatorType, Node, Node)) -> Operator {
        Operator {
            ty: param.0,
            lchild: param.1,
            rchild: param.2,
        }
    }
}

pub enum OperatorType {
    PLUS,
    MINUS,
    MUL,
    DIV,
}

impl OperatorType {
    pub fn string(&self) -> String {
        match self {
            OperatorType::PLUS => String::from("NADD"),
            OperatorType::MINUS => String::from("NSUB"),
            OperatorType::MUL => String::from("NMUL"),
            OperatorType::DIV => String::from("NDIV"),
        }
    }
    pub fn find(s: String) -> OperatorType {
        match s {
            String::from("+") => OperatorType::PLUS,
            String::from("-") => OperatorType::MINUS,
            String::from("*") => OperatorType::MUL,
            String::from("/") => OperatorType::DIV,
        }
    }
}
pub enum TermVal {
    IntVal(i64),
    RealVal(f64),
    CharVal(char), //change u32 after
    StrVal(String),
    InVal,
}
impl TermVal {
    pub fn string(&self) -> String {
        match self {
            TermVal::IntVal(d) => format!("{}", d),
            TermVal::RealVal(r) => format!("{}", r),
            TermVal::CharVal(c) => format!("{}", c),
            TermVal::StrVal(s) => format!("{}", s),
            _ => "".to_string(),
        }
    }
}

pub enum TermType {
    INT,
    CH,
    ID,
}

impl TermType {
    pub fn string(&self) -> String {
        match self {
            TermType::INT => String::from("NINT"),
            TermType::CH => String::from("NCHAR"),
            TermType::ID => String::from("NID"),
        }
    }
}
