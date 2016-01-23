use std::fmt::*;
use std::collections::HashMap;

type SymbolId = i32;


enum Val {
    Int(i32),
    Nil,
    Cons(Box<Val>, Box<Val>),
    Symbol(String),
}

/*
struct VmContext<'a> {
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

    fn new() -> VmContext<'a> {
        VmContext {
            next_symbol : 0,
            symbols : HashMap::new(),
        }
    }
}
*/

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
            Val::Symbol(ref name) => formatter.write_str(name)
        }
    }
}

fn cons(car : Val, cdr : Val) -> Val {
    let ca = Box::new(car);
    let cd = Box::new(cdr);
    Val::Cons(ca, cd)
}

/*
/// Takes a string and interns it as a symbol.
/// Returns the symbol's ID#
fn intern_symbol<'a>(ctx : &mut VmContext<'a>, name : & 'a str) -> SymbolId {
    let id = ctx.next_symbol();
    ctx.symbols.insert(id, name);
    id
}
*/



#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Number(i32),
    Symbol(String),
}

fn tokenize_num(first_char: char, chars : &mut std::str::Chars)  -> i32 {
    let chr_iter = chars.take_while(|x| x.is_digit(10));
    let mut buf = String::new();
    buf.push(first_char);
    for chr in chr_iter {buf.push(chr)}
    buf.parse().expect("Tried to tokenize invalid number!")
}

fn tokenize_symbol(first_char: char, chars : &mut std::str::Chars) -> String {
    let chr_iter = chars.take_while(|x| *x != ' ' && *x != ')');
    // This is awkward but works...
    // Not sure whether it's better to collect() the iterator
    // and then use push_str(), or iterate over it pushing
    // each individual character like this.  I'm guessing this.
    let mut buf = String::new();
    buf.push(first_char);
    for chr in chr_iter {buf.push(chr)}
    buf
}
// Derp.  Go off of first character.  If it's a paren, it's a paren.
// If it's a string or character, read that, then parse it.
// If it's something starting a number, read number, then parse it.  
// If not a number, then a symbol.
// Otherwise, it's a symbol.
fn tokenize(in_str : &str) -> Vec<Token> {
    let mut chars = in_str.chars();
    let mut tokens : Vec<Token> = Vec::new();
    loop {
        let chr = match chars.next() {
            Some(c) => c,
            None => break,
        };
        let token = match chr { 
            c if c.is_whitespace() => {
                continue;
            }
            '(' => {
                Token::LParen
            },
            ')' => {
                Token::RParen
            },
            '0'...'9' | '+' | '-' | '.' => 
                Token::Number(tokenize_num(chr, &mut chars)),
            _other => {
                Token::Symbol(tokenize_symbol(chr, &mut chars))
            },
        };
        tokens.push(token);
    }
    tokens
}

//use std::io::*;

fn main() {
    // let cons = cons(Val::Int(10), 
    // 		cons(Val::Int(20), cons(Val::Int(30), Val::Nil)));
    // println!("Cons is {}", cons);

    //let mut ctx = VmContext::new();

    let in_str = " (10 30 foo 20 bop )  (rawr (hoo hah))";
    //let in_str = "  (foo baz beep)  ";
    let out_str = tokenize(in_str);
    println!("Read thing: {:?}", out_str);
    //println!("Type stuff in:");
    //let mut sin = stdin();
    //let mut buffer = String::new();
    //let inputChars = sin.read_line(&mut buffer);
    //println!("You typed: {}", buffer);
}
