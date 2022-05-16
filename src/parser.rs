use crate::lexer::Token;
use crate::lexer;
use crate::ast::{Expr, SubExpr, SubSeq, Seq, Rep, Factor};

#[derive(Debug)]
pub enum ParseError {
    UnExpectedTokenError(Token, Token),
    UnMatchedTokenError(Token)
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Parser { tokens: input, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError>{
        let subexpr = self.subexpr()?;
        let ret = Expr::SubExprBox(Box::new(subexpr));
        self.skip_expect(Token::EOF)?;
        Ok(ret)
    }

    fn subexpr(&mut self) -> Result<SubExpr, ParseError> {
        let mut retvec = Vec::new();
        let seq = Box::new(self.seq()?);
        retvec.push(seq);

        while *self.cur_token() == Token::UNION {
            self.skip();
            let subexpr = Box::new(self.seq()?);
            retvec.push(subexpr);
        }

        let ret = SubExpr::Union(retvec);
        Ok(ret)
    }

    fn seq(&mut self) -> Result<Seq, ParseError> {
        if self.is_head_of_factor() { 
            let subseq = Box::new(self.subseq()?);
            Ok(Seq::SubSeqBox(subseq))
        } else {
            Ok(Seq::Empty)
        }
    }

    fn subseq(&mut self) -> Result<SubSeq, ParseError> {
        let mut retvec = Vec::new();

        while self.is_head_of_factor() {
            let subseq = Box::new(self.rep()?);
            retvec.push(subseq);
        }

        let ret = SubSeq::Concat(retvec);
        Ok(ret)
    }

    fn rep(&mut self) -> Result<Rep, ParseError> {
        let factor = Box::new(self.factor()?);
        match self.cur_token() {
            Token::PLUS | Token::QUESTION | Token::STAR => { 
                let op = self.cur_token().clone();
                self.skip();
                Ok(Rep::RepeatFactor(factor, op))
            }
            _ => { Ok(Rep::SingleFactor(factor)) }
        }
    }

    fn factor(&mut self) -> Result<Factor, ParseError> {
        match self.cur_token() {
            Token::LPAREN => {
                self.skip_expect(Token::LPAREN)?;
                let subexpr = Box::new(self.subexpr()?);
                self.skip_expect(Token::RPAREN)?;
                Ok(Factor::BracketedSubExpr(subexpr))
            }
            Token::CHARACTER(c) => { 
                let ch = *c;
                self.skip();
                Ok(Factor::Character(ch))
            }
            other => { return Err(ParseError::UnMatchedTokenError(other.clone())); }
        }
    }
}

impl Parser {
    fn skip_expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if *self.cur_token() == expected { self.skip(); return Ok(()); }
        else { return Err(ParseError::UnExpectedTokenError(expected, self.cur_token().clone())); }
    }

    fn skip(&mut self) {
        self.pos += 1;
    }

    fn cur_token(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn is_head_of_factor(&self)-> bool {
        match self.cur_token() {
            Token::CHARACTER(_) | Token::LPAREN => { true }
            _ => { false }
        }
    }
}


