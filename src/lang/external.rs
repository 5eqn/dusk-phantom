use std::fmt::Display;
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Extern<'a> {
    Idx(&'a [f32]),
}

impl<'a> Display for Extern<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extern::Idx(values) => write!(f, "idx({:?})", values),
        }
    }
}

impl<'a> Extern<'a> {
    pub fn apply(self, arg: Value) -> Value {
        match arg {
            Value::Float(f) => {
                match self {
                    Extern::Idx(values) => {
                        let i = f as usize;
                        if i >= values.len() {
                            Value::Float(0.0)
                        } else {
                            Value::Float(values[i])
                        }
                    }
                }
            }
            _ => panic!("{} is not a float", arg),
        }
    }
}

impl<'a> From<Extern<'a>> for ValueType {
    fn from(lib: Extern<'a>) -> Self {
        match lib {
            Extern::Idx(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
        }
    }
}