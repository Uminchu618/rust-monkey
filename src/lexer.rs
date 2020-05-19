use crate::token::*;
use std::iter::FromIterator;

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
        self.skip_white_space();
        tok = match self.ch {
            '=' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::EQ
                }
                _ => Token::ASSIGN,
            },
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '+' => Token::PLUS,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '!' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::NOTEQ
                }
                _ => Token::BANG,
            },
            '-' => Token::MINUS,
            '/' => Token::SLASH,
            '*' => Token::ASTERISK,
            '<' => Token::LT,
            '>' => Token::GT,
            '\u{0}' => Token::EOF,
            _ => {
                if self.is_letter() {
                    return self.lookup_ident();
                } else if self.is_digit() {
                    return self.lookup_number();
                } else {
                    Token::ILLEGAL(self.ch)
                }
            }
        };
        self.read_char();
        return tok;
    }
    /// 1文字読み込む
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\u{0}';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    /// 1文字見る（positionは進めない）
    fn peek_char(&self) -> char {
        self.input[self.read_position]
    }

    /// 英字判定
    fn is_letter(&self) -> bool {
        self.ch.is_alphabetic()
    }

    /// 数字判定
    fn is_digit(&self) -> bool {
        self.ch.is_ascii_digit()
    }
    /// 識別子 読み込み
    fn lookup_ident(&mut self) -> Token {
        let start_position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        let ident = String::from_iter(&self.input[start_position..self.position]);
        match &*ident {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            _ => Token::IDENT(ident),
        }
    }
    /// 数字 読み込み
    fn lookup_number(&mut self) -> Token {
        let start_position = self.position;
        while self.is_digit() {
            self.read_char();
        }
        let number_str = String::from_iter(&self.input[start_position..self.position]);
        match number_str.parse() {
            Ok(number) => Token::INT(number),
            //エラーはありえない
            Err(_) => Token::ILLEGAL(self.input[start_position]),
        }
    }
    fn skip_white_space(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
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
}

#[test]
fn test_next_token() {
    let input = r"
let five = 5;
let ten = 10;

let add = fn(x,y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";
    let tests = [
        Token::LET,
        Token::IDENT("five".to_string()),
        Token::ASSIGN,
        Token::INT(5),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("ten".to_string()),
        Token::ASSIGN,
        Token::INT(10),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("add".to_string()),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENT("x".to_string()),
        Token::COMMA,
        Token::IDENT("y".to_string()),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENT("x".to_string()),
        Token::PLUS,
        Token::IDENT("y".to_string()),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("result".to_string()),
        Token::ASSIGN,
        Token::IDENT("add".to_string()),
        Token::LPAREN,
        Token::IDENT("five".to_string()),
        Token::COMMA,
        Token::IDENT("ten".to_string()),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::BANG,
        Token::MINUS,
        Token::SLASH,
        Token::ASTERISK,
        Token::INT(5),
        Token::SEMICOLON,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::GT,
        Token::INT(5),
        Token::SEMICOLON,
        Token::IF,
        Token::LPAREN,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::RPAREN,
        Token::LBRACE,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RBRACE,
        Token::ELSE,
        Token::LBRACE,
        Token::RETURN,
        Token::FALSE,
        Token::SEMICOLON,
        Token::RBRACE,
        Token::INT(10),
        Token::EQ,
        Token::INT(10),
        Token::SEMICOLON,
        Token::INT(10),
        Token::NOTEQ,
        Token::INT(9),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let mut lex = Lexer::new(input);
    for test in tests.iter() {
        let tok = lex.next_token();
        assert_eq!(tok, *test);
    }
}
