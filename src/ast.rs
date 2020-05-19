pub struct Expression {}

#[derive(Debug)]
pub enum Statement {
    Let { identifier: String },
    Error(String),
}
