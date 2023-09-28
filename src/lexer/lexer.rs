use std::ops::Deref;
use std::rc::Rc;
use crate::lexer::token::Token;

pub struct Lexer {
    input: Rc<str>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: Rc::from(input),
            position: 0,
            read_position: 1,
            ch: input.chars().nth(0).unwrap(),
        }
    }

    pub fn next(lexer: &mut Lexer) -> &mut Lexer {
        if lexer.read_position >= lexer.input.len() {
            lexer.ch = '\0';
        } else {
            lexer.ch = lexer.input.chars().nth(lexer.read_position).unwrap()
        }

        lexer.position = lexer.read_position;
        lexer.read_position += 1;

        lexer
    }


    pub fn peek(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0'
        } else {
            return self.input.chars().nth(self.read_position).unwrap().clone();
        }
    }
}

pub fn read_identifier(l: &mut Lexer) -> (Rc<str>, &mut Lexer) {
    let position = l.position.clone();

    let mut l = l;

    while l.ch.is_alphabetic() {
        l = Lexer::next(l);
    }

    let final_pos = l.position;

    (Rc::from(&l.input.clone()[position..final_pos]), l)
}

pub fn read_numerical(l: &mut Lexer) -> (Rc<str>, &mut Lexer) {
    let position = l.position.clone();

    let mut l = l;

    while l.ch.is_numeric() || l.ch == '.' {
        l = Lexer::next(l);
    }

    let final_pos = l.position;

    (Rc::from(&l.input.clone()[position..final_pos]), l)
}

pub fn next_token(l: &mut Lexer) -> (Token, &mut Lexer) {
    let curr = l.ch;

    if curr == ' ' ||  curr == '\r' || curr == '\t' || curr == '\n' {
        let l = Lexer::next(l);
        return next_token(l);
    }

    else if curr.is_alphabetic() {
        let token_res = read_identifier(l);

        let token_raw = Rc::clone(&token_res.0).clone();

        let token = match token_raw.deref() {
            "let" => Token::LET,
            "fn" => Token::FUNCTION,
            "return" => Token::RETURN,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            _ => Token::IDENTIFIER(Rc::clone(&token_raw))
        };

        return (token, token_res.1)
    }

    else if curr.is_numeric() {
        let token_res = read_numerical(l);

        let token_raw = Rc::clone(&token_res.0).clone();

        let token = Token::Number(Rc::clone(&token_raw));

        return (token, token_res.1)
    }

    else if curr == '=' {
        let nex = l.peek();

        if nex == '=' {
            let l = Lexer::next(l);
            let l = Lexer::next(l);
            return (Token::EQAULITY, l)
        } else {
            let l = Lexer::next(l);
            let l = Lexer::next(l);
            return (Token::ASSIGN, l)
        }
    }

    else if curr == '!' {
        let nex = l.peek();

        if nex == '=' {
            let l = Lexer::next(l);
            let l = Lexer::next(l);
            return (Token::NOTEQUALITY, l)
        } else {
            let l = Lexer::next(l);
            let l = Lexer::next(l);
            return (Token::BANG, l)
        }
    }

    let token = match curr {
        '+' => Token::ADD,
        '-' => Token::SUBTRACT,
        '*' => Token::MULTIPLICATION,
        '/' => Token::DIVISION,
        '<' => Token::LESSTHAN,
        '>' => Token::GREATERTHAN,
        ',' => Token::COMMA,
        '(' => Token::LPAREN,
        ')' => Token::RPAREN,
        '{' => Token::LBRACE,
        '}' => Token::RBRACE,
        ';' => Token::SEMICOLON,
        '\0' => Token::EOF,
        _ => Token::ILLEGAL
    };

    let l = Lexer::next(l);

    (token, l)
}