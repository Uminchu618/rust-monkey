#[derive(PartialEq, Debug)]
pub enum Token {
    ///トークン文字列が未知
    ILLEGAL(char),
    ///ファイル終端
    EOF,
    ///識別子
    IDENT(String),
    ///整数
    INT(i64),
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
