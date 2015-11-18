#![feature(custom_derive)]
extern crate regex;

mod asm_gui;
mod opcode;

//use std::io::prelude::*;
use std::thread::spawn;
use std::collections::HashMap;
use std::io::BufWriter;
use std::io::BufReader;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

// trait imports 
use std::io::BufRead;
use std::borrow::ToOwned;

use std::io::Read;
//use gtk::{WindowTrait, ContainerTrait, WidgetTrait, ButtonSignals, BoxTrait};

// external lib imports
use regex::Regex;

use asm_gui::*;
use opcode::*;

const STATUS_SRC_VALID : u8 = 0;
const STATUS_SRC_INVALID : u8 = 1;

struct Cpu {
	a:u8,
	bc:(u8,u8),
	de:(u8,u8),
	hl:(u8,u8),
	sp:u16,
	pc:u16,
	flag:(FL,FL,FL,FL,FL,FL,FL,FL),
}

enum FL {
	T=1,
	F=0,
}
use FL::*;

impl Cpu {
	fn init() -> Self {
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
    fn execute(&mut self,code_line:&str) {
    }
}

enum TYPE {
	BYTE_1,
	BYTE_2,
	BYTE_3,
	INVALID
}

use TYPE::*;

fn hex_code(line:Line) -> TYPE {
	match line.len {
		1 => BYTE_1,
		2 => BYTE_2,
		3 => BYTE_3,
		_ => {INVALID},
	}
}

struct Line {
    len:u8,
}

fn incr_addr(addr:&u16) -> u16 {
	addr+1
}

fn asm_linter(line:&str) -> bool {
	// early return
	if line.len() < 2 {return false;}
	let ins_format:Regex = if line.len() > 5 {
				// matches opcodes with an explicit data
            	Regex::new(r"^\w+\s[ABCDEHLM],\s?((\d{2})|(\d{4}))h$").unwrap()
            } else {
            	// matches opcodes without an explicit data
            	Regex::new(r"^(\w{1,4}\s[A-Z])|(\w{1,4})$").unwrap()
            };
            if ins_format.is_match(line) { true} else { false}
	}

fn opcode_fetch() {
	// just for test
	let opcode_list = vec!["LXI H","MVI A","MVI B","ADD B","HLT"];

	match File::open("src/tests/src.asm") {
		Ok(handle) => {
			let mut addr = 2000u16;
			let mut code_map = HashMap::new();
			let mut status = STATUS_SRC_VALID;
			let mut line_count = 0;
			let mut l:String;
			let mut reader = BufReader::new(handle);
			let mut line = String::new();
            reader.read_line(&mut line);
            line_count+=1;
            while line.trim() != ""{
            	
            	// inner block
            	{	
            		if asm_linter(line.trim()) && opcode_list.contains(&line.trim().split(",").nth(0).unwrap()) {
            			println!("ASM:Linter Status for line {} : Pass",line_count);
            			// Todo
            			l = line.to_owned();
            			let tok:Vec<&str> = l.trim().split(",").collect();
            			if tok.len() < 2 {
            				let code_tup = ((tok[0]),"None");
            				code_map.insert(incr_addr(&addr),code_tup);	
            			} else {
            				let code_tup = ((tok[0]),tok[1]);
            				code_map.insert(incr_addr(&addr),code_tup);
            			}
            			
            		} else {
            			println!("Syntax Error at line {}",line_count);
            			status = STATUS_SRC_INVALID;
            		}
            		line_count+=1;
            	}
            	// clear line buffer and read next line
            	line.clear();
            	reader.read_line(&mut line);
            }
            // checking if source file is syntactically clean
            if status == STATUS_SRC_INVALID {
            	println!("Code contains syntax Errors.");
            	code_map.clear();
            }
            else { // We are good to go
            	let mut cpu = Cpu::init();
            }
		},
		Err(why) => println!("Error opening src {:?}",why),
	}
}

fn _lxi_h(cpu:&mut Cpu) {
	
}

fn _mvi_a(cpu:&mut Cpu) {
	
}

fn _mvi_b(cpu:&mut Cpu) {
	
}

fn _add_b(cpu:&mut Cpu) {
	
}

fn _hlt(cpu:&mut Cpu) {
	
}

fn main() {
	
    opcode_fetch();

}
