use crate::lexer::Token;
use crate::ast::{Ast, Expr, SubExpr, Seq, Star, Factor};

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

    fn star(&self) -> Result<Star, ParseError> {
        let factor = self.factor()?;
        let is_repeat;
        match self.is_cur_token(Token::STAR) {
            true => { is_repeat = true; self.skip(); }
            false => { is_repeat = false; }
        }
        let ret = Star { factor: Box::new(factor), is_repeat: is_repeat };
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

    fn skip(&self) {
        self.pos += 1;
    }

    fn cur_token(&self) -> Token {
        self.tokens[self.pos]
    }

    fn is_cur_token(&self, token: Token) -> bool {
        self.cur_token() == token
    }
}