use super::*;

#[derive(Clone, PartialEq)]
pub enum Syntax {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Syntax>, Box<Syntax>),
}
