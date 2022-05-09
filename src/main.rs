use std::env;
use regexp::{lexer, parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    let l = lexer::Lexer::new(&args[1]);
    let tokens = l.tokenize();

    let p = parser::Parser::new(tokens);
    let ast = p.parse();

    

    println!("{:?}", l.tokenize());
    println!("{:?}", &args[1]);
}
