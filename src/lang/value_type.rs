use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum ValueType {
    Float,
    Bool,
    Func(Box<ValueType>, Box<ValueType>),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Float => write!(f, "ValueType::Float"),
            ValueType::Bool => write!(f, "ValueType::Bool"),
            ValueType::Func(param, ret) => write!(f, "ValueType::Func({}.into(), {}.into())", param, ret),
        }
    }
}

impl ValueType {
    pub fn pretty_term(&self) -> String {
        match self {
            ValueType::Float => "Float".into(),
            ValueType::Bool => "Bool".into(),
            ValueType::Func(param, ret) => format!("{} -> {}", param.pretty_atom(), ret.pretty_term()),
        }
    }

    pub fn pretty_atom(&self) -> String {
        match self {
            f @ ValueType::Func(_, _) => format!("({})", f.pretty_term()),
            _ => self.pretty_term(),
        }
    }
}