// main.rs
mod lexer;
mod parser;
mod typecheck;
mod optimize;
mod irgen;
mod ir;
mod ir_writer;
mod codegen;

use lexer::{Lexer, Token, read_file_to_string};
use parser::Parser;
use typecheck::TypeChecker;
use optimize::optimize_program;
use irgen::IRGen;
use crate::ir_writer::write_ir_to_file;
use codegen::Codegen;


fn main() -> std::io::Result<()> {
    let filename = "tests/file.dmd";
    let input = read_file_to_string(filename)?;
    let mut lexer = Lexer::new(&input);

    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        //println!("{:?}", token.clone());
        tokens.push(token.clone());
        if token == Token::EOF {
            break;
        }
    }

    let mut parser = Parser::new(tokens);

    match parser.parse_program() {
        Ok(program) => {
            // optimize programm
            let opt = optimize_program(&program);
            //println!("Optimized Program: {:#?}", opt);

            //check types and variables
            let mut typechecker = TypeChecker::new();
            match typechecker.check_program(&program) {
                Ok(_) => println!("Typecheck erfolgreich!"),
                Err(e) => println!("Typecheck Fehler: {}", e),
            }

            // IR generieren
            let mut irgen = IRGen::new();
            let ir_program = irgen.ir_gen(&opt);

            // write IR in file
            let outp_file = "out/ir.idm";
            write_ir_to_file(outp_file, &ir_program);

            // Ausgabe zur Kontrolle
            println!("{:#?}", ir_program);

            let codegen = Codegen::new();
            codegen.generate(&ir_program, "out/output.asm").expect("ASM generation failed");
        }

        Err(e) => {
            eprintln!("Parsing Fehler: {}", e);
        }
    }


    Ok(())
}
