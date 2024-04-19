use std::fmt;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Syntax {
    Float(f32),
    Bool(bool),
    Var(String),
    Lib(Lib),
    Apply(Box<Syntax>, Box<Syntax>),
    Func(Box<ValueType>, String, Box<Syntax>),
    Let(Box<ValueType>, String, Box<Syntax>, Box<Syntax>),
    Alt(Box<Syntax>, Box<Syntax>, Box<Syntax>),
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Syntax::Float(value) => write!(f, "Syntax::Float({:.3})", value),
            Syntax::Bool(value) => write!(f, "Syntax::Bool({})", value),
            Syntax::Var(name) => write!(f, "Syntax::Var({}.into())", name),
            Syntax::Lib(lib) => write!(f, "Syntax::Lib({})", lib),
            Syntax::Apply(func, arg) => write!(f, "Syntax::Apply({}.into(), {}.into())", func, arg),
            Syntax::Func(return_type, name, body) => write!(f, "Syntax::Func({}.into(), {}.into(), {}.into())", name, return_type, body),
            Syntax::Let(value_type, name, value, body) => write!(f, "Syntax::Let({}.into(), {}.into(), {}.into(), {}.into())", name, value_type, value, body),
            Syntax::Alt(cond, then, else_) => write!(f, "Syntax::Alt({}.into(), {}.into(), {}.into())", cond, then, else_),
        }
    }
}