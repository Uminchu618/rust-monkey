use crate::ast::Expression;
use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::token::Token;

type ParseError = String;

struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer) -> Parser<'a> {
        Parser {
            lexer: lex,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
            errors: vec![],
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
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(error) => self.errors.push(error),
            }
            self.next_token();
        }
        return statements;
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let ident_name = self.expect_ident()?;
        println!("{}", ident_name);
        self.expect_peek(Token::ASSIGN)?;
        // self.next_token();
        let int = self.expect_int()?;
        println!("{}", int);
        while self.cur_token != Token::SEMICOLON && self.cur_token != Token::EOF {
            self.next_token();
            //とりあえず、、、進める
        }
        Ok(Statement::Let {
            identifier: ident_name,
            expr: Expression::Int(int),
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        while self.cur_token != Token::SEMICOLON && self.cur_token != Token::EOF {
            self.next_token();
            //とりあえず、、、進める
        }
        Ok(Statement::Return)
    }

    fn parse_expr_statement(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expression()?;
        if self.cur_token != Token::SEMICOLON {
            self.next_token();
        }
        Ok(Statement::Expr(expr))
    }

    fn peek_token_is(&self, tok: &Token) -> bool {
        match (&tok, &self.peek_token) {
            (Token::IDENT(_), Token::IDENT(_)) => true,
            (Token::INT(_), Token::INT(_)) => true,
            _ => tok == &self.peek_token,
        }
    }

    fn expect_peek(&mut self, tok: Token) -> Result<(), ParseError> {
        match self.peek_token_is(&tok) {
            true => {
                self.next_token();
                Ok(())
            }
            false => Err(format!(
                "expected next token to be {}, got {} instead",
                tok, self.peek_token
            )),
        }
    }

    pub fn print_error(&self) {
        for i in self.errors.iter() {
            println!("{}", i);
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match &self.cur_token {
            Token::IDENT(ident) => Ok(Expression::Ident(ident.clone())),
            _ => Err("Unknown token".to_string()),
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        let name = match &self.peek_token {
            Token::IDENT(n) => n.to_string(),
            _ => {
                return Err(format!(
                    "expected next token to be IDENT, got {} instead",
                    self.peek_token
                ))
            }
        };
        self.next_token();
        Ok(name)
    }

    fn expect_int(&mut self) -> Result<i64, ParseError> {
        let val = match &self.peek_token {
            Token::INT(v) => v.clone(),
            _ => {
                return Err(format!(
                    "expected next token to be INTEGER, got {} instead",
                    self.peek_token
                ))
            }
        };
        self.next_token();
        Ok(val)
    }
}

#[test]
fn test_let_statements() {
    let input = r"
let x = 5;
let y = 10;
let foobar = 838383;
";
    let mut lex = Lexer::new(input);
    let mut parser = Parser::new(&mut lex);
    let program = parser.parse_program();
    parser.print_error();
    assert_eq!(program.len(), 3);
    let tests: Vec<(&str, i64)> = vec![("x", 5), ("y", 10), ("foobar", 838383)];
    for test_pair in tests.iter().zip(program.iter()) {
        println!("{:?}", test_pair.1);
        if let Statement::Let { identifier, expr } = test_pair.1 {
            assert_eq!((test_pair.0).0, identifier.to_string());
            if let Expression::Int(int) = expr {
                assert_eq!((test_pair.0).1, *int);
            }
        }
    }
}

#[test]
fn test_return_statements() {
    let input = r"
return 5;
return 10;
return 838383;
";
    let mut lex = Lexer::new(input);
    let mut parser = Parser::new(&mut lex);
    let program = parser.parse_program();
    assert_eq!(program.len(), 3);
}

#[test]
fn test_parse_errors() {
    let input = r"
let x  5;
let  = 10;
let  838383;
";
    let mut lex = Lexer::new(input);
    let mut parser = Parser::new(&mut lex);
    let program = parser.parse_program();
    assert_eq!(program.len(), 0);
    parser.print_error();
}

#[test]
fn test_identifier_expression() {
    let input = r"foobar;";
    let mut lex = Lexer::new(input);
    let mut parser = Parser::new(&mut lex);
    let program = parser.parse_program();
    parser.print_error();
    assert_eq!(program.len(), 1);
    assert_eq!(parser.errors.len(), 0);
    assert_eq!(
        program[0],
        Statement::Expr(Expression::Ident("foobar".to_string()))
    );
}
