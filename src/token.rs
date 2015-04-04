use std;
use std::fmt;


pub fn tokenize_num(in_str : &str) -> (i32, i32) {
	(0,0)
}

pub enum Token {
	LParen,
	RParen,
	Number(i32),
	Symbol(String),
}



impl std::fmt::Display for Token {
	fn fmt(&self, formatter : &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Token::LParen => formatter.write_str("LParen"),
			Token::RParen => formatter.write_str("RParen"),
			Token::Number(ref i) => formatter.write_str(&format!("Number({})", i)),
			Token::Symbol(ref s) => formatter.write_str(&format!("Symbol({})", s)),
		}
	}
}

pub fn tokenize(foo : &str) {
	println!("String: {}", foo);
}