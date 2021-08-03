use std::{iter::Peekable, str::Chars};

use crate::{
    error::{CompileError, CompileErrorKind},
    token::Token,
};

// type TokenResult = Result<Token, CompileErrorKind>;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'_> {
        Lexer {
            input: input.chars().peekable(),
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.read_char() {
            Some('+') => Token::AddVal,
            Some('-') => Token::SubVal,
            Some('>') => Token::AddPtr,
            Some('<') => Token::SubPtr,
            Some(',') => Token::GetByte,
            Some('.') => Token::PutByte,
            Some('[') => Token::LoopOpen,
            Some(']') => Token::LoopEnd,
            Some(c) => Token::UnknownChar(c),
            None => Token::EOF,
        };
        tok
    }
    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok == Token::EOF {
            None
        } else {
            Some(tok)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"[->+<],.a"#;
        let mut l = Lexer::new(input);
        let tests = [
            Token::LoopOpen,
            Token::SubVal,
            Token::AddPtr,
            Token::AddVal,
            Token::SubPtr,
            Token::LoopEnd,
            Token::GetByte,
            Token::PutByte,
            Token::UnknownChar('a'),
        ];
        for t in tests {
            let tok = l.next_token();

            assert_eq!(t, tok, "expected {} token but got {}", t, tok)
        }
    }
}
