use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Lib {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Display for Lib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lib::Add => write!(f, "add"),
            Lib::Sub => write!(f, "sub"),
            Lib::Mul => write!(f, "mul"),
            Lib::Div => write!(f, "div"),
            Lib::Lt => write!(f, "lt"),
            Lib::Le => write!(f, "le"),
            Lib::Gt => write!(f, "gt"),
            Lib::Ge => write!(f, "ge"),
        }
    }
}
