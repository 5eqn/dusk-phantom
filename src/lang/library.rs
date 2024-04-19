use std::fmt::Display;
use super::*;

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

impl From<Lib> for Value {
    fn from(lib: Lib) -> Self {
        match lib {
            Lib::Add => {
                let f: Box<FF2F> = Box::new(|x, y| x + y);
                f.into()
            },
            Lib::Sub => {
                let f: Box<FF2F> = Box::new(|x, y| x - y);
                f.into()
            },
            Lib::Mul => {
                let f: Box<FF2F> = Box::new(|x, y| x * y);
                f.into()
            },
            Lib::Div => {
                let f: Box<FF2F> = Box::new(|x, y| x / y);
                f.into()
            },
            Lib::Lt => {
                let f: Box<FF2B> = Box::new(|x, y| x < y);
                f.into()
            },
            Lib::Le => {
                let f: Box<FF2B> = Box::new(|x, y| x <= y);
                f.into()
            },
            Lib::Gt => {
                let f: Box<FF2B> = Box::new(|x, y| x > y);
                f.into()
            },
            Lib::Ge => {
                let f: Box<FF2B> = Box::new(|x, y| x >= y);
                f.into()
            }
        }
    }
}