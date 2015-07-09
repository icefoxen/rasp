use std;
use std::fmt;

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
			Token::Number(ref i) => 
				formatter.write_str(&format!("Number({})", i)),
			Token::Symbol(ref s) => 
				formatter.write_str(&format!("Symbol({})", s)),
		}
	}
}
pub fn tokenize_num<'a>(chars : &'a mut std::iter::Peekable<std::str::Chars>)  -> i32 {
	let mut accm = String::new();
	//let mut pchars = chars.peekable();
	loop {
		let maybe_chr = chars.peek();
		let chr = match maybe_chr {
			None => break,
			Some(character) => *character,
		};
		match chr {
			' ' => break,
			')' => break,
			other => accm.push(other)
		}
	}

	accm.parse().ok().expect("Number expected")
}

pub fn tokenize_symbol<'a>(chars : &'a mut std::iter::Peekable<std::str::Chars>) -> String {
	let mut accm = String::new();
	//let mut pchars = chars.peekable();
	loop {
		let maybe_chr = chars.peek();
		let chr = match maybe_chr {
			None => break,
			Some(character) => *character,
		};
		match chr {
			' ' => break,
			')' => break,
			other => accm.push(other)
		}
	}
	accm
}

// Derp.  Go off of first character.  If it's a paren, it's a paren.
// If it's a string or character, read that, then parse it.
// If it's something starting a number, read number, then parse it.  
// If not a number, then a symbol.
// Otherwise, it's a symbol.
pub fn tokenize(in_str : &str) -> Vec<Token> {
	let mut chars = in_str.chars().peekable();
	let mut tokens : Vec<Token> = Vec::new();
	// For SOME reason, just using loop here and unpacking
	// the option by hand works, but a for loop doesn't work.
	// So!
	loop {
		let maybe_chr = chars.peek();
		let chr = match maybe_chr {
			Some(c) => *c,
			None => break,
		};
		let token = match chr {
			'(' => { 
				chars.next(); 
				Token::LParen
			},
			')' => { 
				chars.next(); 
				Token::RParen
			},
			'0'...'9' | '+' | '-' | '.' => Token::Number(tokenize_num(&mut chars)),
			_other => Token::Symbol(tokenize_symbol( &mut chars)),
		};
		tokens.push(token);
	}
	for token in &tokens {
		println!("Token: {}", token);
	};
	tokens
}
