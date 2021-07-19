use std::fmt::{self, Display};

use super::keyword::Keyword;

pub enum Token {
    BooleanLiteral(bool),
    EOF,
    Identifier(String),
    Keyword(Keyword),
    NullLiteral,
    NumericLiteral(f64),
    Punctuator(String),
    StringLiteral(String),
    RegularExpression(String),
    Template(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::BooleanLiteral(val) => write!(f, "{}", val),
            Token::EOF => write!(f, "<end>"),
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Keyword(word) => write!(f, "{}", word),
            Token::NullLiteral => write!(f, "null"),
            Token::NumericLiteral(num) => write!(f, "{}", num),
            Token::Punctuator(punc) => write!(f, "{}", punc),
            Token::StringLiteral(lit) => write!(f, "{}", lit),
            Token::RegularExpression(reg) => write!(f, "{}", reg),
            Token::Template(tpl) => write!(f, "{}", tpl),
        }
    }
}
