use std::rc::Rc;
use crate::datatypes::datatypes::DataType;
use crate::lexer::token::Token;
use crate::operators::operators::BinaryOperator;
use crate::parser::expression::{BinaryOperatorExpression, Expression};
use crate::parser::identifier::Identifier;
use crate::parser::statement::{EndStatement, LetStatement, Statement};

pub struct Parser {
    tokens: Rc<[Token]>,
    pos: usize,
    peek: usize,
}

impl Parser {
    pub fn new(tokens:Rc<[Token]>) -> Parser {
        Parser {
            tokens,
            pos: 0,
            peek: 1,
        }
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn next(p: &mut Parser) -> &mut Parser {
        if p.peek >= p.tokens.len() {
            p.pos = p.tokens.len() - 1;
            p.peek = 0;
        } else {
            p.pos = p.peek;
            p.peek += 1;
        }
        p
    }

    pub fn peek_token(&self) -> &Token {
        if self.current_token() == &Token::EOF {
            return self.current_token();
        } else {
            return &self.tokens[self.peek]
        }
    }
}

#[derive(Debug)]
pub struct Program {
    statements: Box<[Box<dyn Statement>]>
}

impl Program {
    pub fn new_from_vec(statements: Vec<Box<dyn Statement>>) -> Program {
        Program { statements: statements.into_boxed_slice() }
    }
}

fn parse_expression(p: &mut Parser) -> (Expression, &mut Parser) {
    let mut p = p;
    let curr = p.current_token();
    let mut expression = Expression::Empty;

    match curr {
            Token::Number(_) => {
                let nex = p.peek_token();
                match nex {
                    &Token::SEMICOLON | &Token::RPAREN => {
                        expression = Expression::SingleValueExpression(Box::new(DataType::from_token(&curr)));
                        p = Parser::next(p);
                    },
                    &Token::ADD | &Token::SUBTRACT | &Token::DIVISION | &Token::MULTIPLICATION => {
                        let op = BinaryOperator::from_token(nex);

                        let left = curr.clone();

                        p = Parser::next(p);
                        p = Parser::next(p);

                        let right = parse_expression(p);

                        expression = Expression::BinaryOperatorExpression(
                            BinaryOperatorExpression{
                                op,
                                l: Box::new(DataType::from_token(&left)),
                                r: Box::new(right.0)
                            });

                        p = right.1;
                    }
                    _ => {panic!("unhandled expression type")}
                }
            },
            Token::IDENTIFIER(v) => {
                let nex = p.peek_token();
                match nex {
                    &Token::SEMICOLON | &Token::RPAREN => {
                        expression = Expression::SingleValueExpression(Box::new(Identifier(Rc::clone(v))));
                        p = Parser::next(p);
                    },
                    &Token::ADD | &Token::SUBTRACT | &Token::DIVISION | &Token::MULTIPLICATION => {
                        let op = BinaryOperator::from_token(nex);

                        let left = v.clone();

                        p = Parser::next(p);
                        p = Parser::next(p);

                        let right = parse_expression(p);

                        expression = Expression::BinaryOperatorExpression(
                            BinaryOperatorExpression{
                                op,
                                l: Box::new(Identifier(Rc::clone(&left))),
                                r: Box::new(right.0)
                            });

                        p = right.1;
                    }
                    _ => {panic!("unhandled expression type")}
                }
            },
            &Token::LPAREN => {
                p = Parser::next(p);
                let intrnl_expr_res  = parse_expression(p);
                p = intrnl_expr_res.1;

                let nex = p.peek_token();

                match nex {
                    &Token::SEMICOLON => {
                        expression = intrnl_expr_res.0;
                        p = Parser::next(p);
                        p = Parser::next(p);
                    }
                    &Token::ADD | &Token::SUBTRACT | &Token::DIVISION | &Token::MULTIPLICATION => {
                        let op = BinaryOperator::from_token(nex);
                        let left = intrnl_expr_res.0;

                        p = Parser::next(p);
                        p = Parser::next(p);

                        let right = parse_expression(p);

                        expression = Expression::BinaryOperatorExpression(
                            BinaryOperatorExpression{
                                op,
                                l: Box::new(left),
                                r: Box::new(right.0)
                            });

                        p = right.1;
                    },
                    _ => {panic!("Invalid Token")}
                }
            }
            _ => {panic!("Invalid token")}
        }
    (expression, p)
}

fn parse_let(p: &mut Parser) -> (LetStatement ,&mut Parser) {
    let mut p = p;

    let mut new_identifer = Identifier(Rc::from(""));

    let curr = p.current_token();

    match curr {
       Token::IDENTIFIER(val) => {

           let nex = p.peek_token();

           match nex {
               Token::ASSIGN => {
                   new_identifer = Identifier(Rc::clone(val));

                   p = Parser::next(p);
                   p = Parser::next(p);

                   let expr_parse_res = parse_expression(p);

                   let statement = LetStatement{ identifier: new_identifer, value: expr_parse_res.0 };
                   return (statement, expr_parse_res.1)
               },
               _ => {panic!("Expected '='")}
           }

       }
        _ => {panic!("Identifier expected")}
    }
}

pub fn parse(p: &mut Parser) -> Program {
    let mut p = p;
    let mut statements: Vec<Box<dyn Statement>> = vec![];

    let mut curr = p.current_token();

    while curr != &Token::EOF {

        match curr {
            &Token::LET{..} => {
                let parse_res = parse_let(Parser::next(p));
                p = parse_res.1;
                statements.push(Box::new(parse_res.0))
            },
            _ => {}
        }

        curr = Parser::next(p).current_token()
    }

    statements.push(Box::new(EndStatement));
    Program::new_from_vec(statements)
}

