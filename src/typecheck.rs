use std::collections::HashMap;
use crate::parser::{Expr, Stmt, VarDecl, Program, Type};

pub struct TypeChecker {
    symbols: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbols: HashMap::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.check_stmt(stmt)?;
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
                if self.symbols.contains_key(name) {
                    return Err(format!("Funktion '{}' wurde bereits definiert", name));
                }

                // Hier: Funktionseintrag (wenn du Signaturen brauchst)
                self.symbols.insert(name.clone(), return_type.clone());

                let mut local = TypeChecker::new();
                for param in params {
                    local.symbols.insert(param.name.clone(), param.typ.clone());
                }

                for stmt in body {
                    local.check_stmt(stmt)?;
                }

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

        self.symbols.insert(decl.name.clone(), decl.typ.clone());
        Ok(())
    }

    fn check_expr(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Number { typ, .. } => Ok(typ.clone()),

            Expr::Variable(name) => {
                self.symbols.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Unbekannte Variable '{}'", name))
            }

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

            // Falls du `FunctionCall` oder `FunctionDef` noch in Expr lässt:
            _ => Err("Nicht unterstützter Ausdruckstyp im TypeChecker".into()),
        }
    }
}
