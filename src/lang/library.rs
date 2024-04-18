use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Lib {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Lib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lib::Add => write!(f, "add"),
            Lib::Sub => write!(f, "sub"),
            Lib::Mul => write!(f, "mul"),
            Lib::Div => write!(f, "div"),
        }
    }
}
