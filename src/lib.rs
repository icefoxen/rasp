mod token;

use std::fmt::*;
use std::collections::HashMap;

pub type SymbolId = i32;


pub enum Val {
	Int(i32),
	Nil,
	Cons(Box<Val>, Box<Val>),
	Symbol(String),
}

pub struct VmContext<'a> {
	// I THINK the lifetime specifier here says that the &str 
	// lasts as long as the VmContext does...
	symbols : std::collections::HashMap<SymbolId, &'a str>,
	next_symbol : SymbolId,
}


impl <'a> VmContext<'a> {
	fn next_symbol(&mut self) -> SymbolId {
		let sym = self.next_symbol;
		self.next_symbol += 1;
		sym
	}

	pub fn new() -> VmContext<'a> {
		VmContext {
			next_symbol : 0,
			symbols : HashMap::new(),
		}
	}
}

fn start_write_cons(car : &Val, cdr : &Val, formatter : &mut std::fmt::Formatter) -> Result {
	formatter.write_str(&format!("({}", car))
			 .and(finish_write_cons(cdr, formatter))
}

fn finish_write_cons(val : &Val, formatter : &mut std::fmt::Formatter) -> Result {
	match *val {
		// If we get another cons cell, we write the first item, then recurse
		Val::Cons(ref car, ref cdr) => formatter.write_str(&format!(" {}", car)).and(finish_write_cons(cdr, formatter)),
		// If we get nil, we end the list
		Val::Nil => formatter.write_str(")"),
		// If we get something else, we're finishing a dotted pair.
		ref someval => formatter.write_str(&format!(" . {})", someval)),
	}
}

impl std::fmt::Display for Val {
	fn fmt(&self, formatter : &mut std::fmt::Formatter) -> Result {
		match *self {
			Val::Int(ref i)  => formatter.write_str(&format!("{}", i)),
			Val::Nil => formatter.write_str("Nil"),
			Val::Cons(ref car, ref cdr) => start_write_cons(car, cdr, formatter),
			Val::Symbol(ref name) => formatter.write_str(name)
		}
	}
}

pub fn cons(car : Val, cdr : Val) -> Val {
	let ca = Box::new(car);
	let cd = Box::new(cdr);
	Val::Cons(ca, cd)
}

/// Takes a string and interns it as a symbol.
/// Returns the symbol's ID#
pub fn intern_symbol<'a>(ctx : &mut VmContext<'a>, name : & 'a str) -> SymbolId {
	let id = ctx.next_symbol();
	ctx.symbols.insert(id, name);
	id
}


pub fn read<'a>(in_str : &str) -> Val {
	let mut chars = in_str.chars();
	token::tokenize("Some string");
	tokenize(&mut chars);
	let c = chars.next();
	match c {
		Some('0') => Val::Int(0),
		Some(_stuff) => Val::Nil,
		None => Val::Nil
	}
}


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
	fn fmt(&self, formatter : &mut std::fmt::Formatter) -> Result {
		match *self {
			Token::LParen => formatter.write_str("LParen"),
			Token::RParen => formatter.write_str("RParen"),
			Token::Number(ref i) => formatter.write_str(&format!("Number({})", i)),
			Token::Symbol(ref s) => formatter.write_str(&format!("Symbol({})", s)),
		}
	}
}

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
		inner(c);
	};
	println!("Accm: {}", accm);
	for tok in &tokens {
		println!("Token: {}", tok);
	}
	tokens
}