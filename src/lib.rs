use std::fmt::*;

pub enum Val {
	Int(i32),
	Nil,
	Cons(Box<Val>, Box<Val>),
}

fn start_write_cons(car : &Val, cdr : &Val, formatter : &mut std::fmt::Formatter) -> Result {
	formatter.write_str(&format!("({}", car))
		.and(
	finish_write_cons(cdr, formatter)
	)
}

fn finish_write_cons(val : &Val, formatter : &mut std::fmt::Formatter) -> Result {
	match *val {
		// If we get another cons cell, we write the first item, a space, then recurse
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
		}
	}
}

pub fn cons(car : Val, cdr : Val) -> Val {
	let ca = Box::new(car);
	let cd = Box::new(cdr);
	let cell = Val::Cons(ca, cd);
	cell
	//Box::new(cell)
}

pub fn test() {
	println!("Foo!");
}
