

pub enum Val {
	Int(i32),
	Cons(Cons),
}

impl std::fmt::Display for Val {
	fn fmt(&self, formatter : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		match *self {
			Val::Int(ref i)  => formatter.write_str(&format!("{}", i)),
			Val::Cons(ref c) => formatter.write_str(&format!("{}", c)),
		}
	}
}


pub enum Cons {
	Nil,
	Cell(Box<Val>, Box<Val>),
}


impl std::fmt::Display for Cons {
	fn fmt(&self, formatter : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let formatted_str : &str = match *self {
			Cons::Nil => "nil",
			Cons::Cell(ref car, ref cdr) => &format!("({} . {})", car, cdr),
		};
		formatter.write_str(formatted_str)
	}
}
pub fn cons(car : Val, cdr : Val) -> Box<Cons> {
	let ca = Box::new(car);
	let cd = Box::new(cdr);
	let cell = Cons::Cell(ca, cd);
	Box::new(cell)
}

pub fn test() {
	println!("Foo!");
}
