use std;
use std::fmt;

extern crate regex;
use self::regex::Regex;


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

pub fn tokenize2(in_str : &str) {
	let re = Regex::new(r"(.*)").unwrap();
	let (start,end) = re.find(in_str).expect("No match!");
	let result_str = &in_str[start..end];
	println!("Match found: {}", result_str)
}


// Derp.  Go off of first character.  If it's a paren, it's a paren.
// If it's a string or character, read that, then parse it.
// If it's something starting a number, read number, then parse it.  If not a number, then a symbol.
// Otherwise, it's a symbol.
pub fn tokenize(in_str : &mut std::str::Chars) -> Vec<Token> {
	let mut accm = String::new();
	let mut tokens : Vec<Token> = Vec::new();
	{
		// The scope block here lets the 'inner' closure borrow accm,
		// then give it back at the end of the scope.
		let mut inner = |chr| {
			match chr {
				'(' => Token::LParen,
				'0'...'9' => Token::Number(1),
				_other => Token::RParen,
			}
		};
		let c_maybe = in_str.next();
		let c = c_maybe.expect("Failed getting a char or something from option...");
		tokens.push(inner(c));
	};
	println!("Accm: {}", accm);
	for tok in &tokens {
		println!("Token: {}", tok);
	}
	tokens
}
