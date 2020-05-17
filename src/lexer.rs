use crate::token::*;

/// 字句分析器
pub struct Lexer {
    /// 入力文字列
    input: Vec<char>,
    /// 入力における現在位置
    position: usize,
    /// これから読み込む位置
    read_position: usize,
    ///　現在検査中の文字
    ch: char,
}
/// 字句分析器
impl Lexer {
    /// 次のトークンを返す
    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        tok = match self.ch {
            '=' => Token {
                ttype: TokenType::ASSIGN,
                literal: ASSIGN,
            },
            ',' => Token {
                ttype: TokenType::COMMA,
                literal: COMMA,
            },
            ';' => Token {
                ttype: TokenType::SEMICOLON,
                literal: SEMICOLON,
            },
            '+' => Token {
                ttype: TokenType::PLUS,
                literal: PLUS,
            },
            '(' => Token {
                ttype: TokenType::LPAREN,
                literal: LPAREN,
            },
            ')' => Token {
                ttype: TokenType::RPAREN,
                literal: RPAREN,
            },
            '{' => Token {
                ttype: TokenType::LBRACE,
                literal: LBRACE,
            },
            '}' => Token {
                ttype: TokenType::RBRACE,
                literal: RBRACE,
            },
            _ => Token {
                ttype: TokenType::EOF,
                literal: "",
            },
        };
        self.read_char();
        return tok;
    }
    /// 1文字読み込む
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

/// 字句解析器の生成
pub fn new(input: &str) -> Lexer {
    let mut lex = Lexer {
        input: input.chars().collect(),
        position: 0,
        read_position: 0,
        ch: 0 as char,
    };
    lex.read_char();
    return lex;
}

#[test]
fn test_next_token() {
    let input = r"let five = 5;
let ten = 10;

let add = fn(x,y) {
    x + y;
};
let result = add(five, ten);
";
    let tests = [
        (TokenType::LET, "let"),
        (TokenType::IDENT, "five"),
        (TokenType::ASSIGN, "="),
        (TokenType::INT, "5"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::LET, "let"),
        (TokenType::IDENT, "ten"),
        (TokenType::ASSIGN, "="),
        (TokenType::INT, "10"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::LET, "let"),
        (TokenType::IDENT, "add"),
        (TokenType::ASSIGN, "="),
        (TokenType::FUNCTION, "fn"),
        (TokenType::LPAREN, "("),
        (TokenType::IDENT, "x"),
        (TokenType::COMMA, ","),
        (TokenType::IDENT, "y"),
        (TokenType::RPAREN, ")"),
        (TokenType::LBRACE, "{"),
        (TokenType::IDENT, "x"),
        (TokenType::PLUS, "+"),
        (TokenType::IDENT, "y"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::RBRACE, "}"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::LET, "let"),
        (TokenType::IDENT, "result"),
        (TokenType::ASSIGN, "="),
        (TokenType::IDENT, "add"),
        (TokenType::LPAREN, "("),
        (TokenType::IDENT, "five"),
        (TokenType::COMMA, ","),
        (TokenType::IDENT, "ten"),
        (TokenType::RPAREN, ")"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::EOF, ""),
    ];
    let mut lex = new(input);
    for test in tests.iter() {
        let tok = lex.next_token();
        assert_eq!(tok.ttype, test.0);
        assert_eq!(tok.literal, test.1);
    }
}
