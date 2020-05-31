use crate::token::Token;
use std::fmt;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
    Int(i64),
    Ident(String),
    Boolean(bool),
    Grouped(Box<Expression>),
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    Infix {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
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
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                operator,
                left,
                right,
            } => write!(f, "({}{}{})", left, operator, right),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expr(expr) => write!(f, "{}", expr),
            Statement::Let{identifier,expr} => write!(f, "Let {0:}={1:}", identifier, expr),
            Statement::Return(expr) => write!(f, "return {}", expr),
        }
    }
}
