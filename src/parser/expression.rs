use std::collections::HashMap;
use std::fmt::Debug;
use crate::datatypes::datatypes::DataType;
use crate::operators::operators::{AddOperatorTrait, BinaryOperator, DivideOperatorTrait, MultiplyOperatorTrait, SubtractOperatorTrait, UnaryOperator};
use crate::parser::identifier::Identifier;

pub trait ExpressionLiteral: Debug {
    fn value(&self, mem_table: &HashMap<Identifier, Box<DataType>>) -> DataType;
}

#[derive(Debug)]
pub struct BinaryOperatorExpression {
    pub l: Box<dyn ExpressionLiteral>,
    pub r: Box<dyn ExpressionLiteral>,
    pub op: BinaryOperator
}

impl ExpressionLiteral for BinaryOperatorExpression {
    fn value(&self, mem_table: &HashMap<Identifier, Box<DataType>>) -> DataType {
        return match self.op {
            BinaryOperator::Add => {
                let one = self.l.value(&mem_table);
                let other = self.r.value(&mem_table);
                one.add(other)
            }
            BinaryOperator::Subtract => {
                let one = self.l.value(&mem_table);
                let other = self.r.value(&mem_table);
                one.subtract(other)
            }
            BinaryOperator::Multiply => {
                let one = self.l.value(&mem_table);
                let other = self.r.value(&mem_table);
                one.multiply(other)
            }
            BinaryOperator::Divide => {
                let one = self.l.value(&mem_table);
                let other = self.r.value(&mem_table);
                one.divide(other)
            }
            BinaryOperator::Or => {todo!()}
            BinaryOperator::And => {todo!()}
        }
    }
}

#[derive(Debug)]
pub struct UnaryOperatorExpression {
    pub inp: Box<dyn ExpressionLiteral>,
    pub op: UnaryOperator
}

impl ExpressionLiteral for UnaryOperatorExpression {
    fn value(&self, mem_table: &HashMap<Identifier, Box<DataType>>) -> DataType {
        todo!()
    }
}

#[derive(Debug)]
pub enum Expression {
    Empty,
    SingleValueExpression(Box<dyn ExpressionLiteral>),
    BinaryOperatorExpression(BinaryOperatorExpression),
    UnaryOperatorExpression(UnaryOperatorExpression)
}

impl ExpressionLiteral for Expression {
    fn value(&self, mem_table: &HashMap<Identifier, Box<DataType>>) -> DataType {
        return match self {
            Expression::Empty => {panic!("Empty expression called for execution")}
            Expression::SingleValueExpression(v) => {v.value(&mem_table)}
            Expression::BinaryOperatorExpression(v) => {v.value(&mem_table)}
            Expression::UnaryOperatorExpression(v) => {v.value(&mem_table)}
        }
    }
}