use std::ops::Deref;
use std::ops::Index;
use std::path::Path;

mod reader;
mod opcode;
mod linter;


use reader::read_asm_source;
use opcode::*;


#[derive(Copy, Clone)]
struct Instruction;

#[derive(Debug)]
struct Cpu {
	a:u8,
	bc:(u8,u8),
	de:(u8,u8),
	hl:(u8,u8),
	sp:u16,
	pc:u16,
	flag:(FL,FL,FL,FL,FL,FL,FL,FL),
}

#[derive(Debug)]
enum FL {
	T=1,
	F=0,
}

use FL::*;

impl Cpu {
	fn new() -> Self {
		Cpu {
			a:0,
			bc:(0,0),
			de:(0,0),
			hl:(0,0),
			sp:0xff,
			pc:0,
			flag:(F,F,F,F,F,F,F,F),
		}
	}
	fn aci(&mut self) {

	}
	fn adc_a(&mut self) {

	}
	fn adc_b(&mut self) {

	}
}

pub struct Instructions {
	opcode:String,
	operand:String,
	byte_len:usize,
}

impl Instructions {
    fn new(opcode:String,operand:String,byte_len:usize) -> Self {
    	Instructions { opcode:opcode,operand:operand,byte_len:byte_len }
    }
    fn mvi_b(&self,cpu:&mut Cpu) {

    }
    fn mvi_c(&self,cpu:&mut Cpu) {

    }
    fn execute(&self,cpu:&mut Cpu) {
    	match &self.opcode[..] {
    		"mvi b" => self.mvi_b(cpu),
    		"mvi c" => self.mvi_c(cpu), 
    		_ => {},
    	}
    }
    
}

impl Index<usize> for Instructions {
	type Output = Instructions;
	fn index(&self,index:usize) -> &Instructions {
		let mut cpu = Cpu::new();
		match index {
			0 => cpu.aci(),
			1 => cpu.adc_a(),
			2 => cpu.adc_a(),
			_ => cpu.adc_a(),
		}
		self
	}
}
fn main() {
    let code = "./file.asm";
    let ins_vector = read_asm_source(code);
    let mut cpu = Cpu::new();
    for ins in ins_vector {
    	ins.execute(&mut cpu);
    }
}
