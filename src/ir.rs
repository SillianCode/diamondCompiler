#[derive(PartialEq, Debug, Clone)]
pub enum IRType {
    Int32,
    Int64,
    DStr,
    SStr
}

#[derive(Debug, Clone)]
pub enum IRInstr {
    LoadConst { dest: String, value: i64, typ: IRType }, 
    LoadString { dest: String, value: String },
    LoadVar   { dest: String, name: String, typ: IRType },
    Add       { dest: String, lhs: String, rhs: String, typ: IRType },
    Mul       { dest: String, lhs: String, rhs: String, typ: IRType },
    Div       { dest: String, lhs: String, rhs: String, typ: IRType },
    Sub       { dest: String, lhs: String, rhs: String, typ: IRType },
    Store     { name: String, src: String, typ: IRType },
    FuncBegin { name: String },
    FuncEnd { name: String },
    FuncCall { name: String, regs: Vec<String> },
    MovReg { dest: String, src: String, typ: IRType },
}

#[derive(Debug)]
pub struct IRProgram {
    pub instructions: Vec<IRInstr>,
}
