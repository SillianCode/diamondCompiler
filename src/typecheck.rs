use std::collections::HashMap;
use crate::parser::{Expr, Stmt, VarDecl, Program, Type, Param};

#[derive(Clone, Debug)]
pub struct FunctionType {
    pub param_types: Vec<Type>,
    pub return_type: Type,
}

#[derive(Clone, Debug)]
pub enum SymbolType {
    Var(Type),
    Func(FunctionType),
}

pub struct TypeChecker {
    symbols: HashMap<String, SymbolType>,
    entry: bool,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbols: HashMap::new(),
            entry: false,
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.check_stmt(stmt)?;
        }

        if !self.entry {
            panic!("Keine Funktion 'main' gefunden.");
        }

        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::VarDecl(decl) => self.check_var_decl(decl),
            Stmt::ExprStmt(expr) => {
                self.check_expr(expr)?;
                Ok(())
            }
            Stmt::FunctionDef { name, params, return_type, body } => {
                if name == "main" {
                    self.entry = true;
                }

                if self.symbols.contains_key(name) {
                    return Err(format!("Funktion '{}' wurde bereits definiert", name));
                }

                let func_type = FunctionType {
                    param_types: params.iter().map(|p| p.typ.clone()).collect(),
                    return_type: return_type.clone(),
                };

                self.symbols.insert(name.clone(), SymbolType::Func(func_type.clone()));

                let mut local = TypeChecker {
                    symbols: self.symbols.clone(), // globale + func-symbole
                    entry: self.entry,
                };

                for param in params {
                    local.symbols.insert(param.name.clone(), SymbolType::Var(param.typ.clone()));
                }

                for stmt in body {
                    local.check_stmt(stmt)?;
                }

                Ok(())
            }
            Stmt::OutStmt(expr) => {
                self.check_expr(expr)?;
                Ok(())
            }
        }
    }

    fn check_var_decl(&mut self, decl: &VarDecl) -> Result<(), String> {
        if self.symbols.contains_key(&decl.name) {
            return Err(format!("Variable '{}' wurde schon deklariert", decl.name));
        }

        let expr_type = self.check_expr(&decl.init)?;
        if expr_type != decl.typ {
            return Err(format!(
                "Typfehler: Variable '{}' erwartet Typ '{:?}', aber Initialisierung ist '{:?}'",
                decl.name, decl.typ, expr_type
            ));
        }

        self.symbols.insert(decl.name.clone(), SymbolType::Var(decl.typ.clone()));
        Ok(())
    }

    fn check_expr(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Number { typ, .. } => Ok(typ.clone()),

            Expr::Variable(name) => match self.symbols.get(name) {
                Some(SymbolType::Var(t)) => Ok(t.clone()),
                Some(SymbolType::Func(_)) => Err(format!("'{}' ist eine Funktion, keine Variable", name)),
                None => Err(format!("Unbekannte Variable '{}'", name)),
            },

            Expr::BinaryOp { left, op: _, right } => {
                let left_type = self.check_expr(left)?;
                let right_type = self.check_expr(right)?;

                if left_type != right_type {
                    return Err(format!(
                        "Typfehler bei binärer Operation: linker Typ '{:?}' stimmt nicht mit rechtem Typ '{:?}' überein",
                        left_type, right_type
                    ));
                }

                match left_type {
                    Type::Int32 | Type::Int64 => Ok(left_type),
                    _ => Err(format!("Binäre Operationen nur für int32 oder int64 unterstützt, nicht für '{:?}'", left_type)),
                }
            }

            Expr::DoubleQuotedString(_) => Ok(Type::DStr),

            Expr::FunctionCall { name, args } => {
                if name == "main" {
                    return Err("Funktion 'main' ist nicht aufrufbar. Sie wird automatisch aufgerufen.".into());
                }

                match self.symbols.get(name) {
                    Some(SymbolType::Func(func_type)) => {
                        if args.len() != func_type.param_types.len() {
                            return Err(format!(
                                "Funktionsaufruf '{}' erwartet {} Argumente, aber {} wurden übergeben",
                                name,
                                func_type.param_types.len(),
                                args.len()
                            ));
                        }

                        for (i, (arg, expected_type)) in args.iter().zip(&func_type.param_types).enumerate() {
                            let actual_type = self.check_expr(arg)?;
                            if &actual_type != expected_type {
                                return Err(format!(
                                    "Typfehler im Argument {} von '{}': erwartet '{:?}', gefunden '{:?}'",
                                    i + 1,
                                    name,
                                    expected_type,
                                    actual_type
                                ));
                            }
                        }

                        Ok(func_type.return_type.clone())
                    }
                    Some(SymbolType::Var(_)) => Err(format!("'{}' ist eine Variable, keine Funktion", name)),
                    None => Err(format!("Unbekannte Funktion '{}'", name)),
                }
            }

            _ => Err("Nicht unterstützter Ausdruckstyp im TypeChecker".into()),
        }
    }
}
