use crate::lexer::Token;
use crate::ast::{Ast, Expr, SubExpr, Seq, Rep, Factor};

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

    pub fn parse(&self) -> Result<Ast, ParseError>{
        let expr = self.expr()?;
        let ret = Ast { expr: Box::new(expr) };
        Ok(ret)
    }

    fn expr(&self) -> Result<Expr, ParseError>{
        let subexpr = self.subexpr()?;
        let ret = Expr { subexpr: Box::new(subexpr) };
        self.skip_expect(Token::EOF)?;
        Ok(ret)
    }

    fn subexpr(&self) -> Result<SubExpr, ParseError> {
        let seq = self.seq()?;
        
    }

    fn seq(&self) -> Result<Seq, ParseError> {
        
    }

    fn subseq(&self) -> Result<SubSeq, ParseError> {

    }

    fn rep(&self) -> Result<Rep, ParseError> {
        let factor = self.factor()?;
        let op;
        match self.cur_token() {
            Token::PLUS | Token::QUESTION | Token::STAR => { 
                op = Some(self.cur_token());
                self.skip();
            }
            _ => { op = None; }
        }
        let ret = Rep { factor: Box::new(factor), op: op };
        Ok(ret)
    }

    fn factor(&self) -> Result<Factor, ParseError> {
        let ret = Factor { subexpr: None, ch: None };
        match self.cur_token() {
            Token::LPAREN => {
                self.skip_expect(Token::LPAREN)?;
                ret.subexpr = Some(Box::new(self.subexpr()?));
                self.skip_expect(Token::RPAREN)?;
            }
            Token::CHARACTER(c) => { 
                ret.ch = Some(c);
                self.skip();
            }
            other => { return Err(ParseError::UnMatchedTokenError(other)); }
        }
        Ok(ret)
    }
}

impl Parser {
    fn skip_expect(&self, expected: Token) -> Result<(), ParseError> {
        match self.cur_token() {
            expected => { self.skip(); }
            other => { return Err(ParseError::UnExpectedTokenError(expected, other)); }
        }
        Ok(())
    }

    fn skip_if(&self, token: Token) -> bool {
        match self.cur_token() == token {
            true => { self.skip(); return true; }
            false => { return false; }
        }
    }

    fn skip(&self) {
        self.pos += 1;
    }

    fn cur_token(&self) -> Token {
        self.tokens[self.pos]
    }
}