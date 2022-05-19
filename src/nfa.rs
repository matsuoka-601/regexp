use crate::ast::AstNode;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateID(usize);
pub const INVALID_STATE_ID: StateID = StateID(usize::MAX);

#[derive(Debug, Clone, Copy)]
pub enum TransitionChar {
    EPS,
    CHAR(char)
}

pub struct NFA {
    start: StateID,
    accept: StateID,
    delta: NFATransition,
}

impl NFA {
    pub fn new(node: impl AstNode) -> Self {
        let mut ret = NFA { start: INVALID_STATE_ID, accept: INVALID_STATE_ID, 
                delta: NFATransition::new() };
        let p = node.assemble(&mut ret);
        ret.start = p.start; ret.accept = p.accept;
        ret.remove_eps();
        ret
    }

    pub fn add_transition(&mut self, from: StateID, to: StateID, ch: TransitionChar) {
        self.delta.add_transition(from, to, ch);
    }

    pub fn new_state(&mut self) -> StateID {
        let ret = self.delta.state_num;
        self.delta.new_state();
        StateID(ret)
    }

    fn remove_eps(&mut self) {
        for state in 0..self.delta.state_num {
            self.remove_eps_of(StateID(state));
        }
    }

    fn remove_eps_of(&mut self, state: StateID) {
        for c in self.delta.transition_char_iter(state) {
            let mut visited = HashSet::new();
            for to_state in self.delta.to_states(state, c) { self.expand(*to_state, &mut visited); }
            let v: Vec<_> = visited.into_iter().collect();
            self.delta.set_new_transition(state, c, v);
        }
    }

    fn expand(&self, cur_state: StateID, visited: &mut HashSet<StateID>) {
        
    }
}

#[derive(Debug)]
pub struct NFATransition {
    v: Vec<Vec<StateID>>,
    state_num: usize,
}

pub const EPS_ID: u8 = 255;

impl NFATransition {
    fn new() -> Self {
        NFATransition { v: Vec::new(), state_num: 0 } 
    }

    fn to_states(&self, from: StateID, ch: TransitionChar) -> &Vec<StateID> {
        let idx = self.ret_idx(from, ch);
        &self.v[idx]
    }

    fn add_transition(&mut self, from: StateID, to: StateID, ch: TransitionChar) {
        let idx = self.ret_idx(from, ch);
        self.v[idx].push(to);
    }

    // should this function be inlined? 
    fn ret_char_id(&self, ch: TransitionChar) -> u8 {
        match ch {
            TransitionChar::CHAR(c) => { c as u8 }
            TransitionChar::EPS => { EPS_ID }
        }
    }

    fn ret_idx(&self, from: StateID, ch: TransitionChar) -> usize {
        let char_id = self.ret_char_id(ch) as usize;
        256 * from.0 + char_id
    }

    fn new_state(&mut self) {
        for _ in 0..256 { self.v.push(Vec::new()) }
        self.state_num += 1;
    }

    fn set_new_transition(&mut self, from: StateID, ch: TransitionChar, new_transition: Vec<StateID>) {
        let idx = self.ret_idx(from, ch);
        self.v[idx] = new_transition;
    }

    fn transition_char_iter(&self, from: StateID) -> Vec<TransitionChar> {
        (0..=255).filter(|c| *c != EPS_ID)
                .filter(|c| self.v[self.ret_idx(from, TransitionChar::CHAR(*c as char))].len() > 0)
                .map(|c| TransitionChar::CHAR(c as char))
                .collect()
    }
}

pub struct NFAStatePair {
    pub start: StateID,
    pub accept: StateID
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer, parser};

    #[test]
    fn test(){
        // check_nfa("a");
        // check_nfa("abc");
        // check_nfa("a|b");
        check_nfa("a*");
        check_nfa("");
        check_nfa("a*(b|c)d");
    }

    fn check_nfa(input: &str) {
        let l = lexer::Lexer::new(input);
        let tokens = l.tokenize();
    
        let mut p = parser::Parser::new(tokens);
        let ast = p.parse().unwrap();

        let nfa = NFA::new(ast);
        println!("state_num:{:?}, start:{:?}, accept:{:?}", nfa.delta.state_num, nfa.start, nfa.accept);
        show_delta(nfa.delta);
        println!("");
    }

    fn show_delta(delta: NFATransition) { // a bit dirty...
        for i in 0..delta.state_num {
            println!("{:?}", i);
            for j in 0..256 {
                let idx = i * 256 + j;
                if delta.v[idx].len() > 0 {
                    if j as u8 == EPS_ID { println!("ε:{:?}", delta.v[idx]); }
                    else { println!("{}:{:?}", (j as u8) as char, delta.v[idx]); }
                }
            }
        }
    }
}