use crate::lexer::Token;
use crate::ast::{Ast, Expr, SubExpr, SubSeq, Seq, Rep, Factor};

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

    pub fn parse(&mut self) -> Result<Ast, ParseError>{
        let expr = self.expr()?;
        let ret = Ast { expr: Box::new(expr) };
        Ok(ret)
    }

    fn expr(&mut self) -> Result<Expr, ParseError>{
        let subexpr = self.subexpr()?;
        let ret = Expr { subexpr: Box::new(subexpr) };
        self.skip_expect(Token::EOF)?;
        Ok(ret)
    }

    fn subexpr(&mut self) -> Result<SubExpr, ParseError> {
        let seq = Box::new(self.seq()?);
        let mut subexpr = None;
        match self.cur_token() {
            Token::UNION => { self.skip(); subexpr = Some(Box::new(self.subexpr()?)); }
            _ => {}
        }
        let ret = SubExpr { seq: seq, subexpr: subexpr };
        Ok(ret)
    }

    fn seq(&mut self) -> Result<Seq, ParseError> {
        let mut subseq = None;
        match self.cur_token() {
            Token::LPAREN | Token::CHARACTER(_) => { 
                subseq = Some(Box::new(self.subseq()?)); 
            }
            _ => {}
        }
        let ret = Seq { subseq: subseq };
        Ok(ret)
    }

    fn subseq(&mut self) -> Result<SubSeq, ParseError> {
        let rep = Box::new(self.rep()?);
        let mut subseq = None;
        match self.cur_token() {
            Token::LPAREN | Token::CHARACTER(_) => { subseq = Some(Box::new(self.subseq()?)); }
            _ => {}
        }
        let ret = SubSeq { rep: rep, subseq: subseq };
        Ok(ret)
    }

    fn rep(&mut self) -> Result<Rep, ParseError> {
        let factor = Box::new(self.factor()?);
        let mut op = None;
        match self.cur_token() {
            Token::PLUS | Token::QUESTION | Token::STAR => { 
                op = Some(self.cur_token().clone());
                self.skip();
            }
            _ => {}
        }
        let ret = Rep { factor: factor, op: op };
        Ok(ret)
    }

    fn factor(&mut self) -> Result<Factor, ParseError> {
        let mut subexpr = None;
        let mut ch = None;
        match self.cur_token() {
            Token::LPAREN => {
                self.skip_expect(Token::LPAREN)?;
                subexpr = Some(Box::new(self.subexpr()?));
                self.skip_expect(Token::RPAREN)?;
            }
            Token::CHARACTER(c) => { 
                ch = Some(*c);
                self.skip();
            }
            other => { return Err(ParseError::UnMatchedTokenError(other.clone())); }
        }
        let ret = Factor { subexpr: subexpr, ch: ch };
        Ok(ret)
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
}