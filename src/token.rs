///トークン文字列が未知
pub const ILLEGAL: &str = "ILLEGAL";

///ファイル終端
pub const EOF: &str = "EOF";

///識別子・リテラル
pub const IDENT: &str = "IDENT"; // add, foobar , x, y
pub const INT: &str = "INT";

///演算子
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";

///デリミタ
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ":";

///カッコ
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

///キーワード
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

pub const TokenStrs: [&'static str; 14] = [
    "ILLEGAL", "EOF", "IDENT", "INT", "=", "+", ";", ";", "(", ")", "{", "}", "FUNCTION", "LET",
];

#[derive(PartialEq, Debug)]
pub enum TokenType {
    ///トークン文字列が未知
    ILLEGAL,
    ///ファイル終端
    EOF,
    ///識別子
    IDENT,
    ///整数
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

#[derive(PartialEq)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub literal: &'a str,
}
