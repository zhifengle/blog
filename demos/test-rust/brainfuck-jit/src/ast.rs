use std::fmt;

#[derive(Debug)]
pub enum Node {
    Program(Box<Program>),
    Statement(Box<Statement>),
    Expression(Box<Expression>),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let statements: Vec<String> = (&self.statements)
            .into_iter()
            .map(|stmt| stmt.to_string())
            .collect();
        write!(f, "{}", statements.join(""))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Statement {
    Expression(Box<ExpressionStatement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Statement::Expression(exp) => format!("{}", exp),
        };
        write!(f, "{}", s)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Loop(Box<LoopLiteral>),
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Loop(a) => a.to_string(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct LoopLiteral {
    pub elements: Vec<Expression>,
}

impl fmt::Display for LoopLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = (&self.elements)
            .into_iter()
            .map(|e| e.to_string())
            .collect();
        write!(f, "[{}]", elements.join(""))
    }
}
