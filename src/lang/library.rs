use std::{fmt::Display, sync::Arc};
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Extern {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Display for Extern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extern::Add => write!(f, "add"),
            Extern::Sub => write!(f, "sub"),
            Extern::Mul => write!(f, "mul"),
            Extern::Div => write!(f, "div"),
            Extern::Lt => write!(f, "lt"),
            Extern::Le => write!(f, "le"),
            Extern::Gt => write!(f, "gt"),
            Extern::Ge => write!(f, "ge"),
        }
    }
}

impl From<Extern> for Value {
    fn from(lib: Extern) -> Self {
        match lib {
            Extern::Add => {
                let f: Arc<FF2F> = Arc::new(|x, y| x + y);
                f.into()
            },
            Extern::Sub => {
                let f: Arc<FF2F> = Arc::new(|x, y| x - y);
                f.into()
            },
            Extern::Mul => {
                let f: Arc<FF2F> = Arc::new(|x, y| x * y);
                f.into()
            },
            Extern::Div => {
                let f: Arc<FF2F> = Arc::new(|x, y| x / y);
                f.into()
            },
            Extern::Lt => {
                let f: Arc<FF2B> = Arc::new(|x, y| x < y);
                f.into()
            },
            Extern::Le => {
                let f: Arc<FF2B> = Arc::new(|x, y| x <= y);
                f.into()
            },
            Extern::Gt => {
                let f: Arc<FF2B> = Arc::new(|x, y| x > y);
                f.into()
            },
            Extern::Ge => {
                let f: Arc<FF2B> = Arc::new(|x, y| x >= y);
                f.into()
            }
        }
    }
}

impl From<Extern> for ValueType {
    fn from(lib: Extern) -> Self {
        match lib {
            Extern::Add | Extern::Sub | Extern::Mul | Extern::Div => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)))),
            Extern::Lt | Extern::Le | Extern::Gt | Extern::Ge => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)))),
        }
    }
}