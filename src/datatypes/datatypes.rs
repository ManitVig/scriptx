use std::collections::HashMap;
use std::fmt::Debug;
use crate::lexer::token::Token;
use crate::operators::operators::{AddOperatorTrait, DivideOperatorTrait, MultiplyOperatorTrait, SubtractOperatorTrait};
use crate::parser::expression::ExpressionLiteral;
use crate::parser::identifier::Identifier;

#[derive(Debug, Clone)]
pub enum NumberType {
    Integer(i32),
    Float(f32)
}

#[derive(Debug, Clone)]
pub enum DataType {
    Number(NumberType),
    Boolean(bool)
}

impl DataType {
    pub fn from_token(t: &Token) -> DataType {
        match t {
            &Token::TRUE => {return DataType::Boolean(true)},
            &Token::FALSE => {return DataType::Boolean(false)},
            Token::Number(val) => {
                if val.contains('.') {
                    let lit_val = val.parse::<f32>();
                    return match lit_val {
                        Ok(t) => DataType::Number(NumberType::Float(t)),
                        _err => {panic!("Invalid floating point number")}
                    }
                } else {
                    let lit_val = val.parse::<i32>();
                    return match lit_val {
                        Ok(t) =>  DataType::Number(NumberType::Integer(t)),
                        _err => {panic!("Invalid number")}
                    }
                }
            },
            _ => {panic!("Invalid datatype")}
        }
    }
}

impl ExpressionLiteral for DataType {
    fn value(&self, _mem_table: &HashMap<Identifier, Box<DataType>>) -> DataType {
        self.clone()
    }
}

impl AddOperatorTrait for DataType {
    fn add(&self, other: DataType) -> DataType {
        return match self {
            DataType::Number(one) => {
                return match other {
                    DataType::Number(other) => {
                        return match one {
                            NumberType::Integer(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Integer(one_raw.clone() + other_raw)),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float((one_raw.clone() as f32) + other_raw))
                                }
                            }
                            NumberType::Float(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() + (other_raw as f32))),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() + other_raw))
                                }
                            }
                        }
                    }
                    DataType::Boolean(_) => {panic!("The operation addition is not defined for booleans")}
                }
            }
            DataType::Boolean(_) => {panic!("The operation addition is not defined for booleans")}
        }
    }
}

impl SubtractOperatorTrait for DataType {
    fn subtract(&self, other: DataType) -> DataType {
        return match self {
            DataType::Number(one) => {
                return match other {
                    DataType::Number(other) => {
                        return match one {
                            NumberType::Integer(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Integer(one_raw.clone() - other_raw)),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float((one_raw.clone() as f32) - other_raw))
                                }
                            }
                            NumberType::Float(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() - (other_raw as f32))),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() - other_raw))
                                }
                            }
                        }
                    }
                    DataType::Boolean(_) => {panic!("The operation subtraction is not defined for booleans")}
                }
            }
            DataType::Boolean(_) => {panic!("The operation subtraction is not defined for booleans")}
        }
    }
}

impl MultiplyOperatorTrait for DataType {
    fn multiply(&self, other: DataType) -> DataType {
        return match self {
            DataType::Number(one) => {
                return match other {
                    DataType::Number(other) => {
                        return match one {
                            NumberType::Integer(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Integer(one_raw.clone() * other_raw)),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float((one_raw.clone() as f32) * other_raw))
                                }
                            }
                            NumberType::Float(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() * (other_raw as f32))),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() * other_raw))
                                }
                            }
                        }
                    }
                    DataType::Boolean(_) => {panic!("The operation multiplication is not defined for booleans")}
                }
            }
            DataType::Boolean(_) => {panic!("The operation multiplication is not defined for booleans")}
        }
    }
}

impl DivideOperatorTrait for DataType {
    fn divide(&self, other: DataType) -> DataType {
        return match self {
            DataType::Number(one) => {
                return match other {
                    DataType::Number(other) => {
                        return match one {
                            NumberType::Integer(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Integer(one_raw.clone() / other_raw)),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float((one_raw.clone() as f32) / other_raw))
                                }
                            }
                            NumberType::Float(one_raw) => {
                                match other {
                                    NumberType::Integer(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() / (other_raw as f32))),
                                    NumberType::Float(other_raw) => DataType::Number(NumberType::Float(one_raw.clone() / other_raw))
                                }
                            }
                        }
                    }
                    DataType::Boolean(_) => {panic!("The operation division is not defined for booleans")}
                }
            }
            DataType::Boolean(_) => {panic!("The operation division is not defined for booleans")}
        }
    }
}