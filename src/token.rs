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
	let mut accm = String::new();
	accm.push(start);
	for c in rest {
		match c {
			' ' => break,
			')' => break,
			other => accm.push(other)
		}
	};
	accm.parse().ok().expect("Number expected")
}

pub fn tokenize_symbol<'a>(start : char, rest : &'a mut std::str::Chars) -> String {
	let mut accm = String::new();
	accm.push(start);
	for c in rest {
		match c {
			' ' => break,
			')' => break,
			other => accm.push(other)
		}
	};
	accm
}

fn tokenize_num_(slice : [char]) -> (Token, usize) {
	let n = match slice.position_elem(' ') {
		None => slice.len(),
		Some(i) => i
	}
	let subslice = slice[0..n];
	subslice.parse().ok().expect("Expected a number d00d")
}

fn tokenize_num_(slice : [char]) -> (Token, usize) {
	let n = match slice.position_elem(' ') {
		None => slice.len(),
		Some(i) => i
	}
	let subslice = slice[0..n];
	subslice.parse().ok().expect("Expected a number d00d")
}


// Tokenize a string by first unpacking it into a Vec<char>
// and then dealing with each char individually.
// You'd THINK that an iterator would offer a better way
// of doing this, but something screwy keeps happening
// with the borrow checker.  And you can't peek iterators,
// which is sorta important.
pub fn tokenize_unpack(in_str : &str) -> Vec<Token> {
    let mut characters : Vec<char> = Vec::new();
    let mut tokens : Vec<Token> = Vec::new();
    for chr in in_str.chars() {
        characters.push(chr);
    }
    let mut start_position = 0;
    let mut end_position = 0;
    loop {
    	if endPosition > characters.len() {
    		break;
    	}

    	let current_char = characters[start_position];
    	match current_char {
			'(' => tokens.push(Token::LParen),
			')' => tokens.push(Token::RParen),
			'0'...'9' | '+' | '-' | '.' => 
				let number, next = tokenize_num_([start_position..]);
				start_position <- next;
				Token::Number(number),
			_other => 
				tokens.push(Token::Symbol(tokenize_symbol(chr, &mut chars))),

    	}
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
	let mut tokens : Vec<Token> = Vec::new();
	
	// For SOME reason, just using loop here and unpacking
	// the option by hand works, but a for loop doesn't work.
	// So!
	loop {
		let maybe_chr = chars.next();
		let chr = match maybe_chr {
			Some(c) => c,
			None => break,
		};
		let token = match chr {
			'(' => Token::LParen,
			')' => Token::RParen,
			'0'...'9' | '+' | '-' | '.' => Token::Number(tokenize_num(chr, &mut chars)),
			_other => Token::Symbol(tokenize_symbol(chr, &mut chars)),
		};
		tokens.push(token);
	}
	for token in &tokens {
		println!("Token: {}", token);
	};
	tokens
}
