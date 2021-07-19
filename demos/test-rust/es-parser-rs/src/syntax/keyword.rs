// https://github.com/boa-dev/boa/blob/master/boa/src/syntax/ast/keyword.rs
use std::{error, fmt, str::FromStr};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Keyword {
    Await,
    Async,
    Break,
    Case,
    Catch,
    Class,
    Continue,
    Const,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    In,
    InstanceOf,
    Import,
    Let,
    New,
    Null,
    Of,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,
    Yield,
}

impl Keyword {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Await => "await",
            Self::Async => "async",
            Self::Break => "break",
            Self::Case => "case",
            Self::Catch => "catch",
            Self::Class => "class",
            Self::Continue => "continue",
            Self::Const => "const",
            Self::Debugger => "debugger",
            Self::Default => "default",
            Self::Delete => "delete",
            Self::Do => "do",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Extends => "extends",
            Self::Export => "export",
            Self::False => "false",
            Self::Finally => "finally",
            Self::For => "for",
            Self::Function => "function",
            Self::If => "if",
            Self::In => "in",
            Self::InstanceOf => "instanceof",
            Self::Import => "import",
            Self::Let => "let",
            Self::New => "new",
            Self::Null => "null",
            Self::Of => "of",
            Self::Return => "return",
            Self::Super => "super",
            Self::Switch => "switch",
            Self::This => "this",
            Self::Throw => "throw",
            Self::True => "true",
            Self::Try => "try",
            Self::TypeOf => "typeof",
            Self::Var => "var",
            Self::Void => "void",
            Self::While => "while",
            Self::With => "with",
            Self::Yield => "yield",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeywordError;
impl fmt::Display for KeywordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid token")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for KeywordError {
    fn description(&self) -> &str {
        "invalid token"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl FromStr for Keyword {
    type Err = KeywordError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "await" => Ok(Self::Await),
            "async" => Ok(Self::Async),
            "break" => Ok(Self::Break),
            "case" => Ok(Self::Case),
            "catch" => Ok(Self::Catch),
            "class" => Ok(Self::Class),
            "continue" => Ok(Self::Continue),
            "const" => Ok(Self::Const),
            "debugger" => Ok(Self::Debugger),
            "default" => Ok(Self::Default),
            "delete" => Ok(Self::Delete),
            "do" => Ok(Self::Do),
            "else" => Ok(Self::Else),
            "enum" => Ok(Self::Enum),
            "extends" => Ok(Self::Extends),
            "export" => Ok(Self::Export),
            "false" => Ok(Self::False),
            "finally" => Ok(Self::Finally),
            "for" => Ok(Self::For),
            "function" => Ok(Self::Function),
            "if" => Ok(Self::If),
            "in" => Ok(Self::In),
            "instanceof" => Ok(Self::InstanceOf),
            "import" => Ok(Self::Import),
            "let" => Ok(Self::Let),
            "new" => Ok(Self::New),
            "null" => Ok(Self::Null),
            "of" => Ok(Self::Of),
            "return" => Ok(Self::Return),
            "super" => Ok(Self::Super),
            "switch" => Ok(Self::Switch),
            "this" => Ok(Self::This),
            "throw" => Ok(Self::Throw),
            "true" => Ok(Self::True),
            "try" => Ok(Self::Try),
            "typeof" => Ok(Self::TypeOf),
            "var" => Ok(Self::Var),
            "void" => Ok(Self::Void),
            "while" => Ok(Self::While),
            "with" => Ok(Self::With),
            "yield" => Ok(Self::Yield),
            _ => Err(KeywordError),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}
