pub struct Term {
    pub name: String,
    pub ty: TermType,
    pub val: TermVal,
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
