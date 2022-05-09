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

pub struct Star {
    factor: Box<Factor>,
    is_repeat: bool
}

pub struct Factor {
    pub subexpr: Option<Box<SubExpr>>,
    pub ch: Option<char>
}