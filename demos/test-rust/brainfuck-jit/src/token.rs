use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    AddVal,   // '+'
    SubVal,   // '-'
    AddPtr,   // '>'
    SubPtr,   // '<'
    GetByte,  // ','
    PutByte,  // '.'
    LoopOpen, // '['
    LoopEnd,  // ']'
    EOF,
    UnknownChar(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::AddVal => write!(f, "+"),
            Token::SubVal => write!(f, "-"),
            Token::AddPtr => write!(f, ">"),
            Token::SubPtr => write!(f, "<"),
            Token::GetByte => write!(f, ","),
            Token::PutByte => write!(f, "."),
            Token::LoopOpen => write!(f, "["),
            Token::LoopEnd => write!(f, "]"),
            tok => write!(f, "{:?}", tok),
        }
    }
}
