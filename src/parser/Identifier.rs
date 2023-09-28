use std::collections::HashMap;
use std::rc::Rc;
use crate::datatypes::datatypes::DataType;
use crate::parser::expression::ExpressionLiteral;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Identifier(pub Rc<str>);

impl ExpressionLiteral for Identifier {
    fn value(&self, mem_table: &HashMap<Identifier, Box<DataType>>) -> DataType {
        match mem_table.get(&self) {
            None => { panic!("Variable used without declaration") }
            Some(data) => { *data.clone() }
        }
    }
}