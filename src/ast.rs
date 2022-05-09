use crate::lexer::Token;

pub struct Ast {
    expr: Box<Expr>
}

pub struct Expr {
    subexpr: Box<SubExpr>
}

pub struct SubExpr {
    seq: Box<Seq>,
    subexpr: Box<SubExpr>
}

pub struct Seq {

}

pub struct SubSeq {

}

pub struct Rep {
    factor: Box<Factor>,
    op: Option<Token>
}

pub struct Factor {
    pub subexpr: Option<Box<SubExpr>>,
    pub ch: Option<char>
}