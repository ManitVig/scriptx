use crate::lexer::token::Token;

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Or,
    And
}

impl BinaryOperator {
    pub fn from_token(t: &Token) -> BinaryOperator {
        return match t {
            &Token::ADD => BinaryOperator::Add,
            &Token::SUBTRACT => BinaryOperator::Subtract,
            &Token::MULTIPLICATION => BinaryOperator::Multiply,
            &Token::DIVISION => BinaryOperator::Divide,
            _ => {panic!("Invalid operator token")}
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not
}

pub trait AddOperatorTrait {
    fn add(&self, other: Self) -> Self;
}

pub trait SubtractOperatorTrait {
    fn subtract(&self, other: Self) -> Self;
}

pub trait MultiplyOperatorTrait {
    fn multiply(&self, other: Self) -> Self;
}

pub trait DivideOperatorTrait {
    fn divide(&self, other: Self) -> Self;
}
