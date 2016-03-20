use std::path::Path;
use std::fs::File;
use std::io::Read;
use ::Instructions;
pub struct SourceCode;

use linter::asm_lint;

#[derive(Debug)]
enum AsmError {
	ParseError,
	InstructionNotFound,
}

// this function reads a line of code and returns an Instruction Struct or an AsmError variant, 
// wrapped together as a Result type
fn parse_asm_line(line:&str) -> Result<Instructions,AsmError> {
	let mut byte_len: usize = 0;
	//println!("Processing {:?}",line);
	let opcode = line.split(",").take(1).collect::<String>();
	//println!("{:?}",opcode);
	let operand = line.split(",").skip(1).collect::<String>();
	if operand.len() == 0 {
		byte_len = 1;
		Ok(Instructions::new(opcode,operand,byte_len))
	} else {
		byte_len = 2;
		Ok(Instructions::new(opcode,operand,byte_len))
	}

}


// the only exposed function
pub fn read_asm_source(path:&str) -> Vec<Instructions> {
	let mut ins_vector:Vec<Instructions> = vec![];
	match File::open(path) {
		Ok(ref mut handle) => {
			let mut buff = String::new();
			handle.read_to_string(&mut buff);
			for line in buff.split("\n").enumerate() {
				asm_lint(line);
				let instruction = parse_asm_line(line.1).unwrap();
				ins_vector.push(instruction);
			}
		},
		Err(kind) => {},
	}
	ins_vector
}