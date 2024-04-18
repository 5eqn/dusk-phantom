use super::*;

#[derive(Clone, PartialEq)]
pub enum Term {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Term>, Box<Term>),
}
