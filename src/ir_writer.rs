use std::fs::File;
use std::io::{Result, Write};
use crate::ir::{IRInstr, IRProgram, IRType};

impl std::fmt::Display for IRProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instr in &self.instructions {
            writeln!(f, "{}", instr)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for IRType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRType::Int32 => write!(f, "int32"),
            IRType::Int64 => write!(f, "int64"),
            IRType::DStr => write!(f, "dstring"),
            IRType::SStr => write!(f, "sstring"),
        }
    }
}

impl std::fmt::Display for IRInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRInstr::LoadConst { dest, value, typ } => write!(f, "({}) {} = const {}", typ, dest, value),
            IRInstr::LoadString { dest, value } => write!(f, "{} = string \"{}\"", dest, value),
            IRInstr::LoadVar { dest, name, typ }     => write!(f, "({}) {} = load {}", typ, dest, name),
            IRInstr::Add { dest, lhs, rhs, typ }     => write!(f, "({}) {} = add {}, {}", typ, dest, lhs, rhs),
            IRInstr::Mul { dest, lhs, rhs, typ }     => write!(f, "({}) {} = mul {}, {}", typ, dest, lhs, rhs),
            IRInstr::Div { dest, lhs, rhs, typ }     => write!(f, "({}) {} = div {}, {}", typ, dest, lhs, rhs),
            IRInstr::Sub { dest, lhs, rhs, typ }     => write!(f, "({}) {} = sub {}, {}", typ, dest, lhs, rhs),
            IRInstr::Store { name, src, typ }        => write!(f, "({}) store {}, {}", typ, name, src),
            IRInstr::FuncBegin { name } => write!(f, "FUNC: {}", name),
            IRInstr::Label { name } => write!(f, "{}", name),
            IRInstr::FuncEnd => write!(f, "END_FUNC"),
        }
    }
}

pub fn write_ir_to_file(path: &str, program: &IRProgram) -> Result<()> {
    let path = std::path::Path::new(path);
    
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    write!(file, "{}", program)?;
    Ok(())
}
