use std::fmt::Debug;
use crate::parser::expression::Expression;
use crate::parser::identifier::Identifier;

pub trait Statement: Debug {
    fn run(&self);
}

#[derive(Debug)]
pub struct EndStatement;

impl Statement for EndStatement{
    fn run(&self) {}
}

#[derive(Debug)]
pub struct LetStatement {
    pub(crate) identifier: Identifier,
    pub(crate) value: Expression
}

impl Statement for LetStatement {
    fn run(&self) {
        todo!()
    }
}