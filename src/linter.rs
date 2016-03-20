pub fn asm_lint(line_tup:(usize,&str)) -> bool {
        // case where line_tup.1 does not contain a comma
    if line_tup.1.starts_with("//") {return false;}
    if !(line_tup.1.contains(",")) {
        if line_tup.1.len() == 5 && !(line_tup.1.contains(" ")) {
            println!("Line:{}- Invalid Opcode",line_tup.0+1);
            false
        }
        else if  line_tup.1.len() > 5 {
            println!("Line:{}- Invalid Opcode",line_tup.0+1);
            false
        }
        else {
            true
        }

    } 
    // case where line_tup.1 contains a comma
     else {
        if line_tup.1.chars().last().unwrap() == ',' {
            println!("Line:{}- Missing operand",line_tup.0+1);
            false
        }
        else if line_tup.1.split(",").nth(1).unwrap().replace(" ","").len() > 5 {
            println!("Line:{}- Invalid operand",line_tup.0+1);
            false
        }
        else {
            true
        }
    }
}