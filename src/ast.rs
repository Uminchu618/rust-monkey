use crate::token::Token;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
    Int(i64),
    Ident(String),
    Prefix(Box<PrefixExpression>)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Statement {
    Let { identifier: String , expr : Expression },
    Return,
    Expr(Expression),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PrefixExpression {
    pub operator : Token,
    pub right : Expression,
}