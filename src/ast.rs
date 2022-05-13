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

        let p1 = self.seq.assemble(nfa);
        nfa.add_transition(q, p1.start, TransitionChar::EPS);
        nfa.add_transition(p1.accept, f, TransitionChar::EPS);
        if let Some(v) = self.subexpr {
            let p2 = v.assemble(nfa);
            nfa.add_transition(q, p2.start, TransitionChar::EPS);
            nfa.add_transition(p2.accept, f, TransitionChar::EPS);
        }

        NFAStatePair { start: q, accept: f }
    }
}


pub struct Seq {
    pub subseq: Option<Box<SubSeq>>
}

impl Seq {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        match self.subseq {
            Some(v) => {
                let p1 = v.assemble(nfa);
                return p1;
            }
            None => { // empty character
                let q = nfa.new_state();
                let f = nfa.new_state();
                nfa.add_transition(q, f, TransitionChar::EPS);
                return NFAStatePair { start: q, accept: f };
            }
        }
    }
}

pub struct SubSeq {
    pub rep: Box<Rep>,
    pub subseq: Option<Box<SubSeq>>,
}

impl SubSeq {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        let p1 = self.rep.assemble(nfa);
        let q = p1.start;
        let f;
        
        match self.subseq {
            Some(v) => {
                let p2 = v.assemble(nfa);
                nfa.add_transition(p1.accept, p2.start, TransitionChar::EPS);
                f = p2.accept;
            }
            None => {
                f = p1.accept;
            }
        }

        NFAStatePair { start: q, accept: f }
    }
}

pub struct Rep {
    pub factor: Box<Factor>,
    pub op: Option<Token>
}

impl Rep {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        let p1 = self.factor.assemble(nfa);

        match self.op {
            Some(v) => {
                match v {
                    Token::STAR => { return self.assemble_star(&p1, nfa); }
                    Token::PLUS => { return self.assemble_plus(&p1, nfa); }
                    Token::QUESTION => { return self.assemble_question(&p1, nfa); }
                    _ => {}
                }
            } 
            None => {
                return p1;
            }
        }
    }

    fn assemble_star(&self, p: &NFAStatePair, nfa: &mut NFA) -> NFAStatePair {
        let q = nfa.new_state();
        let f = nfa.new_state();

        nfa.add_transition(q, p.start, TransitionChar::EPS);
        nfa.add_transition(q, f, TransitionChar::EPS);
        nfa.add_transition(p.accept, p.start, TransitionChar::EPS);
        nfa.add_transition(p.accept, f, TransitionChar::EPS);

        NFAStatePair { start: q, accept: f }
    }

    fn assemble_plus(&self, p: &NFAStatePair, nfa: &mut NFA) -> NFAStatePair {
        let q = nfa.new_state();
        let f = nfa.new_state();

        nfa.add_transition(q, p.start, TransitionChar::EPS);
        nfa.add_transition(p.accept, f, TransitionChar::EPS);
        nfa.add_transition(p.accept, p.start, TransitionChar::EPS);

        NFAStatePair { start: q, accept: f }
    }

    fn assemble_question(&self, p: &NFAStatePair, nfa: &mut NFA) -> NFAStatePair {
        let q = nfa.new_state();
        let f = nfa.new_state();

        nfa.add_transition(q, p.start, TransitionChar::EPS);
        nfa.add_transition(q, f, TransitionChar::EPS);
        nfa.add_transition(p.accept, f, TransitionChar::EPS);

        NFAStatePair { start: q, accept: f }
    }
}

pub struct Factor {
    pub subexpr: Option<Box<SubExpr>>,
    pub ch: Option<char>
}

impl Factor {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        
    }
}