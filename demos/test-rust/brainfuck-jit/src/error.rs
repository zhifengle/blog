use core::fmt;

#[derive(Debug, thiserror::Error)]
pub enum CompileErrorKind {
    #[error("Unclosed left bracket")]
    UnclosedLeftBracket,
    #[error("Unexpected right bracket")]
    UnexpectedRightBracket,
    #[error("unknown operator")]
    UnknownOperator,
}

#[derive(Debug)]
pub struct CompileError {
    line: u32,
    col: u32,
    kind: CompileErrorKind,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}:{}", self.kind, self.line, self.col)
    }
}
