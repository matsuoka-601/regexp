#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    CHARACTER(char),
    UNION,
    STAR,
    PLUS,
    QUESTION,
    LPAREN,
    RPAREN,
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a>{
    pub fn new(s: &'a str) -> Self {
        Lexer { input: s }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut ret = Vec::new(); 

        let mut iter = self.input.chars().peekable();
        while let Some(c) = iter.next() {
            if c == '\\' {
                ret.push(Token::CHARACTER(*iter.peek().unwrap()));
                iter.next();
                continue;
            }
            ret.push(self.match_char(c));
        }
        ret.push(Token::EOF);

        ret
    }

    fn match_char(&self, c: char) -> Token {
        match c {
            '(' => { return Token::LPAREN; }
            ')' => { return Token::RPAREN; }
            '*' => { return Token::STAR; }
            '+' => { return Token::PLUS; }
            '?' => { return Token::QUESTION; }
            '|' => { return Token::UNION; }
            _ => { return Token::CHARACTER(c); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        check_tokenize(r"a(b|c)d", 
                        vec![Token::CHARACTER('a'),
                        Token::LPAREN,
                        Token::CHARACTER('b'),
                        Token::UNION,
                        Token::CHARACTER('c'),
                        Token::RPAREN,
                        Token::CHARACTER('d')]);

        check_tokenize(r"\*\(\\a\\", 
                        vec![Token::CHARACTER('*'),
                            Token::CHARACTER('('),
                            Token::CHARACTER('\\'),
                            Token::CHARACTER('a'),
                            Token::CHARACTER('\\')]);
        
        
    }

    fn check_tokenize(input: &str, ans: Vec<Token>) {
        let l = Lexer::new(input);
        let v = l.tokenize();
        assert_eq!(v, ans);
        println!("{:?}", v);
    }
}