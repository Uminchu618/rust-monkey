use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    ///トークン文字列が未知
    ILLEGAL(char),
    ///ファイル終端
    EOF,
    ///識別子
    IDENT(String),
    ///整数
    INT(i64),
    ///代入
    ASSIGN,
    ///四則演算
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    BANG,
    LT,
    GT,
    EQ,
    NOTEQ,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::IDENT(name) => write!(f, "{}", name),
            Token::INT(val) => write!(f, "INT({})", val),
            Token::MINUS => write!(f, "-"),
            Token::PLUS => write!(f, "+"),
            Token::BANG => write!(f, "!"),
            Token::ASTERISK => write!(f, "*"),
            Token::SLASH => write!(f, "/"),
            Token::GT => write!(f, ">"),
            Token::LT => write!(f, "<"),
            Token::EQ => write!(f, "=="),
            Token::NOTEQ => write!(f, "!="),
            Token::SEMICOLON => write!(f, ";"),
            Token::ASSIGN => write!(f, "="),
            Token::FUNCTION => write!(f, "fn"),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::COMMA => write!(f, ","),
            tok => write!(f, "{:?}", tok),
        }
    }
}