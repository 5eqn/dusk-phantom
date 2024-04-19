use std::fmt;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Syntax {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Syntax>, Box<Syntax>),
    Func(Box<ValueType>, String, Box<Syntax>),
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Syntax::Float(value) => write!(f, "{}", value),
            Syntax::Var(name) => write!(f, "{}", name),
            Syntax::Lib(lib) => write!(f, "{}", lib),
            Syntax::Apply(func, arg) => write!(f, "({} {})", func, arg),
            Syntax::Func(return_type, name, body) => write!(f, "({}: {} => {})", name, return_type, body),
        }
    }
}