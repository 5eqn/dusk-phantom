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

impl Term {
    pub fn pretty_term(&self) -> String {
        match self {
            Term::Float(x) => format!("{:.3}", x),
            Term::Var(x) => x.to_string(),
            Term::Lib(x) => x.to_string(),
            Term::Apply(func, arg) => format!(
                "{}({})",
                func.pretty_atom(),
                arg.pretty_term(),
            ),
            Term::Func(param, name, body) => format!(
                "({}: {}) => {}", 
                name, 
                param.pretty_term(), 
                body.pretty_term(),
            ),
            Term::Let(value_type, name, body, next) => format!(
                "let {}: {} = {} in {}", 
                name, 
                value_type.pretty_term(), 
                body.pretty_term(), 
                next.pretty_term(),
            ),
        }
    }

    pub fn pretty_atom(&self) -> String {
        match self {
            f @ Term::Func(_, _, _) => format!("({})", f.pretty_term()),
            _ => self.pretty_term(),
        }
    }
}