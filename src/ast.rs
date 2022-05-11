use crate::lexer::Token;
use crate::nfa::{NFA, NFAStatePair, TransitionChar};

pub struct Ast {
    pub expr: Box<Expr>
}

impl Ast {
    pub fn assemble(&self, nfa: &mut NFA) {
        self.expr.assemble(nfa);
    }
}

pub struct Expr {
    pub subexpr: Box<SubExpr>
}

impl Expr {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        self.subexpr.assemble(nfa);
    }
}


pub struct SubExpr {
    pub seq: Box<Seq>,
    pub subexpr: Option<Box<SubExpr>>
}

impl SubExpr {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        let q = nfa.new_state();
        let f = nfa.new_state();

        let n1 = self.seq.assemble(nfa);
        nfa.add_transition(q, n1.start, TransitionChar::EPS);
        nfa.add_transition(n1.accept, f, TransitionChar::EPS);
        if let Some(v) = self.subexpr {
            let n2 = v.assemble(nfa);
            nfa.add_transition(q, n2.start, TransitionChar::EPS);
            nfa.add_transition(n2.accept, f, TransitionChar::EPS);
        }

        NFAStatePair { start: q, accept: f }
    }
}


pub struct Seq {
    pub subseq: Option<Box<SubSeq>>
}

impl Seq {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {

    }
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