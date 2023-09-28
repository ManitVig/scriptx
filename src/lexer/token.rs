use std::rc::Rc;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Token {
    IDENTIFIER(Rc<str>),
    Number(Rc<str>),
    ILLEGAL,
    EOF,
    ASSIGN,
    ADD,
    SUBTRACT,
    MULTIPLICATION,
    DIVISION,
    LESSTHAN,
    GREATERTHAN,
    BANG,
    EQAULITY,
    NOTEQUALITY,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    SEMICOLON,
    COMMA,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET
}
