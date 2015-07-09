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
pub fn tokenize_num<'a>(start : char, rest : &'a mut std::str::Chars)  -> i32 {
	//let numberString = rest.take_while(|chr| *chr != ' ').collect();
	//let fullString = start.to_string() + numberString;
	0
}

pub fn tokenize_symbol<'a>(start : char, rest : &'a mut std::str::Chars) -> String {
	start.to_string()
}

// Tokenize a string by first unpacking it into a Vec<char>
// and then dealing with each char individually.
// You'd THINK that an iterator would offer a better way
// of doing this, but something screwy 
pub fn tokenize_unpack(in_str : &str) -> Vec<Token> {
	let mut characters : Vec<char> = Vec::new();
	let mut tokens : Vec<Token> = Vec::new();
	for chr in in_str.chars() {
		characters.push(chr);
	}
	for i in 0..characters.len() {
		let x = &characters[0..i];
		for chr in x {
			println!("Foo: {}", chr);
		}
	}


	//println!("Accm: {}", characters);
	for tok in &tokens {
		println!("Token: {}", tok);
	}
	tokens
}

// Derp.  Go off of first character.  If it's a paren, it's a paren.
// If it's a string or character, read that, then parse it.
// If it's something starting a number, read number, then parse it.  
// If not a number, then a symbol.
// Otherwise, it's a symbol.
pub fn tokenize(in_str : &str) -> Vec<Token> {
	let mut chars = in_str.chars();
	let mut accm = String::new();
	let mut tokens : Vec<Token> = Vec::new();
	
	loop {
		let maybeChr = chars.next();
		let chr = match maybeChr {
			Some(c) => c,
			None => break,
		};
		let token = match chr {
			'(' => Token::LParen,
			')' => Token::RParen,
			'0'...'9' | '+' | '-' | '.' => Token::Number(tokenize_num(chr, &mut chars)),
			other => Token::Symbol(tokenize_symbol(chr, &mut chars)),
		};
		tokens.push(token);
	}
	/*
	for chr in &mut chars {
		let token = match chr {
			'(' => Token::LParen,
			')' => Token::RParen,
			'0'...'9' | '+' | '-' | '.' => Token::Number(tokenize_num(chr, &mut chars)),
			other => Token::Symbol(tokenize_symbol(chr, &mut chars)),
		};
		tokens.push(token);
	}
	*/
	/*
	for chr in chars {
		let token = match chr {
			'(' => Token::LParen,
			')' => Token::RParen,
			//'0'...'9' | '+' | '-' | '.' => Token::Number(tokenize_num(chr, &mut chars)),
			//_other => Token::Symbol(tokenize_symbol(chr, &mut chars)),
			'0'...'9' | '+' | '-' | '.' => {
				let res : String = chars.take_while(|x| *x != ' ').collect();
				let num : i32 = res.trim().parse()
					.ok()
					.expect("Expected a number");
				Token::Number(num)
			},
			_other => Token::Symbol("whatever".to_string()),
		};
		tokens.push(token);
	}
	*/
	/*
	tokens
	{
		// The scope block here lets the 'inner' closure borrow accm,
		// then give it back at the end of the scope.
		let mut inner = |chr| {
			match chr {
				'(' => Token::LParen,
				')' => Token::RParen,
				'0'...'9' | '+' | '-' | '.' => Token::Number(1),
				_other => Token::RParen,
			}
		};
		let c_maybe = in_str.next();
		let c = c_maybe.expect("Failed getting a char or something from option...");
		tokens.push(inner(c));
	};
	*/
	for token in &tokens {
		println!("Token: {}", token);
	};
	tokens
}
