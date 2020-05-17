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
            // TODO:2回同じ文字を書くのは気に食わない・・・（文字と文字列の違い）
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
    let input = "=+(){},;";
    struct ExpectedToken<'a> {
        expected_ttype: TokenType,
        expected_literal: &'a str,
    }
    let tests = [
        ExpectedToken {
            expected_ttype: TokenType::ASSIGN,
            expected_literal: "=",
        },
        ExpectedToken {
            expected_ttype: TokenType::PLUS,
            expected_literal: "+",
        },
        ExpectedToken {
            expected_ttype: TokenType::LPAREN,
            expected_literal: "(",
        },
        ExpectedToken {
            expected_ttype: TokenType::RPAREN,
            expected_literal: ")",
        },
        ExpectedToken {
            expected_ttype: TokenType::LBRACE,
            expected_literal: "{",
        },
        ExpectedToken {
            expected_ttype: TokenType::RBRACE,
            expected_literal: "}",
        },
        ExpectedToken {
            expected_ttype: TokenType::COMMA,
            expected_literal: ",",
        },
        ExpectedToken {
            expected_ttype: TokenType::SEMICOLON,
            expected_literal: ":",
        },
        ExpectedToken {
            expected_ttype: TokenType::EOF,
            expected_literal: "",
        },
    ];
    let mut lex = new(input);
    for test in tests.iter() {
        let tok = lex.next_token();
        assert_eq!(tok.ttype, test.expected_ttype);
        assert_eq!(tok.literal, test.expected_literal);
    }
}
