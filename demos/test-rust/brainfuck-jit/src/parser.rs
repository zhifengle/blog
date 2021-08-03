use crate::{
    error::{CompileError, CompileErrorKind},
    lexer::Lexer,
    token::Token,
};

pub type ParseResult<T> = Result<T, CompileErrorKind>;

pub struct Parser<'a> {
    l: Lexer<'a>,

    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'_>) -> Parser<'_> {
        let mut l = l;
        let cur = l.next_token();
        let next = l.next_token();
        Parser {
            l,
            cur_token: cur,
            peek_token: next,
        }
    }
}
