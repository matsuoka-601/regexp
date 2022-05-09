use crate::lexer::Token;

pub struct Ast {
    pub expr: Box<Expr>
}

pub struct Expr {
    pub subexpr: Box<SubExpr>
}

pub struct SubExpr {
    pub seq: Box<Seq>,
    pub subexpr: Option<Box<SubExpr>>
}

pub struct Seq {
    pub subseq: Option<Box<SubSeq>>
}

pub struct SubSeq {
    pub rep: Box<Rep>,
    pub subseq: Option<Box<SubSeq>>,
}

pub struct Rep {
    pub factor: Box<Factor>,
    pub op: Option<Token>
}

pub struct Factor {
    pub subexpr: Option<Box<SubExpr>>,
    pub ch: Option<char>
}