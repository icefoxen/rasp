use std::fmt::*;
use std::collections;

pub struct SymbolId(i32);

pub enum Val {
	Int(i32),
	Nil,
	Cons(Box<Val>, Box<Val>),
	Symbol(SymbolId),
}

pub struct VmContext<'a> {
	// I THINK the lifetime specifier here says that the &str 
	// lasts as long as the VmContext does...
	symbols : std::collections::HashMap<i32, & 'a str>,
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

fn intern_symbol(ctx : &mut VmContext, name : &str) -> SymbolId {
	SymbolId(0)
}

pub fn read() -> Val {
	Val::Nil
}