use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Syntax {
    Float(f32),
    Var(String),
    Lib(Lib),
    Apply(Box<Syntax>, Box<Syntax>),
}
