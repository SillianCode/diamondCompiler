use crate::parser::{Expr, Program, Stmt, VarDecl};
use crate::parser::Expr::{BinaryOp, Number, Variable, DoubleQuotedString};
use crate::lexer::Token;

fn optimize_expr(expr: &Expr) -> Expr {
    match expr {
        Number { val: _, typ: _ } | Variable(_) | DoubleQuotedString(_) => expr.clone(),

        BinaryOp { left, op, right } => {
            let left = optimize_expr(left);
            let right = optimize_expr(right);

            match (&left, &right, op) {
                (Number { val: l, typ }, Number { val: r, .. }, Token::Plus) => {
                    Number { val: l + r, typ: typ.clone() }
                }
                (Number { val: l, typ }, Number { val: r, .. }, Token::Asterisk) => {
                    Number { val: l * r, typ: typ.clone() }
                }
                (Number { val: l, typ }, Number { val: r, .. }, Token::Minus) => {
                    Number { val: l - r, typ: typ.clone() }
                }
                (Number { val: _, .. }, Number { val: 0, .. }, Token::Slash) => {
                    BinaryOp {
                        left: Box::new(left),
                        op: op.clone(),
                        right: Box::new(right),
                    }
                }
                (Number { val: l, typ }, Number { val: r, .. }, Token::Slash) => {
                    Number { val: l / r, typ: typ.clone() }
                }

                (Number { val: 0, .. }, expr, Token::Plus) => expr.clone(),
                (expr, Number { val: 0, .. }, Token::Plus) => expr.clone(),
                (expr, Number { val: 0, .. }, Token::Minus) => expr.clone(),

                (Number { val: 0, typ }, _, Token::Asterisk) => Number { val: 0, typ: typ.clone() },
                (_, Number { val: 0, typ }, Token::Asterisk) => Number { val: 0, typ: typ.clone() },

                (expr, Number { val: 1, .. }, Token::Asterisk) => expr.clone(),
                (Number { val: 1, .. }, expr, Token::Asterisk) => expr.clone(),
                (expr, Number { val: 1, .. }, Token::Slash) => expr.clone(),

                _ => BinaryOp {
                    left: Box::new(left),
                    op: op.clone(),
                    right: Box::new(right),
                },
            }
        }
    }
}

fn optimize_stmt(stmt: &Stmt) -> Stmt {
    match stmt {
        Stmt::VarDecl(decl) => {
            Stmt::VarDecl(VarDecl {
                name: decl.name.clone(),
                typ: decl.typ.clone(),
                init: optimize_expr(&decl.init),
            })
        }
        Stmt::ExprStmt(expr) => {
            Stmt::ExprStmt(optimize_expr(expr))
        }
        Stmt::FunctionDef { name, params, return_type, body } => {
            let body = body.iter().map(optimize_stmt).collect();
            Stmt::FunctionDef {
                name: name.clone(),
                params: params.clone(),
                return_type: return_type.clone(),
                body,
            }
        }
        Stmt::OutStmt(expr) => {
            Stmt::OutStmt(optimize_expr(expr))
        }
    }
}

pub fn optimize_program(program: &Program) -> Program {
    Program {
        statements: program
            .statements
            .iter()
            .map(optimize_stmt)
            .collect(),
    }
}
