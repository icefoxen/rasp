extern crate rasp;


fn main() {
    let cons = rasp::cons(rasp::Val::Int(10), 
    	rasp::cons(rasp::Val::Int(20), rasp::cons(rasp::Val::Int(30), rasp::Val::Nil)));
    println!("Cons is {}", cons);
}
