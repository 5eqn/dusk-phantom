use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Term>, Box<Term>),
}
