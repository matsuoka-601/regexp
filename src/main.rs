use std::env;
use regexp::{lexer, parser, nfa};

fn main() {
    let args: Vec<String> = env::args().collect();
    let l = lexer::Lexer::new(&args[1]);
    let tokens = l.tokenize();

    let mut p = parser::Parser::new(tokens);
    let ast = p.parse();

    // let nfa = nfa::NFA::new(ast);

    // println!("{:?}", l.tokenize());
    // println!("{:?}", &args[1]);
}
