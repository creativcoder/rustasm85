#![feature(custom_derive)]
use std::ops::Deref;
use std::ops::Index;
use std::path::Path;

mod reader;
mod opcode;
mod linter;

use reader::read_asm_source;
use opcode::*;

#[derive(Debug)]
struct Cpu {
    a: u8,
    bc: (u8, u8),
    de: (u8, u8),
    hl: (u8, u8),
    sp: u16,
    pc: u16,
    flag: (FL, FL, FL, FL, FL, FL, FL, FL),
}

#[derive(Debug)]
enum FL {
    T = 1,
    F = 0,
}

use FL::*;

impl Cpu {
    fn new() -> Self {
        Cpu {
            a: 0,
            bc: (0, 0),
            de: (0, 0),
            hl: (0, 0),
            sp: 0xff,
            pc: 0,
            flag: (F, F, F, F, F, F, F, F),
        }
    }
    fn mvi_b(&mut self, ins: &Instruction) {
        let operand = ins.operand.clone().trim().replace("h", "").parse::<u8>().unwrap();
        self.bc.0 = operand;
    }
    fn mvi_c(&mut self, ins: &Instruction) {}
    fn add_b(&mut self, ins: &Instruction) {}
    fn hlt(&mut self, ins: &Instruction) {}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Instruction {
    opcode: String,
    operand: String,
    byte_len: usize,
    line_no: usize,
}

impl Instruction {
    fn new(opcode: String, operand: String, byte_len: usize, line_no: usize) -> Self {
        Instruction {
            opcode: opcode,
            operand: operand,
            byte_len: byte_len,
            line_no: line_no,
        }
    }
}

trait Executable {
    fn execute(&mut self, ins: &Instruction);
}

impl Executable for Cpu {
    fn execute(&mut self, ins: &Instruction) {
        match &ins.opcode[..] {
            "mvi b" => self.mvi_b(ins),
            _ => {}
        }
    }
}


fn main() {
    let code = "src/src_files/src.asm";
    let mut ins_vector = Vec::new();
    match read_asm_source(code) {
        Ok(vec) => ins_vector = vec,
        Err(why) => println!("{:?}", why),
    }

    let mut cpu = Cpu::new();
    for ins in ins_vector {
        println!("{:?}", ins);
        cpu.execute(&ins);
    }
}
