extern crate rasp;

use std::io::*;

fn main() {
    let cons = rasp::cons(rasp::Val::Int(10), 
    	rasp::cons(rasp::Val::Int(20), rasp::cons(rasp::Val::Int(30), rasp::Val::Nil)));
    println!("Cons is {}", cons);
    println!("Type stuff in:");
    let mut sin = stdin();
    let mut buffer = String::new();
    let inputChars = sin.read_line(&mut buffer);
    println!("You typed: {}", buffer);
}
