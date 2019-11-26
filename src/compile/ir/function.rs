use super::super::super::ce::types::Error;
use super::super::frontend::frontmanager::frontmanager::Symbol;
use super::super::frontend::parse::node::{Func, Node};
use super::super::frontend::sema::semantics::Type;
use super::basicblock::BasicBlock;
use super::constant::Constant;
use super::instruction::CalcMode;
use super::instruction::CompareMode;
use super::instruction::Instruction as Inst;
use super::intrinsic::Intrinsic;
use super::llvm_type::LLVMType;
use super::llvm_value::{LLVMSymbol, LLVMValue};

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write;
pub struct Function {
    pub blocks: Vec<BasicBlock>,
    // ty: LLVMType
    pub args: Vec<LLVMType>,
    pub name: String,
    pub insert_point: usize,
    pub label: usize,
    pub env: BTreeMap<String, LLVMSymbol>,
    pub constants: Vec<Constant>,
    pub const_label: usize,
    pub declares: HashSet<Intrinsic>,
}

impl Function {
    pub fn new(name: String, len: usize) -> Function {
        let entry_block = BasicBlock::new("entry".to_string());
        Self {
            blocks: vec![entry_block],
            name: name,
            insert_point: 0,
            args: Vec::new(),
            label: len,
            env: BTreeMap::new(),
            constants: Vec::new(),
            declares: HashSet::new(),
            const_label: 0,
        }
    }
    pub fn dump(&self) {
        println!(
            "define i64 @{}({}) {}",
            self.name,
            self.format_argtype(),
            "{"
        );
        for bb in self.blocks.iter() {
            bb.dump();
        }
        println!("{}", "}");
    }
    fn format_argtype(&self) -> String {
        let mut base_string = String::new();
        for (i, arg) in self.args.iter().enumerate() {
            if i == self.args.len() - 1 {
                if let Err(err) = base_string.write_fmt(format_args!("{}", arg)) {
                    Error::LLVM.found(&format!("{}", err));
                }
            } else {
                if let Err(err) = base_string.write_fmt(format_args!("{},", arg)) {
                    Error::LLVM.found(&format!("{}", err));
                }
            }
        }
        base_string
    }
    pub fn add_inst(&mut self, inst: Inst) {
        match inst {
            Inst::Store(_, _, _, _) => (),
            Inst::Memcpy64(_, _, _, _) => (),
            _ => self.label += 1,
        }
        self.blocks[self.insert_point].insts.push(inst);
    }
    pub fn build_function(&mut self, f: &Func) {
        for arg in f.args.iter() {
            if let Node::DEFARG(name) = arg {
                if let Some(ref mut s) = f.env.sym_table.get(name) {
                    if let Ok(ty) = s.ty.clone() {
                        let llvm_type = self.get_llvmtype_from_type(&ty);
                        let alignment = llvm_type.alignment();
                        self.args.push(llvm_type.clone());
                        self.add_inst(Inst::Alloca(self.label, llvm_type.clone(), alignment));
                        let llvm_symbol = LLVMSymbol::new(self.label - 1, llvm_type.clone());
                        self.env.insert(name.to_string(), llvm_symbol);
                    }
                }
            }
        }
        for (i, (_key, value)) in self.env.clone().iter().enumerate() {
            let alignment = value.ty.alignment();
            self.add_inst(Inst::Store(
                value.ty.clone(),
                LLVMValue::VREG(i),
                value.label,
                alignment,
            ));
        }
        for st in f.stmts.iter() {
            self.build_stmt(Some(f), st.clone());
        }
    }
    fn build_stmt(&mut self, option_f: Option<&Func>, stmt: Node) {
        if let Some(f) = option_f {
            match stmt {
                Node::RETURN(bexpr) => self.build_return(*bexpr.clone()),
                Node::LET(ident_name, bexpr) => {
                    if let Some(v) = f.env.sym_table.get(&ident_name) {
                        self.build_let(ident_name.to_string(), v, *bexpr.clone())
                    } else {
                        Error::LLVM.found(&format!("{} is not defined", &ident_name));
                    }
                }
                Node::ASSIGN(ident_name, bexpr) => {
                    let label = self.get_symbol_if_defined(&ident_name.to_string()).label;
                    self.build_assign(label, *bexpr.clone())
                }
                Node::CONDLOOP(bcond_expr, bblock) => {
                    self.build_condloop(f, *bcond_expr, *bblock);
                }
                Node::BLOCK(bstmts) => {
                    for bst in bstmts.iter() {
                        self.build_stmt(Some(f), bst.clone());
                    }
                }
                _ => (),
            }
        }
    }
    fn build_condloop(&mut self, f: &Func, cond_node: Node, block: Node) {
        let cond_label = self.label;
        self.add_inst(Inst::UnconditionalBranch(cond_label));

        self.insert_point += 1;
        let insert_point_after_generate_all = self.insert_point;

        let cond_block = BasicBlock::new(format!("{}", cond_label));
        self.blocks.push(cond_block);
        let (cond_value, _) = self.build_expr(cond_node);

        let true_label = self.label;

        let true_block = BasicBlock::new(format!("{}", true_label));
        self.blocks.push(true_block);
        self.insert_point += 1;
        self.label += 1;

        self.build_stmt(Some(f), block);
        let false_label = self.label;
        self.add_inst(Inst::UnconditionalBranch(cond_label));

        self.blocks[insert_point_after_generate_all]
            .insts
            .push(Inst::ConditionalBranch(
                LLVMType::I1,
                cond_value,
                true_label,
                false_label,
            ));
        let breaked_block = BasicBlock::new(format!("{}", false_label));
        self.blocks.push(breaked_block);
        self.insert_point += 1;
    }
    fn build_let(&mut self, ident_name: String, symbol: &Symbol, mut expr: Node) {
        if let Ok(ty) = &symbol.ty {
            let llvm_type = self.get_llvmtype_from_type(ty);
            let alignment = llvm_type.alignment();
            let label = self.label;
            let llvm_symbol = LLVMSymbol::new(label, llvm_type.clone());
            self.env.insert(ident_name, llvm_symbol);
            self.add_inst(Inst::Alloca(label, llvm_type.clone(), alignment));
            if let Node::ARRAYLIT(ref mut elements, ref mut name) = expr {
                *name = format!("{}", self.const_label);
                self.const_label += 1;
                self.add_inst(Inst::BitCast(
                    self.label,
                    llvm_type.clone(),
                    LLVMValue::VREG(label),
                    LLVMType::I8,
                ));
                let total_size = alignment * elements.len();
                self.add_inst(Inst::Memcpy64(
                    LLVMValue::VREG(self.label - 1),
                    LLVMValue::ConstBitCast(
                        llvm_type.clone(),
                        self.name.to_string(),
                        name.to_string(),
                        LLVMType::I8,
                    ),
                    total_size,
                    false,
                ));
                self.declares.insert(Intrinsic::Memcpy);
                self.add_constant_array((*elements).to_vec(), llvm_type, name.to_string());
            } else {
                let (llvm_value, _) = self.build_expr(expr);
                self.add_inst(Inst::Store(llvm_type, llvm_value, label, alignment));
            }
        }
    }
    fn build_assign(&mut self, label: usize, expr: Node) {
        let (llvm_value, llvm_type) = self.build_expr(expr);
        let alignment = llvm_type.alignment();
        self.add_inst(Inst::Store(llvm_type, llvm_value, label, alignment));
    }
    fn build_return(&mut self, expr: Node) {
        let (llvm_value, llvm_type) = self.build_expr(expr);
        self.add_inst(Inst::RetTy(llvm_type, llvm_value));
    }
    fn build_expr(&mut self, expr: Node) -> (LLVMValue, LLVMType) {
        match expr {
            Node::INTEGER(value) => (LLVMValue::INTEGER(value), LLVMType::I64),
            Node::IDENT(name) => {
                let label = self.label;
                let llvm_symbol = self.get_symbol_if_defined(&name);
                let llvm_type = llvm_symbol.ty.clone();
                if let LLVMType::ARRAY(_, _) = &llvm_type {
                    return (LLVMValue::VREG(llvm_symbol.label), llvm_type);
                }
                let alignment = llvm_type.alignment();
                let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                self.add_inst(Inst::Load(label, llvm_type.clone(), llvm_value, alignment));
                (LLVMValue::VREG(label), llvm_type)
            }
            Node::CALL(name, elements) => {
                let mut args: Vec<(LLVMValue, LLVMType)> = Vec::new();
                for elem in elements.iter() {
                    let (elem_value, elem_type) = self.build_expr(elem.clone());
                    args.push((elem_value, elem_type));
                }
                let label = self.label;
                self.add_inst(Inst::Call(label, LLVMType::I64, name, args)); // TODO: func_type
                (LLVMValue::VREG(label), LLVMType::I64)
            }
            Node::INDEX(bary_node, bidx_node) => {
                let (index_value, index_type) = self.build_expr(*bidx_node.clone());
                let (ary_value, ary_type) = self.build_expr(*bary_node);
                let label = self.label;
                self.add_inst(Inst::GetElementPtrInbounds(
                    label,
                    ary_type.clone(),
                    ary_value,
                    index_type,
                    index_value,
                ));
                if let LLVMType::ARRAY(elem_type, _) = ary_type {
                    let alignment = elem_type.alignment();
                    self.add_inst(Inst::Load(
                        label + 1,
                        *elem_type.clone(),
                        LLVMValue::VREG(label),
                        alignment,
                    ));
                    (LLVMValue::VREG(label + 1), *elem_type)
                } else {
                    eprintln!("something wrong in index expression");
                    (LLVMValue::UNKNOWN, LLVMType::UNKNOWN)
                }
            }
            Node::ADDRESS(bchild) => {
                if let Node::IDENT(name) = *bchild {
                    let llvm_symbol = self.get_symbol_if_defined(&name);
                    let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                    let inner_type = Box::new(llvm_symbol.ty.clone());
                    (llvm_value, LLVMType::POINTER(inner_type))
                } else {
                    Error::LLVM.found(&"addressing with constant".to_string());
                    (LLVMValue::UNKNOWN, LLVMType::UNKNOWN)
                }
            }
            Node::DEREFERENCE(bchild) => {
                if let Node::IDENT(name) = *bchild.clone() {
                    let label = self.label;
                    let llvm_symbol = self.get_symbol_if_defined(&name);
                    let llvm_value = LLVMValue::VREG(llvm_symbol.label);
                    let llvm_type = llvm_symbol.ty.clone();
                    let alignment = llvm_type.alignment();
                    self.add_inst(Inst::Load(label, llvm_type.clone(), llvm_value, alignment));
                    if let LLVMType::POINTER(binner) = llvm_type {
                        self.add_inst(Inst::Load(
                            self.label,
                            *binner.clone(),
                            LLVMValue::VREG(label),
                            alignment,
                        ));
                        (LLVMValue::VREG(self.label - 1), *binner)
                    } else {
                        Error::LLVM.found(&"dereference with not pointer".to_string());
                        (LLVMValue::UNKNOWN, LLVMType::UNKNOWN)
                    }
                } else if let Node::DEREFERENCE(bchild_child) = *bchild.clone() {
                    let (child_value, child_type) = self.build_expr(*bchild_child.clone());
                    let alignment = child_type.alignment();
                    let label = self.label;
                    self.add_inst(Inst::Load(
                        label,
                        child_type.clone(),
                        child_value.clone(),
                        alignment,
                    ));
                    if let LLVMType::POINTER(binner) = child_type {
                        (LLVMValue::VREG(label), *binner)
                    } else {
                        Error::LLVM.found(&"dereference with not pointer".to_string());
                        (LLVMValue::UNKNOWN, LLVMType::UNKNOWN)
                    }
                } else {
                    Error::LLVM.found(&"dereference with invalid node".to_string());
                    (LLVMValue::UNKNOWN, LLVMType::UNKNOWN)
                }
            }
            Node::ADD(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Add(label, CalcMode::NSW, lop_type, lop, rop));
                    (LLVMValue::VREG(label), rop_type)
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    (LLVMValue::UNKNOWN, LLVMType::UNKNOWN)
                }
            }
            Node::SUB(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Sub(label, CalcMode::NSW, lop_type, lop, rop));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::MUL(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Mul(label, CalcMode::NSW, lop_type, lop, rop));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::DIV(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Sdiv(label, lop_type, lop, rop));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::MOD(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Srem(label + 1, lop_type, lop, rop));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::EQ(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Icmp(label, CompareMode::EQUAL, lop_type, lop, rop));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::NTEQ(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Icmp(label, CompareMode::NOTEQUAL, lop_type, lop, rop));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::LT(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Icmp(label, CompareMode::LESSTHAN, lop_type, lop, rop));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::GT(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Icmp(
                        label,
                        CompareMode::GREATERTHAN,
                        lop_type,
                        lop,
                        rop,
                    ));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::LTEQ(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Icmp(
                        label,
                        CompareMode::LESSTHANEQUAL,
                        lop_type,
                        lop,
                        rop,
                    ));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::GTEQ(blop, brop) => {
                let (lop, lop_type) = self.build_expr(*blop);
                let (rop, rop_type) = self.build_expr(*brop);
                let label = self.label;
                if lop_type == rop_type {
                    self.add_inst(Inst::Icmp(
                        label,
                        CompareMode::GREATERTHANEQUAL,
                        lop_type,
                        lop,
                        rop,
                    ));
                    return (LLVMValue::VREG(label), rop_type);
                } else {
                    Error::LLVM.found(&format!(
                        "type inference failed between {} and {}",
                        lop, rop
                    ));
                    return (LLVMValue::UNKNOWN, LLVMType::UNKNOWN);
                }
            }
            Node::ARRAYLIT(elements, name) => {
                for (i, elem) in elements.iter().enumerate() {
                    let (_elem_value, elem_type) = self.build_expr(elem.clone());
                    if i == elements.len() - 1 {
                        eprintln!("reached");
                        return (
                            LLVMValue::Const(name),
                            LLVMType::ARRAY(Box::new(elem_type), elements.len()),
                        );
                    }
                }
                (LLVMValue::UNKNOWN, LLVMType::UNKNOWN)
            }
            _ => (LLVMValue::UNKNOWN, LLVMType::UNKNOWN),
        }
    }
    fn get_llvmtype_from_type(&mut self, ty: &Type) -> LLVMType {
        match ty {
            Type::INTEGER => LLVMType::I64,
            Type::POINTER(inner) => {
                let inner_type = self.get_llvmtype_from_type(inner);
                LLVMType::POINTER(Box::new(inner_type))
            }
            Type::ARRAY(elem, length) => {
                let elem_type = self.get_llvmtype_from_type(elem);
                LLVMType::ARRAY(Box::new(elem_type), *length)
            }
            _ => LLVMType::UNKNOWN,
        }
    }
    fn get_symbol_if_defined(&mut self, name: &str) -> &LLVMSymbol {
        if let Some(llvm_symbol) = self.env.get(name) {
            return llvm_symbol;
        } else {
            Error::LLVM.found(&format!("{} is not defined", &name));
            return &LLVMSymbol {
                label: 0,
                ty: LLVMType::UNKNOWN,
            };
        }
    }
    fn add_constant_array(&mut self, elements: Vec<Node>, ty: LLVMType, name: String) {
        let mut values: Vec<(LLVMType, LLVMValue)> = Vec::new();
        for elem in elements.iter() {
            let (elem_value, elem_type) = self.build_expr(elem.clone());
            values.push((elem_type, elem_value));
        }
        let cons = Constant::Array(format!("@__const.{}.{}", self.name, name), ty, values);
        self.constants.push(cons);
    }
}
