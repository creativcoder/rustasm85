use std::collections::HashMap;

use std::io::BufReader;
use std::io::BufWriter;
use std::io::BufRead;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug)]

enum Register {
    A(u8),
    B(u8),
    C(u8),
    D(u8),
    E(u8),
    H(u8),
    L(u8),
    SP(u8),
    PC(u8),
}


use Register::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
struct CPU {
    A:Register,
    HL_PAIR : (Register,Register),
    BC_PAIR : (Register,Register),
    DE_PAIR : (Register,Register),
    PC : Register,
    SP : Register,
}

impl CPU {
    fn new() -> CPU {
        let cpu = CPU {A:A(0),
                       HL_PAIR:( H(0),L(0) ),
                       BC_PAIR:( B(0),C(0) ),
                       DE_PAIR: ( D(0),E(0) ),
                       PC: PC(0),
                       SP: PC(0), };
    cpu
    }
}       
#[allow(unused_variables)]
#[allow(dead_code)]
fn move_ins(code:&str) {
    // do move instruction
}
#[allow(unused_variables)]
#[allow(dead_code)]
fn arith_ins(code:&str) {
    // do move instruction
}
#[allow(unused_variables)]
#[allow(dead_code)]
fn branch_ins(code:&str) {
    // do move instruction
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
fn main() {
    let mut cpu = CPU::new();
    /*let mut codeline = HashMap::new();
    
    codeline.insert(4000,"MVI A,05");
       let val =  codeline.get(&4000);
       if val.unwrap().split(",").nth(0).unwrap() == "MVI A" {
        cpu.A = A(val.unwrap().split(",").nth(1).unwrap().parse::<u8>().unwrap());
        println!("Register A initialized with {:?}",cpu.A);
       }*/

       match File::open("src.asm") {
           Ok(handle) => {
            let mut reader = BufReader::new(handle);
            let mut line = String::new();
            reader.read_line(&mut line);
            
            while line.trim()!="" {
                // immutable block
                {
                let opcode = line.trim().split(",").nth(0).unwrap();
                
                if opcode == "MVI A" {
                    cpu.A = A(line.trim().split(",").nth(1).unwrap().parse::<u8>().unwrap());
                    println!("Register A initialized to a value");
                } else if opcode == "MVI B" {
                    print!("Before panic");
                    cpu.BC_PAIR.0 = B(line.trim().split(",").nth(1).unwrap().parse::<u8>().unwrap());
                    println!("Add instruction found, dispatching request to move instruction");
                }else if opcode == "ADD B" {
                    print!("Before panic");
                    cpu.A = cpu.A + cpu.B;
                    println!("Add instruction found, dispatching request to move instruction");
                }
                 else if opcode == "HLT" {
                    println!("Program Reset Routine Called");
                }

            } //end mutable block
                //println!("{:?}",line.trim().split(",").collect::<String>());
                line.clear();
                reader.read_line(&mut line);
            }

           },
           Err(msg) => print!("{:?}",msg),
       }
}