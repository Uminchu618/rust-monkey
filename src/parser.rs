use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::token::Token;

struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer) -> Parser<'a> {
        Parser {
            lexer: lex,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        self.next_token();
        self.next_token();
        while self.cur_token != Token::EOF {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }
        return statements;
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::LET => Some(self.parse_let_statement()),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Statement {
        if let Token::IDENT(identifier) = self.peek_token.clone() {
            self.next_token();
            if self.peek_token != Token::ASSIGN {
                return Statement::Error("Let の識別子の次は=のはず".to_string());
            }
            self.next_token();
            let statement = Statement::Let {
                identifier: identifier,
            };
            while self.cur_token != Token::SEMICOLON && self.cur_token != Token::EOF {
                self.next_token();
            }
            return statement;
        } else {
            return Statement::Error("Let文に識別子がない".to_string());
        }
    }
}

#[test]
fn test_let_statements() {
    let input = r"
let x = 5;
let y = 10;
let foobar = 838383
";
    let mut lex = Lexer::new(input);
    let mut parser = Parser::new(&mut lex);
    let program = parser.parse_program();
    assert_eq!(program.len(), 3);
    let tests: Vec<&str> = vec!["x", "y", "foobar"];
    for test_pair in tests.iter().zip(program.iter()) {
        println!("{:?}", test_pair.1);
        if let Statement::Let { identifier } = test_pair.1 {
            println!("{:?}", identifier);
            assert_eq!(test_pair.0, identifier);
        }
    }
}
