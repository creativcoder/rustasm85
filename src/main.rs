use std::io::prelude::*;
use std::fs::File;
use std::thread::spawn;

enum Register {
    BIT_8(u8),
    BIT_16(u16),
    None,
}

use Register::{BIT_8,BIT_16};

enum Opcodes {

}

struct CPU {

    A:Register,
    BC:(Register,Register),
    DE:(Register,Register),
    HL:(Register,Register),
    PC:Register,
    SP:Register,
}

struct Flag {
    S:u8,
    Z:u8,
    AC:u8,
    P:u8,
    C:u8,
}

impl CPU {
    fn new() -> Self {
        CPU {A:BIT_8(0),BC:(BIT_8(0),BIT_8(0)),DE:(BIT_8(0),BIT_8(0)),HL:(BIT_8(0),BIT_8(0)),PC:BIT_16(0),SP:BIT_16(0)}
    }
}

fn main() {
    let cpu = CPU::new();
}