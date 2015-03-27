use std::fmt::*;
use std::collections;

pub struct SymbolId(i32);


impl Clone for SymbolId {
	// Really now?  Really?
	// SymbolId is a value type, essentially.  Why do I have to do this?
	fn clone(&self) -> SymbolId {
		let SymbolId(i) = *self;
		SymbolId(i)
	}
}

impl SymbolId {
	fn incr(&self) -> SymbolId {
		let SymbolId(num) = *self;
		SymbolId(num+1)
	}
}

pub enum Val {
	Int(i32),
	Nil,
	Cons(Box<Val>, Box<Val>),
	Symbol(SymbolId),
}

pub struct VmContext<'a> {
	// I THINK the lifetime specifier here says that the &str 
	// lasts as long as the VmContext does...
	symbols : std::collections::HashMap<i32, &'a str>,
	next_symbol : SymbolId,
}


impl <'a> VmContext<'a> {
	fn next_symbol<'a>(&mut self) -> SymbolId {
		// The clone() here feels stupid, but, it works.
		let sym = self.next_symbol.clone();
		self.next_symbol = self.next_symbol.incr();
		sym
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
			Val::Symbol(ref handle) => formatter.write_str("symbol")
		}
	}
}

pub fn cons(car : Val, cdr : Val) -> Val {
	let ca = Box::new(car);
	let cd = Box::new(cdr);
	Val::Cons(ca, cd)
}

fn intern_symbol<'a>(ctx : &mut VmContext<'a>, name : & 'a str) -> SymbolId {
	let id = ctx.next_symbol();
	ctx.symbols.insert(3, name);
	//ctx.symbols.insert(id, name);
	id
}

pub fn read() -> Val {
	Val::Nil
}