use std::io::prelude::*;
use std::fs::File;
use std::thread::spawn;

fn scan_opcode(s:&str) {
    match s {
        "mvia" => println!("opcode"),
          _ => println!("Invalid Opcode"),
    }
}

fn length_of(s:&str) -> usize {
    s.len()
}

fn loop_plain(s:&str) {

    for i in s.chars() {
            if i == '\n' {println!("");}
            print!("{:?}",i);
    }
}

fn loop_scanned(s:&str,cnt:&i32) {

    for i in s.chars() {
            print!("{:?}",i);
    }
}

fn main() {

   // let op_mvi_a = 34;
   // let op_mov_a_b = 47;
    //let op_df = 0xdf;
    
    fn process_asm() -> std::io::Result<()> {
    let mut f = try!(File::open("code.asm"));
    let mut s = String::new();
    let mut cnt = 0;
    try!(f.read_to_string(&mut s));
    loop_plain(&s);
    println!("The length of string is  {:?}",length_of(&s));
    Ok(())
    }

    process_asm();
}