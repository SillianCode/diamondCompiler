

// irgen.rs

use crate::parser::{Program, Expr, Stmt, Param};
use crate::lexer::Token;
use crate::ir::{IRInstr, IRProgram, IRType};
use std::io::Write;
use std::collections::HashMap;

pub struct IRGen {
    temp_counter: usize,
    instrs: Vec<IRInstr>,
    free_temps: Vec<String>,
    loaded_vars: HashMap<String, String>,
    var_types: HashMap<String, IRType>,
    func_types: HashMap<String, IRType>,
}

impl IRGen {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            instrs: Vec::new(),
            free_temps: Vec::new(),
            loaded_vars: HashMap::new(),
            var_types: HashMap::new(),
            func_types: HashMap::new(),
        }
    }

    fn fresh_temp(&mut self) -> String {
        if let Some(temp) = self.free_temps.pop() {
            temp
        } else {
            let name = format!("r{}", self.temp_counter);
            self.temp_counter += 1;
            name
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> (String, IRType) {
        use Expr::*;
        match expr {
            Number { val, typ } => {
                let temp = self.fresh_temp();
                let ir_typ = match typ {
                    crate::parser::Type::Int32 => IRType::Int32,
                    crate::parser::Type::Int64 => IRType::Int64,
                    crate::parser::Type::DStr => IRType::DStr,
                    crate::parser::Type::SStr => IRType::SStr,
                };
                self.instrs.push(IRInstr::LoadConst {
                    dest: temp.clone(),
                    value: *val as i64,
                    typ: ir_typ.clone(),
                });
                (temp, ir_typ)
            }

            Variable(name) => {
                if let Some(existing) = self.loaded_vars.get(name) {
                    if let Some(var_typ) = self.var_types.get(name) {
                        return (existing.clone(), var_typ.clone());
                    } else {
                        panic!("Kein Typ für Variable {}", name);
                    }
                }

                let temp = self.fresh_temp();
                let var_typ = self.var_types.get(name).expect("Unbekannte Variable").clone();
                self.instrs.push(IRInstr::LoadVar {
                    dest: temp.clone(),
                    name: name.clone(),
                    typ: var_typ.clone(),
                });
                self.loaded_vars.insert(name.clone(), temp.clone());
                (temp, var_typ)
            }

            BinaryOp { left, op, right } => {
                let (left_reg, left_typ) = self.gen_expr(left);
                let (right_reg, right_typ) = self.gen_expr(right);

                if left_typ != right_typ {
                    panic!("Typfehler in Binäroperation: {:?} vs {:?}", left_typ, right_typ);
                }

                let dest = self.fresh_temp();
                let op_typ = left_typ.clone();

                match op {
                    Token::Plus => self.instrs.push(IRInstr::Add {
                        dest: dest.clone(),
                        lhs: left_reg.clone(),
                        rhs: right_reg.clone(),
                        typ: op_typ.clone(),
                    }),
                    Token::Asterisk => self.instrs.push(IRInstr::Mul {
                        dest: dest.clone(),
                        lhs: left_reg.clone(),
                        rhs: right_reg.clone(),
                        typ: op_typ.clone(),
                    }),
                    Token::Slash => self.instrs.push(IRInstr::Div {
                        dest: dest.clone(),
                        lhs: left_reg.clone(),
                        rhs: right_reg.clone(),
                        typ: op_typ.clone(),
                    }),
                    Token::Minus => self.instrs.push(IRInstr::Sub {
                        dest: dest.clone(),
                        lhs: left_reg.clone(),
                        rhs: right_reg.clone(),
                        typ: op_typ.clone(),
                    }),
                    _ => panic!("Nicht unterstützter Binäroperator: {:?}", op),
                }

                self.release_temp(&left_reg);
                self.release_temp(&right_reg);

                (dest, op_typ)
            }

            DoubleQuotedString(s) => {
                let temp = self.fresh_temp();
                self.instrs.push(IRInstr::LoadString {
                    dest: temp.clone(),
                    value: s.clone(),
                });
                (temp, IRType::DStr)
            }

            FunctionCall { name, args } => {
                // Argumente auswerten und Registernamen sammeln
                let mut arg_regs = Vec::new();

                let mut p_count = 0;
                for arg in args {
                    let (reg, typ) = self.gen_expr(arg);
                    let register = match p_count {
                        0 => "rdi",
                        1 => "rsi",
                        2 => "rdx",
                        3 => "rcx",
                        4 => "r8",
                        5 => "r9",
                        _ => panic!("not implemented: calling more than 6 function params"),
                    };
                    arg_regs.push(register.to_string());
                    self.instrs.push(IRInstr::MovReg {
                        dest: register.to_string(),
                        src: reg.clone(),
                        typ: typ.clone(),
                    });
                    self.release_temp(&reg);
                    p_count += 1;
                }

                // Funktionsaufruf mit Argumenten
                self.instrs.push(IRInstr::FuncCall {
                    name: name.clone(),
                    regs: arg_regs,
                });

                // Rückgabetyp bestimmen
                let ret_type = self.func_types.get(name)
                    .unwrap_or_else(|| panic!("Rückgabetyp der Funktion '{}' unbekannt", name))
                    .clone();

                // Rückgabe aus `rax` holen und in temporären Wert speichern
                let dest = self.fresh_temp();

                self.instrs.push(IRInstr::MovReg {
                    dest: dest.clone(),
                    src: "rax".to_string(),
                    typ: ret_type.clone(),
                });

                (dest, ret_type)
            }

        }
    }

    fn gen_expr_in_rax(&mut self, expr: &Expr) -> IRType {
        use Expr::*;
        match expr {
            Number { val, typ } => {
                let ir_typ = match typ {
                    crate::parser::Type::Int32 => IRType::Int32,
                    crate::parser::Type::Int64 => IRType::Int64,
                    crate::parser::Type::DStr => IRType::DStr,
                    crate::parser::Type::SStr => IRType::SStr,
                };
                self.instrs.push(IRInstr::LoadConst {
                    dest: "rax".to_string(),
                    value: *val as i64,
                    typ: ir_typ.clone(),
                });
                ir_typ
            }

            Variable(name) => {
                let var_typ = self.var_types.get(name).expect("Unbekannte Variable").clone();
                self.instrs.push(IRInstr::LoadVar {
                    dest: "rax".to_string(),
                    name: name.clone(),
                    typ: var_typ.clone(),
                });
                var_typ
            }

            BinaryOp { left, op, right } => {
                // Links in rax laden
                let left_typ = self.gen_expr_in_rax(left);
                let (right_reg, right_typ) = self.gen_expr(right);

                if left_typ != right_typ {
                    panic!("Typfehler in Binäroperation: {:?} vs {:?}", left_typ, right_typ);
                }

                match op {
                    Token::Plus => {
                        self.instrs.push(IRInstr::Add {
                            dest: "rax".to_string(),
                            lhs: "rax".to_string(),
                            rhs: right_reg.clone(),
                            typ: left_typ.clone(),
                        });
                    }
                    Token::Minus => {
                        self.instrs.push(IRInstr::Sub {
                            dest: "rax".to_string(),
                            lhs: "rax".to_string(),
                            rhs: right_reg.clone(),
                            typ: left_typ.clone(),
                        });
                    }
                    Token::Asterisk => {
                        self.instrs.push(IRInstr::Mul {
                            dest: "rax".to_string(),
                            lhs: "rax".to_string(),
                            rhs: right_reg.clone(),
                            typ: left_typ.clone(),
                        });
                    }
                    Token::Slash => {
                        // [!] Div nutzt rax & rdx
                        self.instrs.push(IRInstr::Div {
                            dest: "rax".to_string(),
                            lhs: "rax".to_string(),
                            rhs: right_reg.clone(),
                            typ: left_typ.clone(),
                        });
                    }
                    _ => panic!("Nicht unterstützter Binäroperator: {:?}", op),
                }

                self.release_temp(&right_reg);

                left_typ
            }

            DoubleQuotedString(s) => {
                // FIXME: string ist immer in rax
                self.instrs.push(IRInstr::LoadString {
                    dest: "rax".to_string(),
                    value: s.clone(),
                });
                IRType::DStr
            }

            FunctionCall { name, args } => {
                todo!();
            }

        }
    }

    fn release_temp(&mut self, name: &str) {
        if name.starts_with("r") {
            self.free_temps.push(name.to_string());
        }
    }

    pub fn write_ir_to_file(&self, path: &str, program: &IRProgram) -> std::io::Result<()> {
        let path = std::path::Path::new(path);
    
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::File::create(path)?;
        let out = format!("{:#?}", program);
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl(decl) => {
                let (value_reg, value_type) = self.gen_expr(&decl.init);

                self.instrs.push(IRInstr::Store {
                    name: decl.name.clone(),
                    src: value_reg.clone(),
                    typ: value_type.clone(),
                });

                self.var_types.insert(decl.name.clone(), value_type.clone());
                self.free_temps.push(value_reg);
            }

            Stmt::ExprStmt(expr) => {
                let (temp, _) = self.gen_expr(expr);
                self.release_temp(&temp);
            }

            Stmt::FunctionDef { name, params, return_type, body } => {
                self.instrs.push(IRInstr::FuncBegin {
                    name: name.clone(),
                });
                let ret_type = match return_type {
                    crate::parser::Type::Int32 => IRType::Int32,
                    crate::parser::Type::Int64 => IRType::Int64,
                    crate::parser::Type::DStr  => IRType::DStr,
                    crate::parser::Type::SStr  => IRType::SStr,
                };
                self.func_types.insert(name.clone(), ret_type);


                let mut p_count = 0;
                for Param { name: param_name, typ } in params {
                    let ir_type = match typ {
                        crate::parser::Type::Int32 => IRType::Int32,
                        crate::parser::Type::Int64 => IRType::Int64,
                        crate::parser::Type::DStr  => IRType::DStr,
                        crate::parser::Type::SStr  => IRType::SStr,
                    };


                    let reg = match p_count {
                        0 => "rdi",
                        1 => "rsi",
                        2 => "rdx",
                        3 => "rcx",
                        4 => "r8",
                        5 => "r9",
                        _ => panic!("not implemented: more than 6 function params"),
                    };

                    self.loaded_vars.insert(param_name.clone(), reg.to_string());
                    self.var_types.insert(param_name.clone(), ir_type.clone());

                    p_count += 1;
                }

                for stmt in body {
                    self.gen_stmt(stmt);
                }

                self.instrs.push(IRInstr::FuncEnd { name: name.to_string() });
                self.loaded_vars.clear();
            }
            Stmt::OutStmt(expr) => {
                self.gen_expr_in_rax(expr);
            }
        }
    }

    pub fn ir_gen(&mut self, program: &Program) -> IRProgram {
        for stmt in &program.statements {
            self.gen_stmt(stmt);
        }

        IRProgram {
            instructions: self.instrs.clone(),
        }
    }
}
