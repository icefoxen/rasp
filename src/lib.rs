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
			// Interestingly, we can now no longer display interned symbols 'cause we can't
			// get the associated VM...
			// WHY did I think that would make life easier?
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


pub fn tokenize(in_str : &mut std::str::Chars, accm : String) -> String {
	let inner = |chr| {
		match chr {
			'0'...'9' => accm.push(chr),
			_other => (),
		}
	};
	let c = in_str.next();
	c.map_or(accm, |_v| String::new())
}

pub fn read<'a>(in_str : &str) -> Val {
	let mut chars = in_str.chars();
	let c = chars.next();
	match c {
		Some('0') => Val::Int(0),
		Some(_stuff) => Val::Nil,
		None => Val::Nil
	}
}