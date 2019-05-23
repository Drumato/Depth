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
    pub lchild: Box<Node>,
    pub rchild: Box<Node>,
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
    fn new(ty: OperatorType, lchild: Box<Node>, rchild: Box<Node>) -> Operator {
        Operator {
            ty: ty,
            lchild: lchild,
            rchild: rchild,
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
    pub fn find(s: &str) -> Option<OperatorType> {
        match s {
            "+" => Some(OperatorType::PLUS),
            "-" => Some(OperatorType::MINUS),
            "*" => Some(OperatorType::MUL),
            "/" => Some(OperatorType::DIV),
            _ => None,
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
