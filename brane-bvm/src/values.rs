use crate::bytecode::Function;
use specifications::common::Value as SpecValue;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Unit,
    Function(Function),
    Class(Class),
}

impl Value {
    pub fn as_spec_value(&self) -> SpecValue {
        match self {
            Value::String(value) => SpecValue::Unicode(value.clone()),
            Value::Boolean(value) => SpecValue::Boolean(value.clone()),
            Value::Integer(value) => SpecValue::Integer(value.clone()),
            Value::Real(value) => SpecValue::Real(value.clone()),
            Value::Unit => SpecValue::Unit,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
}

impl From<SpecValue> for Value {
    fn from(value: SpecValue) -> Self {
        match value {
            SpecValue::Unicode(value) => Value::String(value.clone()),
            SpecValue::Boolean(value) => Value::Boolean(value.clone()),
            SpecValue::Integer(value) => Value::Integer(value.clone()),
            SpecValue::Real(value) => Value::Real(value.clone()),
            SpecValue::Unit => Value::Unit,
            _ => unreachable!(),
        }
    }
}

impl From<Function> for Value {
    fn from(function: Function) -> Self {
        Value::Function(function)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl From<i64> for Value {
    fn from(integer: i64) -> Self {
        Value::Integer(integer)
    }
}

impl From<f64> for Value {
    fn from(real: f64) -> Self {
        Value::Real(real)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Unit
    }
}
