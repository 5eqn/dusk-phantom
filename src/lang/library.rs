use std::fmt::Display;
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Lib {
    Add,
    Add1(f32),
    Sub,
    Sub1(f32),
    Mul,
    Mul1(f32),
    Div,
    Div1(f32),
    Lt,
    Lt1(f32),
    Le,
    Le1(f32),
    Gt,
    Gt1(f32),
    Ge,
    Ge1(f32),
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
            Lib::Add1(x) => write!(f, "add({:.3})", x),
            Lib::Sub1(x) => write!(f, "sub({:.3})", x),
            Lib::Mul1(x) => write!(f, "mul({:.3})", x),
            Lib::Div1(x) => write!(f, "div({:.3})", x),
            Lib::Lt1(x) => write!(f, "lt({:.3})", x),
            Lib::Le1(x) => write!(f, "le({:.3})", x),
            Lib::Gt1(x) => write!(f, "gt({:.3})", x),
            Lib::Ge1(x) => write!(f, "ge({:.3})", x),
        }
    }
}

impl Lib {
    pub fn apply(self, arg: Value) -> Value {
        match arg {
            Value::Float(f) => {
                match self {
                    Lib::Add => Value::Lib(Lib::Add1(f)),
                    Lib::Sub => Value::Lib(Lib::Sub1(f)),
                    Lib::Mul => Value::Lib(Lib::Mul1(f)),
                    Lib::Div => Value::Lib(Lib::Div1(f)),
                    Lib::Lt => Value::Lib(Lib::Lt1(f)),
                    Lib::Le => Value::Lib(Lib::Le1(f)),
                    Lib::Gt => Value::Lib(Lib::Gt1(f)),
                    Lib::Ge => Value::Lib(Lib::Ge1(f)),
                    Lib::Add1(x) => Value::Float(x + f),
                    Lib::Sub1(x) => Value::Float(x - f),
                    Lib::Mul1(x) => Value::Float(x * f),
                    Lib::Div1(x) => Value::Float(x / f),
                    Lib::Lt1(x) => Value::Bool(x < f),
                    Lib::Le1(x) => Value::Bool(x <= f),
                    Lib::Gt1(x) => Value::Bool(x > f),
                    Lib::Ge1(x) => Value::Bool(x >= f),
                }
            }
            _ => panic!("{} is not a float", arg),
        }
    }
}

impl From<Lib> for ValueType {
    fn from(lib: Lib) -> Self {
        match lib {
            Lib::Add | Lib::Sub | Lib::Mul | Lib::Div => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)))),
            Lib::Lt | Lib::Le | Lib::Gt | Lib::Ge => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)))),
            Lib::Add1(_) | Lib::Sub1(_) | Lib::Mul1(_) | Lib::Div1(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
            Lib::Lt1(_) | Lib::Le1(_) | Lib::Gt1(_) | Lib::Ge1(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)),
        }
    }
}