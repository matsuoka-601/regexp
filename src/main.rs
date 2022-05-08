use std::env;
use regexp::lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let l = lexer::Lexer::new(&args[1]);
    println!("{:?}", l.tokenize());
    println!("{:?}", &args[1]);


}
