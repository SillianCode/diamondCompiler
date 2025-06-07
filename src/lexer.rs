// Lexer.rs

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(String),
    Identifier(String),
    Keyword(String),
    SingleQuotedString(String),
    DoubleQuotedString(String),
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Colon,
    Comma,
    Bang,
    EOF,
}

const KEYWORDS: &[&str] = &[
    "if", "else", "while", "out", "fn", "int32", "int64", "str", "bool", "float64", "void",
];


pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.peek() {
            if ch == '#' {
                self.skip_comment();
                continue;
            }

            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            return match ch {
                '=' => {
                    self.advance();
                    Token::Equal
                }
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    Token::Asterisk
                }
                '/' => {
                    self.advance();
                    Token::Slash
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                ':' => {
                    self.advance();
                    Token::Colon
                }
                '{' =>{
                    self.advance();
                    Token::LBrace
                }
                '}' => {
                    self.advance();
                    Token::RBrace
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                '!' => {
                    self.advance();
                    Token::Bang
                }
                '\'' => self.read_string('\''),
                '"' => self.read_string('"'),
                c if c.is_ascii_digit() => self.read_number(),
                c if c.is_ascii_alphabetic() || c == '_' => self.read_identifier(),
                _ => {
                    self.advance();
                    continue;
                }
            };
        }
        Token::EOF
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_string(&mut self, quote_char: char) -> Token {
        self.advance(); // Anfangsquote überspringen
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch == quote_char {
                let s: String = self.input[start..self.position].iter().collect();
                self.advance(); // schließende quote überspringen
                return match quote_char {
                    '\'' => Token::SingleQuotedString(s),
                    '"' => Token::DoubleQuotedString(s),
                    _ => unreachable!(),
                };
            }
            self.advance();
        }
        // Falls keine schließende Quote gefunden wird, nehmen wir einfach das, was da ist
        let s: String = self.input[start..self.position].iter().collect();
        match quote_char {
            '\'' => Token::SingleQuotedString(s),
            '"' => Token::DoubleQuotedString(s),
            _ => unreachable!(),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        let number: String = self.input[start..self.position].iter().collect();
        Token::Number(number)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let ident: String = self.input[start..self.position].iter().collect();
        if KEYWORDS.contains(&ident.as_str()) {
            Token::Keyword(ident)
        } else {
            Token::Identifier(ident)
        }
    }

}

pub fn read_file_to_string(filename: &str) -> io::Result<String> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }
    Ok(content)
}

