use super::*;
use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub enum Value {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Value>, Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(x) => write!(f, "{}", x),
            Value::Var(x) => write!(f, "{}", x),
            Value::Lib(x) => write!(f, "{}", x),
            Value::Apply(func, args) => write!(
                f,
                "{}({})",
                func,
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        }
    }
}
