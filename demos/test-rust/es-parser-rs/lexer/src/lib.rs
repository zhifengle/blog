use std::usize;

use token::{lookup_identifier, Span, Token, TokenKind};

pub mod token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };
        l.read_char();
        return l;
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            }
        }
    }
    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                ch
            } else {
                panic!("out of range");
            }
        }
    }
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let t = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::EQ
                } else {
                    TokenKind::ASSIGN
                }
            }
            ';' => TokenKind::SEMICOLON,
            '(' => TokenKind::LPAREN,
            ')' => TokenKind::RPAREN,
            ',' => TokenKind::COMMA,
            '+' => TokenKind::PLUS,
            '-' => TokenKind::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::NotEq
                } else {
                    TokenKind::BANG
                }
            }
            '*' => TokenKind::ASTERISK,
            '/' => TokenKind::SLASH,
            '<' => TokenKind::LT,
            '>' => TokenKind::GT,
            '{' => TokenKind::LBRACE,
            '}' => TokenKind::RBRACE,
            '[' => TokenKind::LBRACKET,
            ':' => TokenKind::COLON,
            ']' => TokenKind::RBRACKET,
            '\u{0}' => TokenKind::EOF,
            '"' => {
                let (start, end, str) = self.read_string();
                return Token {
                    span: Span { start, end },
                    kind: TokenKind::STRING(str),
                };
            }
            _ => {
                if is_letter(self.ch) {
                    let (start, end, identifier) = self.read_identifier();
                    return Token {
                        span: Span { start, end },
                        kind: lookup_identifier(&identifier),
                    };
                } else if is_digit(self.ch) {
                    let (start, end, num) = self.read_number();
                    return Token {
                        span: Span { start, end },
                        kind: TokenKind::INT(num),
                    };
                } else {
                    TokenKind::ILLEGAL
                }
            }
        };
        self.read_char();
        return Token {
            span: Span {
                start: self.position - 1,
                end: self.read_position,
            },
            kind: t,
        };
    }
    fn read_identifier(&mut self) -> (usize, usize, String) {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let x = self.input[pos..self.position].to_string();
        return (pos, self.position, x);
    }
    fn read_number(&mut self) -> (usize, usize, i64) {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        let x = self.input[pos..self.position].parse().unwrap();
        return (pos, self.position, x);
    }
    fn read_string(&mut self) -> (usize, usize, String) {
        let pos = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\u{0}' {
                break;
            }
        }
        let x = self.input[pos..self.position].to_string();
        if self.ch == '"' {
            self.read_char();
        }
        return (pos, self.position, x);
    }
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
