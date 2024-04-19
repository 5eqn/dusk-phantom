use std::fmt;

use super::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    Float(f32),
    Bool(bool),
    Var(Index),
    Extern(Extern),
    Apply(Box<Term>, Box<Term>),
    Func(Box<ValueType>, String, Box<Term>),
    Let(Box<ValueType>, String, Box<Term>, Box<Term>),
    Alt(Box<Term>, Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Float(value) => write!(f, "Term::Float({:.3})", value),
            Term::Bool(value) => write!(f, "Term::Bool({})", value),
            Term::Var(name) => write!(f, "Term::Var({}.into())", name),
            Term::Extern(lib) => write!(f, "Term::Extern({:?})", lib),
            Term::Apply(term1, term2) => write!(f, "Term::Apply({}.into(), {}.into())", term1, term2),
            Term::Func(value_type, name, term) => write!(f, "Term::Func({}.into(), {}.into(), {}.into())", value_type, name, term),
            Term::Let(value_type, name, term1, term2) => write!(f, "Term::Let({}.into(), {}.into(), {}.into(), {}.into())", value_type, name, term1, term2),
            Term::Alt(cond, then, else_) => write!(f, "Term::Alt({}.into(), {}.into(), {}.into())", cond, then, else_),
        }
    }
}

impl Term {
    pub fn pretty_term(&self) -> String {
        match self {
            Term::Float(x) => format!("{:.3}", x),
            Term::Bool(x) => x.to_string(),
            Term::Var(x) => x.to_string(),
            Term::Extern(x) => x.to_string(),
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
            Term::Alt(cond, then, else_) => format!(
                "if {} then {} else {}", 
                cond.pretty_term(), 
                then.pretty_term(), 
                else_.pretty_term(),
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