// parser.rs

use crate::lexer::Token;

// NODES //////////////////////////////////
///////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number {
        val: i32,
        typ: Type,
    },
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    DoubleQuotedString(String),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(VarDecl),
    ExprStmt(Expr),
    FunctionDef {
        name: String,
        params: Vec<Param>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    OutStmt(Expr),
}


#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Int32,
    Int64,
    DStr, 
    SStr,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub typ: Type,
    pub init: Expr,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}


/////////////////////////////////////////////////

pub struct Parser {
    input: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Parser { input, position: 0 }
    }

    pub fn current_token(&self) -> Option<&Token> {
        self.input.get(self.position)
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn expect(&mut self, expected: &Token) -> bool {
        if let Some(token) = self.current_token() {
            if token == expected {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while let Some(token) = self.current_token() {
            if token == &Token::EOF {
                break;
            }

            match token {
                Token::Keyword(k) if k == "fn" => {
                    let func = self.parse_function_def()?;
                    statements.push(func);
                }
                _ => {
                    panic!("no global code other than functions allowed!");
                    //let decl = self.parse_var_decl()?;
                    //statements.push(Stmt::VarDecl(decl));
                }
            }
        }

        Ok(Program { statements })
    }



    fn parse_var_decl(&mut self) -> Result<VarDecl, String> {
        let name = match self.current_token().cloned() {
            Some(Token::Identifier(n)) => {
                self.advance();
                n
            }
            _ => return Err("Erwartet Identifier".into()),
        };

        if !self.expect(&Token::Colon) {
            return Err("Erwartet ':'".into());
        }

        let typ_str = match self.current_token().cloned() {
            Some(Token::Keyword(t)) => {
                self.advance();
                t
            }
            _ => return Err("Erwartet Typ als Identifier oder Keyword".into()),
        };

        let typ = match typ_str.as_str() {
            "int32" => Type::Int32,
            "int64" => Type::Int64,
            "str" => Type::DStr,
            _ => return Err(format!("Unbekannter Typ: {}", typ_str)),
        };

        if !self.expect(&Token::Equal) {
            return Err("Erwartet '='".into());
        }

        let init = self.parse_expression(Some(typ.clone()))?;

        if !self.expect(&Token::Semicolon) {
            return Err("Erwartet ';'".into());
        }

        Ok(VarDecl {
            name,
            typ,
            init,
        })
    }

    fn parse_expression(&mut self, expected_type: Option<Type>) -> Result<Expr, String> {
        self.parse_expression_precedence(0, expected_type)
    }

    fn parse_expression_precedence(
        &mut self,
        min_prec: u8,
        expected_type: Option<Type>,
    ) -> Result<Expr, String> {
        let mut left = match self.current_token().cloned() {
            Some(Token::Number(n)) => {
                self.advance();
                let val = n.parse::<i32>().map_err(|_| "Ungültige Zahl")?;
                Expr::Number {
                    val,
                    typ: expected_type.clone().unwrap_or(Type::Int32),
                }
            }

            Some(Token::Identifier(name)) => {
                self.advance();
                Expr::Variable(name)
            }

            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression(expected_type.clone())?;
                if !self.expect(&Token::RParen) {
                    return Err("Erwartet ')'".into());
                }
                expr
            }

            Some(Token::DoubleQuotedString(s)) => {
                self.advance();
                Expr::DoubleQuotedString(s)
            }

            Some(Token::Bang) => {
                // FnCall
                self.advance();
                let fnCall = self.parse_function_call()?;
                fnCall
            }

            _ => {
                return Err("Erwartet Zahl, Variable oder '('".into());
            }
        };

        loop {
            let (prec, op_token) = match self.current_token() {
                Some(Token::Plus) => (1, Token::Plus),
                Some(Token::Minus) => (1, Token::Minus),
                Some(Token::Asterisk) => (2, Token::Asterisk),
                Some(Token::Slash) => (2, Token::Slash),
                _ => break,
            };

            if prec < min_prec {
                break;
            }

            self.advance();

            let right = self.parse_expression_precedence(prec + 1, expected_type.clone())?;

            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op_token,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_function_def(&mut self) -> Result<Stmt, String> {
        self.advance(); // fn

        let name = match self.current_token().cloned() {
            Some(Token::Identifier(n)) => {
                self.advance();
                n
            }
            _ => return Err("Erwartet Funktionsnamen".into()),
        };

        if !self.expect(&Token::Colon) {
            return Err("Erwartet ':'".into());
        }

        let return_type = match self.current_token().cloned() {
            Some(Token::Keyword(t)) => {
                self.advance();
                match t.as_str() {
                    "int32" => Type::Int32,
                    "int64" => Type::Int64,
                    "str" => Type::DStr,
                    _ => return Err(format!("Unbekannter Rückgabetyp: {}", t)),
                }
            }
            _ => return Err("Erwartet Rückgabetyp".into()),
        };

        if !self.expect(&Token::Equal) {
            return Err("Erwartet '='".into());
        }

        if !self.expect(&Token::LParen) {
            return Err("Erwartet '(' für Parameterliste".into());
        }

        let mut params = Vec::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::Identifier(_) => {
                    let param_name = if let Some(Token::Identifier(n)) = self.current_token().cloned() {
                        self.advance();
                        n
                    } else {
                        return Err("Erwartet Parametername".into());
                    };

                    if !self.expect(&Token::Colon) {
                        return Err("Erwartet ':' nach Parameternamen".into());
                    }

                    let param_type_str = match self.current_token().cloned() {
                        Some(Token::Keyword(t)) => {
                            self.advance();
                            t
                        }
                        _ => return Err("Erwartet Parametertyp".into()),
                    };

                    let param_type = match param_type_str.as_str() {
                        "int32" => Type::Int32,
                        "int64" => Type::Int64,
                        "str" => Type::DStr,
                        _ => return Err(format!("Unbekannter Parametertyp: {}", param_type_str)),
                    };

                    params.push(Param {
                        name: param_name,
                        typ: param_type,
                    });

                    if self.expect(&Token::Comma) {
                        continue;
                    } else if self.expect(&Token::RParen) {
                        break;
                    } else {
                        return Err("Erwartet ',' oder ')'".into());
                    }
                }
                Token::RParen => {
                    self.advance();
                    break;
                }
                _ => return Err("Erwartet Parameter oder ')'".into()),
            }
        }

        if !self.expect(&Token::LBrace) {
            return Err("Erwartet '{' für Funktionskörper".into());
        }

        let mut body = Vec::new();
        while let Some(token) = self.current_token() {
            if *token == Token::RBrace {
                self.advance();
                break;
            }

            match token {
                Token::Identifier(_) => {
                    let decl = self.parse_var_decl()?;
                    body.push(Stmt::VarDecl(decl));
                }
                Token::Keyword(k) => match k.as_str() {
                    "fn" => {
                        let func = self.parse_function_def()?;
                        body.push(func);
                    }
                    "out" => {
                        self.advance();
                        let expr = self.parse_expression(None)?;
                        body.push(Stmt::OutStmt(expr));

                        if !self.expect(&Token::Semicolon) {
                            return Err("Erwartet ';'".into());
                        }
                    }
                    _ => { 
                        panic!("unexpected keyword '{}'", k);
                    }
                }
                _ => {
                    let expr = self.parse_expression(None)?;
                    body.push(Stmt::ExprStmt(expr));
                }
            }
        }

        Ok(Stmt::FunctionDef {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_function_call(&mut self) -> Result<Expr, String> {
        // ident(expr, ...?)
        let name = match self.current_token().cloned() {
            Some(Token::Identifier(n)) => {
                self.advance();
                n
            }
            _ => return Err("Erwartet Funktionsnamen".into()),
        };

        if !self.expect(&Token::LParen) {
            return Err("Erwartet '(' nach Funktionsnamen".into());
        }

        let mut args = Vec::new();

        // no params => ()
        if let Some(token) = self.current_token().cloned() {
            if token == Token::RParen {
                self.advance();
                return Ok(Expr::FunctionCall { name, args });
            }
        }

        // mind. 1 arg
        loop {
            let expr = self.parse_expression(None)?;
            args.push(expr);

            match self.current_token().cloned() {
                Some(Token::Comma) => {
                    self.advance();
                }
                Some(Token::RParen) => {
                    self.advance();
                    break;
                }
                _ => {
                    return Err("Erwartet ',' oder ')' in Funktionsaufruf".into());
                }
            }
        }

        Ok(Expr::FunctionCall { name, args })
    }



}
