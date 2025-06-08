// codegen.rs

use crate::ir::{IRInstr, IRProgram, IRType};
use std::fs::File;
use std::io::{Result, Write};


pub struct Codegen;

impl Codegen {
    pub fn new() -> Self {
        Codegen
    }

    pub fn generate(&self, ir: &IRProgram, output_path: &str) -> Result<()> {
        let mut file = File::create(output_path)?;

        writeln!(file, "section .data")?;
        // Platz für Variablen
        for instr in &ir.instructions {
            match instr {
                IRInstr::Store { name, typ: IRType::Int32, .. } => {
                    writeln!(file, "{}: dd 0", name)?;
                },
                IRInstr::Store { name, typ: IRType::Int64, .. } => {
                    writeln!(file, "{}: dq 0", name)?;
                },
                _ => {}
            };
        }

        writeln!(file, "\nsection .text")?;
        writeln!(file, "global _start")?;
        writeln!(file, "_start:")?;
        writeln!(file, "    call main")?;
        writeln!(file, "    jmp exit")?;

        for instr in &ir.instructions {
            match instr {
                IRInstr::LoadConst { dest, value, typ } => {
                    writeln!(file, "    mov {}, {}", reg(dest, typ), value)?;
                }
                IRInstr::LoadVar { dest, name, typ } => {
                    writeln!(file, "    mov {}, [{}]", reg(dest, typ), name)?;
                }
                IRInstr::Store { name, src, typ } => {
                    writeln!(file, "    mov [{}], {}", name, reg(src, typ))?;
                }
                IRInstr::Add { dest, lhs, rhs, typ } => {
                    writeln!(file, "    mov {}, {}", reg(dest, typ), reg(lhs, typ))?;
                    writeln!(file, "    add {}, {}", reg(dest, typ), reg(rhs, typ))?;
                }
                IRInstr::Sub { dest, lhs, rhs, typ } => {
                    writeln!(file, "    mov {}, {}", reg(dest, typ), reg(lhs, typ))?;
                    writeln!(file, "    sub {}, {}", reg(dest, typ), reg(rhs, typ))?;
                }
                IRInstr::Mul { dest, lhs, rhs, typ } => {
                    writeln!(file, "    mov {}, {}", reg(dest, typ), reg(lhs, typ))?;
                    writeln!(file, "    imul {}, {}", reg(dest, typ), reg(rhs, typ))?;
                }
                IRInstr::Div { dest, lhs, rhs, typ } => {
                    // Achtung: Division nutzt rax & rdx!
                    let r = "rax";
                    let cqo = "cqo";
                    writeln!(file, "    mov {}, {}", reg(r, typ), reg(lhs, typ))?;
                    writeln!(file, "    {}", reg(cqo, typ))?;
                    writeln!(file, "    idiv {}", reg(rhs, typ))?;
                    writeln!(file, "    mov {}, {}", reg(dest, typ), reg(r, typ))?;
                }
                IRInstr::LoadString { .. } => {
                    // vorerst überspringen – Stringhandling = next step
                    // du kannst hier später String-Konstanten verwalten
                }
                IRInstr::FuncBegin { name } => {
                    writeln!(file, "{}:", name)?;
                    writeln!(file, "    push rbp    ;save caller")?;
                    writeln!(file, "    mov rbp, rsp    ; own base_ptr")?;
                }
                IRInstr::FuncEnd => {
                    writeln!(file, "    mov rsp, rbp    ; aufräumen")?;
                    writeln!(file, "    pop rbp")?;
                    writeln!(file, "    ret")?;
                }
            }
        }

        // Exit syscall
        writeln!(file, "exit:")?;
        writeln!(file, "    mov rax, 60")?;
        writeln!(file, "    xor rdi, rdi")?;
        writeln!(file, "    syscall")?;

        Ok(())
    }
}

fn reg<'a>(name: &'a str, typ: &'a IRType) -> &'a str {
    match (name, typ) {
        ("r0", IRType::Int64) => "r8",
        ("r1", IRType::Int64) => "r9",
        ("r2", IRType::Int64) => "r10",
        ("r3", IRType::Int64) => "r11",
        ("r0", IRType::Int32) => "r8d",
        ("r1", IRType::Int32) => "r9d",
        ("r2", IRType::Int32) => "r10d",
        ("r3", IRType::Int32) => "r11d",
        ("r0", IRType::DStr) => "r8",
        ("r1", IRType::DStr) => "r9",
        ("r2", IRType::DStr) => "r10",
        ("r3", IRType::DStr) => "r11",
        // extra for special operations
        ("rax", IRType::Int32) => "eax",
        ("rax", IRType::Int64) => "rax",
        ("cqo", IRType::Int32) => "cdq",
        ("cqo", IRType::Int64) => "cqo",
        //
        ("rdi", IRType::Int32) => "edi",
        ("rdi", IRType::Int64) => "rdi",
        ("rsi", IRType::Int32) => "esi",
        ("rsi", IRType::Int64) => "rsi",
        ("rdx", IRType::Int32) => "edx",
        ("rdx", IRType::Int64) => "rdx",
        ("rcx", IRType::Int32) => "ecx",
        ("rcx", IRType::Int64) => "rcx",
        _ => panic!("No Registers left or unknown type: '{:?}'", name),
    }
}

