 pub enum ParseState {
 	Success(u32),
 	Fail,
 }

use ParseState::Success;

pub fn parse_line(v:Vec<String>) -> ParseState {
	
	Success(1)
}

pub fn string(s:&str) -> String {
	s.to_string()
}


fn main() {
	let source:Vec<String> = vec![string("mvi a,05h"),
								  string("mvi b,02h"),
								  string("add b"),
								  string("rst 3")];

}