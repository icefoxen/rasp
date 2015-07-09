extern crate rasp;

//use std::io::*;

fn main() {
    let cons = rasp::cons(rasp::Val::Int(10), 
    	rasp::cons(rasp::Val::Int(20), rasp::cons(rasp::Val::Int(30), rasp::Val::Nil)));
    println!("Cons is {}", cons);

    //let mut ctx = rasp::VmContext::new();

    let in_str = "(10 30 foo 20 bop )";
    let out_str = rasp::read(in_str);
    println!("Read thing: {}", out_str);
    //println!("Type stuff in:");
    //let mut sin = stdin();
    //let mut buffer = String::new();
    //let inputChars = sin.read_line(&mut buffer);
    //println!("You typed: {}", buffer);
}
