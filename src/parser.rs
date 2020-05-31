use crate::ast::{Expression, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

type ParseError = String;

struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
enum Precedences {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
}

impl Precedences {
    fn get(token: &Token) -> Precedences {
        match token {
            Token::EQ | Token::NOTEQ => Precedences::Equals,
            Token::LT | Token::GT => Precedences::LessGreater,
            Token::PLUS | Token::MINUS => Precedences::Sum,
            Token::ASTERISK | Token::SLASH => Precedences::Product,
            _ => Precedences::Lowest,
        }
    }
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
                Err(error) => {
                    while self.cur_token != Token::SEMICOLON && self.cur_token != Token::EOF {
                        self.next_token();
                        //とりあえず、、、進める
                    }
                    self.errors.push(error)
                }
            }
            self.next_token();
        }
        return statements;
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        let ident_name = self.expect_ident()?;
        println!("{:?}", ident_name);
        self.expect_peek(Token::ASSIGN)?;
        self.next_token();
        println!("{:?}", Token::ASSIGN);
        let ret_val = Ok(Statement::Let{
            identifier: ident_name,
            expr: self.parse_expression(&Precedences::Lowest)?,
        });
        while self.cur_token != Token::SEMICOLON && self.cur_token != Token::EOF {
            self.next_token();
            //とりあえず、、、進める
        }
        ret_val
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.next_token();
        let ret_val = Ok(Statement::Return(
            self.parse_expression(&Precedences::Lowest)?,
        ));
        while self.cur_token != Token::SEMICOLON && self.cur_token != Token::EOF {
            self.next_token();
            //とりあえず、、、進める
        }
        return ret_val;
    }

    fn parse_expr_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression(&Precedences::Lowest)?;
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

    fn parse_expression(&mut self, precedence: &Precedences) -> Result<Expression, ParseError> {
        let mut left = match &self.cur_token {
            Token::IDENT(ident) => Ok(Expression::Ident(ident.clone())),
            Token::INT(val) => Ok(Expression::Int(val.clone())),
            Token::FALSE =>  Ok(Expression::Boolean(false)),
            Token::TRUE =>  Ok(Expression::Boolean(true)),
            Token::BANG | Token::MINUS => Ok(self.parse_prefix_expression()?),
            _ => Err("Unknown token".to_string()),
        }?;
        while !self.peek_token_is(&Token::SEMICOLON)
            && precedence < &Precedences::get(&self.peek_token)
        {
            self.next_token();
            left = self.parse_infix_expression(left)?;
        }
        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        self.next_token();
        Ok(Expression::Prefix {
            operator: token,
            right: Box::new(self.parse_expression(&Precedences::Prefix)?),
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let precedence = Precedences::get(&self.cur_token);
        let token = self.cur_token.clone();
        self.next_token();
        Ok(Expression::Infix {
            left: Box::new(left),
            operator: token,
            right: Box::new(self.parse_expression(&precedence)?),
        })
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
    assert_eq!(parser.errors.len(), 0);
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
    parser.print_error();
    println!("{:?}", program);
    assert_eq!(program.len(), 0);
    assert_eq!(parser.errors.len(), 3);
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

#[test]
fn test_int_literal_expression() {
    let input = r"5;";
    let mut lex = Lexer::new(input);
    let mut parser = Parser::new(&mut lex);
    let program = parser.parse_program();
    parser.print_error();
    assert_eq!(program.len(), 1);
    assert_eq!(parser.errors.len(), 0);
    assert_eq!(program[0], Statement::Expr(Expression::Int(5)));
}

#[test]
fn test_prefix_expression() {
    let input = ["!5;", "-15;","!true;","!false;"];
    let test_expr = [
        Statement::Expr(Expression::Prefix {
            operator: Token::BANG,
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Prefix {
            operator: Token::MINUS,
            right: Box::new(Expression::Int(15)),
        }),
        Statement::Expr(Expression::Prefix {
            operator: Token::BANG,
            right: Box::new(Expression::Boolean(true)),
        }),
        Statement::Expr(Expression::Prefix {
            operator: Token::BANG,
            right: Box::new(Expression::Boolean(false)),
        }),
    ];

    assert_eq!(input.len(), test_expr.len());
    for i in 0..input.len() {
        let mut lex = Lexer::new(input[i]);
        let mut parser = Parser::new(&mut lex);
        let program = parser.parse_program();
        parser.print_error();
        assert_eq!(program.len(), 1);
        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program[0], test_expr[i]);
    }
}

#[test]
fn test_infix_expression() {
    let input = [
        "5 + 5;", "5 - 5;", "5 * 5;", "5 / 5;", "5 < 5;", "5 > 5;", "5 == 5;", "5 != 5;",
    ];
    let test_expr = [
        Statement::Expr(Expression::Infix {
            operator: Token::PLUS,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Infix {
            operator: Token::MINUS,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Infix {
            operator: Token::ASTERISK,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Infix {
            operator: Token::SLASH,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Infix {
            operator: Token::LT,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Infix {
            operator: Token::GT,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Infix {
            operator: Token::EQ,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
        Statement::Expr(Expression::Infix {
            operator: Token::NOTEQ,
            left: Box::new(Expression::Int(5)),
            right: Box::new(Expression::Int(5)),
        }),
    ];

    for i in 0..8 {
        let mut lex = Lexer::new(input[i]);
        let mut parser = Parser::new(&mut lex);
        let program = parser.parse_program();
        parser.print_error();
        println!("{:?}", program);
        assert_eq!(program.len(), 1);
        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program[0], test_expr[i]);
    }
}

#[test]
fn test_operator_precedence_pasing() {
    let input = [
        "-a * b;",
        "!-a;",
        "a+b+c;",
        "a+b-c;",
        "a*b*c;",
        "a*b/c",
        "a+b/c;",
        "a+b*c+d/e-f;",
        "3+4;-5*5;",
        "5>4==3<4;",
        "5<4!=3>4;",
        "3+4*5==3*1+4*5;",
        "true",
        "false",
        "!true",
        "3>5 == false",
        "3<5 == true",
    ];
    let test_expr = [
        "((-a)*b)",
        "(!(-a))",
        "((a+b)+c)",
        "((a+b)-c)",
        "((a*b)*c)",
        "((a*b)/c)",
        "(a+(b/c))",
        "(((a+(b*c))+(d/e))-f)",
        "(3+4)((-5)*5)",
        "((5>4)==(3<4))",
        "((5<4)!=(3>4))",
        "((3+(4*5))==((3*1)+(4*5)))",
        "true",
        "false",
        "(!true)",
        "((3>5)==false)",
        "((3<5)==true)",
    ];
    assert_eq!(input.len(), test_expr.len());
    for i in 0..input.len() {
        let mut lex = Lexer::new(input[i]);
        let mut parser = Parser::new(&mut lex);
        let program = parser.parse_program();
        parser.print_error();
        // println!("{:?}", program);
        assert_eq!(parser.errors.len(), 0);
        let mut result_expr = String::new();
        for stmt in program {
            result_expr.push_str(&stmt.to_string());
        }
        // println!("{2:} :  {0:?} , {1:?}", result_expr, test_expr[i], i);
        assert_eq!(result_expr, test_expr[i]);
    }
}
