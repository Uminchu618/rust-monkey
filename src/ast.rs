use crate::token::Token;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
    Int(i64),
    Ident(String),
    Boolean(bool),
    Grouped(Box<Expression>),
    If {
        condition: Box<Expression>,
        consequence: Box<Expression>,
        alternative: Option<Box<Expression>>,
    },
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    Infix {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Statement {
    Let {
        identifier: String,
        expr: Expression,
    },
    Return(Expression),
    Expr(Expression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Ident(value) => write!(f, "{}", &value),
            Expression::Int(value) => write!(f, "{}", value),
            Expression::Boolean(value) => write!(f, "{}", value),
            Expression::Grouped(value) => write!(f, "{}", value),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => match alternative {
                Some(alt) => write!(f, "If{}{{{}}}else{{{}}}", condition, consequence, alt),
                None => write!(f, "If{}{{{}}}", condition, consequence),
            },
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                operator,
                left,
                right,
            } => write!(f, "({}{}{})", left, operator, right),
            Expression::Block(statements) => {
                for stmt in statements.iter() {
                    write!(f, "{}", stmt)?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expr(expr) => write!(f, "{}", expr),
            Statement::Let { identifier, expr } => write!(f, "Let {0:}={1:}", identifier, expr),
            Statement::Return(expr) => write!(f, "return {}", expr),
        }
    }
}
