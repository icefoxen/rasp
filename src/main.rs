extern crate rasp;


fn main() {
	rasp::test();
    let cons = rasp::cons(rasp::Val::Int(10), rasp::Val::Int(20));
    println!("Cons is {}", cons);
}
