use crate::lexer::Token;
use crate::nfa::{NFA, NFAStatePair, TransitionChar};

pub trait AstNode {
    fn assemble(&self, nfa: &mut NFA) -> NFAStatePair;
}

pub enum Expr {
    SubExprBox(Box<SubExpr>)
}

impl AstNode for Expr {
    fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        match self {
            Self::SubExprBox(b) => b.assemble(nfa)
        }
    }
}


pub enum SubExpr {
    Union(Vec<Box<Seq>>)
}

impl AstNode for SubExpr {
    fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        let new_start = nfa.new_state();
        let new_accept = nfa.new_state();

        match self {
            Self::Union(v) => {
                for s in v {
                    let p = s.assemble(nfa);
                    nfa.add_transition(new_start, p.start, TransitionChar::EPS);
                    nfa.add_transition(p.accept, new_accept, TransitionChar::EPS);
                }
            }
        }

        NFAStatePair { start: new_start, accept: new_accept }
    }
}


pub enum Seq {
    Empty,
    SubSeqBox(Box<SubSeq>)
}

impl AstNode for Seq {
    fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        match self {
            Self::Empty => {
                let new_start = nfa.new_state();
                let new_accept = nfa.new_state();
                nfa.add_transition(new_start, new_accept, TransitionChar::EPS);
                return NFAStatePair { start: new_start, accept: new_accept };
            }
            Self::SubSeqBox(b) => { return b.assemble(nfa); }
        }
    }
}

pub enum SubSeq {
    Concat(Vec<Box<Rep>>)
}

impl AstNode for SubSeq {
    fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        let new_start = nfa.new_state();
        let new_accept = nfa.new_state();

        match self {
            Self::Concat(v) => {
                let mut last_accept = new_start;
                for r in v {
                    let p = r.assemble(nfa);
                    nfa.add_transition(last_accept, p.start, TransitionChar::EPS);
                    last_accept = p.accept;
                }
                nfa.add_transition(last_accept, new_accept, TransitionChar::EPS);
            }
        }

        NFAStatePair { start: new_start, accept: new_accept }
    }
}

pub enum Rep {
    SingleFactor(Box<Factor>),
    RepeatFactor(Box<Factor>, Token),
}

impl AstNode for Rep {
    fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        match self {
            Self::SingleFactor(b) => {
                return b.assemble(nfa);
            } 
            Self::RepeatFactor(b, t) => {
                let p = b.assemble(nfa);
                match t {
                    Token::STAR => { return self.assemble_star(&p, nfa); }
                    Token::PLUS => { return self.assemble_plus(&p, nfa); }
                    Token::QUESTION => { return self.assemble_question(&p, nfa); }
                    _ => { 
                        unreachable!(); // error handling should be added later
                    }
                }
            }
        }
    }
}

impl Rep {
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

pub enum Factor {
    BracketedSubExpr(Box<SubExpr>),
    Character(char),
}

impl Factor {
    pub fn assemble(&self, nfa: &mut NFA) -> NFAStatePair {
        match self {
            Self::BracketedSubExpr(b) => { b.assemble(nfa) }
            Self::Character(c) => {
                let q = nfa.new_state();
                let f = nfa.new_state();
                nfa.add_transition(q, f, TransitionChar::CHAR(*c));
                NFAStatePair { start: q, accept: f } 
            }
        }
    }
}