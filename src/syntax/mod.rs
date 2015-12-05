pub fn asm_lint(line:&str) -> bool {
        // case where line does not contain a comma
    if !(line.contains(",")) {
        if line.len() == 5 && !(line.contains(" ")) {
            println!("Invalid Opcode");
            false
        }
        else if  line.len() > 5 {
            println!("Invalid Opcode");
            false
        }
        else {
            true
        }

    } 
    // case where line contains a comma
     else {
        if line.chars().last().unwrap() == ',' {
            println!("Missing operand");
            false
        }
        else if line.split(",").nth(1).unwrap().replace(" ","").len() > 5 {
            println!("Invalid operand");
            false
        }
        else {
            true
        }
    }
}