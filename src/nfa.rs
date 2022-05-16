use crate::ast::AstNode;

#[derive(Debug, Clone, Copy)]
pub struct StateID(usize);
pub const INVALID_STATE_ID: StateID = StateID(usize::MAX);

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
        self.delta.new_state();
        StateID(ret)
    }
}


pub struct NFATransition {
    v: Vec<Vec<StateID>>
}

impl NFATransition {
    fn new() -> Self {
        NFATransition { v: Vec::new() } 
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
    fn ret_char_id(&self, ch: TransitionChar) -> usize {
        match ch {
            TransitionChar::CHAR(c) => { c as usize }
            TransitionChar::EPS => { 255 }
        }
    }

    fn ret_idx(&self, from: StateID, ch: TransitionChar) -> usize {
        let char_id = self.ret_char_id(ch);
        256 * from.0 + char_id
    }

    fn new_state(&mut self) {
        for _ in 0..256 { self.v.push(Vec::new()) }
    }
}

pub struct NFAStatePair {
    pub start: StateID,
    pub accept: StateID
}