use std::fmt;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Term>, Box<Term>),
    Func(Box<ValueType>, String, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Float(value) => write!(f, "{}", value),
            Term::Var(name) => write!(f, "{}", name),
            Term::Lib(lib) => write!(f, "{}", lib),
            Term::Apply(term1, term2) => write!(f, "({} {})", term1, term2),
            Term::Func(value_type, name, term) => write!(f, "({}: {} => {})", value_type, name, term),
        }
    }
}