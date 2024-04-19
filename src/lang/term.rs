use std::fmt;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Term>, Box<Term>),
    Func(Box<ValueType>, String, Box<Term>),
    Let(Box<ValueType>, String, Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Float(value) => write!(f, "Term::Float({:.3})", value),
            Term::Var(name) => write!(f, "Term::Var({}.into())", name),
            Term::Lib(lib) => write!(f, "Term::Lib({:?})", lib),
            Term::Apply(term1, term2) => write!(f, "Term::Apply({}.into(), {}.into())", term1, term2),
            Term::Func(value_type, name, term) => write!(f, "Term::Func({}.into(), {}.into(), {}.into())", value_type, name, term),
            Term::Let(value_type, name, term1, term2) => write!(f, "Term::Let({}.into(), {}.into(), {}.into(), {}.into())", value_type, name, term1, term2),
        }
    }
}