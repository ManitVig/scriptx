use std::rc::Rc;
use crate::lexer::lexer::{Lexer, next_token};
use crate::lexer::token::Token;
use crate::parser::parser::{parse, Parser};

mod lexer;
mod parser;
mod datatypes;
mod operators;

fn main() {
    let mut lexer = Lexer::new("
    let foo = 5 * (10 + 2);
    ");

    let mut tokens: Vec<Token> = vec![];

    let mut token = next_token(&mut lexer);

    while token.0 != Token::EOF {
        tokens.push(token.0);
        let lexer = token.1;
        token = next_token(lexer)
    }

    tokens.push(Token::EOF);

    let tokens: Rc<[Token]> = Rc::from(tokens.into_boxed_slice());

    let mut parser = Parser::new(tokens);

    println!("{:?}", parse(&mut parser))
}