use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Float,
    Func(Box<ValueType>, Box<ValueType>),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Float => write!(f, "Float"),
            ValueType::Func(param, ret) => write!(f, "Func({}, {})", param, ret),
        }
    }
}