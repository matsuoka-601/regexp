use crate::ast::AstNode;
use std::collections::{HashMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct StateID(usize);
pub const INVALID_STATE_ID: StateID = StateID(usize::MAX);

#[derive(Eq, PartialEq, Hash)]
pub enum TransitionChar {
    EPS,
    CHAR(char)
}

pub struct NFA {
    start: StateID,
    accept: StateID,
    delta: NFATransition,
    state_num: usize,
}

impl NFA {
    pub fn new(node: impl AstNode) -> Self {
        let mut ret = NFA { start: INVALID_STATE_ID, accept: INVALID_STATE_ID, 
                delta: NFATransition::new(), state_num: 0 };
        let p = node.assemble(&mut ret);
        ret.start = p.start; ret.accept = p.accept;
        ret
    }

    pub fn add_transition(&mut self, from: StateID, to: StateID, ch: TransitionChar) {
        self.delta.add_transition(from, to, ch);
    }

    pub fn new_state(&mut self) -> StateID {
        let ret = self.state_num;
        self.state_num += 1;
        StateID(ret)
    }
}


pub struct NFATransition {
    v: Vec<HashMap<TransitionChar, BTreeSet<StateID>>>
}

impl NFATransition {
    pub fn new() -> Self {
        NFATransition { v: Vec::new() } 
    }

    fn to_states(&self, from: StateID, ch: TransitionChar) -> &BTreeSet<StateID> {
        &self.v[from.0][&ch]
    }

    fn add_transition(&mut self, from: StateID, to: StateID, ch: TransitionChar) {
        let mp = &mut self.v[from.0];
        mp.get_mut(&ch).unwrap().insert(to);
    }
}

pub struct NFAStatePair {
    pub start: StateID,
    pub accept: StateID
}