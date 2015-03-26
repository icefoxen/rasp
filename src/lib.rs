

pub enum Val {
	Int(i32),
	Cons(Cons),
}

pub enum Cons {
	Nil,
	Cell(Box<Val>, Box<Val>),
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
