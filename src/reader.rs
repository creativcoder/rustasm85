use std::path::Path;
use std::fs::File;
use std::io::Read;
use Instruction;

use linter::asm_lint;

#[derive(Debug)]
pub enum AsmError {
    ParseError,
    InstructionNotFound,
    ReadError,
}

// this function reads a line of code and returns an Instruction Struct or an AsmError variant,
// wrapped together as a Result type
fn parse_asm_line(ins_tup: (usize, &str)) -> Result<Instruction, AsmError> {
    let (line_no, line) = ins_tup;
    let mut byte_len: usize = 0;
    let opcode = line.split(",").take(1).collect::<String>();
    let operand = line.split(",").skip(1).collect::<String>();
    if operand.len() == 0 {
        byte_len = 1;
        Ok(Instruction::new(opcode, operand, byte_len, line_no))
    } else {
        byte_len = 2;
        Ok(Instruction::new(opcode, operand, byte_len, line_no))
    }

}

pub fn read_asm_source(path: &str) -> Result<Vec<Instruction>, AsmError> {
    let mut ins_vector: Vec<Instruction> = vec![];
    match File::open(path) {
        Ok(ref mut handle) => {
            let mut buff = String::new();
            handle.read_to_string(&mut buff);
            for line in buff.split("\n").enumerate() {
                asm_lint(line);
                let instruction = parse_asm_line(line).unwrap();
                ins_vector.push(instruction);
            }
            Ok(ins_vector)
        }
        Err(kind) => Err(AsmError::ReadError),
    }

}
