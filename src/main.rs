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

#[derive(Debug)]
enum FL {
	T=1,
	F=0,
}
use FL::*;

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

fn opcode_fetch() {

	// just for test
	let opcode_vec = vec!["LXI H","MVI A","MVI B","ADD B","HLT","JMP","CMP","SHLD","CMC"];
	let mut opcode_list:HashMap<String,u8> = HashMap::new();
	opcode_list.insert("LXI H".to_string(),1);
	opcode_list.insert("MVI A".to_string(),2);
	opcode_list.insert("MVI B".to_string(),3);
	opcode_list.insert("ADD B".to_string(),4);
	opcode_list.insert("HLT".to_string(),5);
	let mut code_map:HashMap<String,u8> = HashMap::new();

	match File::open("src/tests/src.asm") {
		Ok(handle) => {
			let mut addr = 2000u16;
			//let mut code_map = HashMap::new();
			let mut status = STATUS_SRC_VALID;
			let mut line_count = 0;
			let mut l:String;
			let mut reader = BufReader::new(handle);
			let mut line = String::new();

            reader.read_line(&mut line);
            line_count+=1;
            while line.trim() != "" {
            	
            	// inner block
            	{	
            		if asm_lint(&line.trim().replace(" ","")) && opcode_vec.contains(&line.trim().split(",").nth(0).unwrap()) {
            			println!("ASM:Linter Status for line {} : Pass",line_count);
            			let current_opcode = line.trim().split(",").nth(0).unwrap().to_owned();
            			code_map.insert(current_opcode.to_owned(), 1);
            			// dispatch current ins_code to controller
            			controller((opcode_list.get(&current_opcode)).unwrap());
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
            	//code_map.clear();
            }
            else { // We are good to go
            	//Compile sources here
            }
		},
		Err(why) => println!("Error opening src {:?}",why),
	}

	// the asm linter routine which does syntax analysis.
	fn asm_lint(line:&str) -> bool {
        // handling cases where line does not contain a comma
    if !(line.contains(",")) {
        if (line.len() == 5 && !(line.contains(" "))){
            println!("Invalid Opcode");
            false
        }
        else if ( line.len() > 5 ) {
            println!("Invalid Opcode");
            false
        }
        else {
            true
        }

    } 
    // handling cases where line contains a comma
     else {
        if(line.chars().last().unwrap() == ',') {
            println!("Missing operand");
            false
        }
        else if (line.split(",").nth(1).unwrap().len() > 5){
            println!("Invalid operand");
            false
        }
        else {
            true
        }
    }
}
}

fn _lxi_h(cpu:&mut Cpu) {
	
}

fn _mvi_a(cpu:&mut Cpu) {
	
}

fn _mvi_b(cpu:&mut Cpu) {
	
}

fn _add_b(cpu:&mut Cpu) {
	// test
	// initilize b with some value
	cpu.bc.0 += 1;
	// add to accumulator
	cpu.a += cpu.bc.0;
	println!("{:?}",cpu );
}

fn _hlt(cpu:&mut Cpu) {
	
}

// controller dispatches the instructions to mutate cpu state
fn controller(ins_code:&u8) {
	let mut cpu = Cpu::init();
	// TODO
	match ins_code {
		&1 => _add_b(&mut cpu),
		&2 => {},
		&_ => {},
	}
}

fn main() {
    opcode_fetch();
}
