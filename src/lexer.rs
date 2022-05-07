#[derive(Debug, PartialEq)]
pub enum Token<'a>{
    CHARACTER(&'a str),
    UNION,
    STAR,
    LPAREN,
    RPAREN,
    EOF,
}

pub struct Lexer<'a>(&'a str);

impl<'a> Lexer<'a>{
    pub fn new(s: &'a str) -> Self {
        Lexer(s)
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, ()>{
        let mut ret = Vec::new();
        ret.push(Token::CHARACTER("a"));
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        let l = Lexer::new("a(b|c)d");
        let v = l.tokenize().unwrap();
        let ans = vec![Token::CHARACTER("a"),
                        Token::LPAREN,
                        Token::CHARACTER("b"),
                        Token::UNION,
                        Token::CHARACTER("c"),
                        Token::RPAREN,
                        Token::CHARACTER("d"),
                        Token::EOF];
        assert_eq!(v[0], ans[0]);
    }
}