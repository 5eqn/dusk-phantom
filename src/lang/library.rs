use std::fmt::Display;
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Lib {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Sin,
    Cos,
    Lt,
    Le,
    Gt,
    Ge,
    Add1(f32),
    Sub1(f32),
    Mul1(f32),
    Div1(f32),
    Mod1(f32),
    Lt1(f32),
    Le1(f32),
    Gt1(f32),
    Ge1(f32),
    AddI(i32),
    SubI(i32),
    MulI(i32),
    DivI(i32),
    ModI(i32),
    LtI(i32),
    LeI(i32),
    GtI(i32),
    GeI(i32),  
}

impl Display for Lib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lib::Add => write!(f, "add"),
            Lib::Sub => write!(f, "sub"),
            Lib::Mul => write!(f, "mul"),
            Lib::Div => write!(f, "div"),
            Lib::Mod => write!(f, "mod"),
            Lib::Sin => write!(f, "sin"),
            Lib::Cos => write!(f, "cos"),
            Lib::Lt => write!(f, "lt"),
            Lib::Le => write!(f, "le"),
            Lib::Gt => write!(f, "gt"),
            Lib::Ge => write!(f, "ge"),
            Lib::Add1(x) => write!(f, "add({:.3})", x),
            Lib::Sub1(x) => write!(f, "sub({:.3})", x),
            Lib::Mul1(x) => write!(f, "mul({:.3})", x),
            Lib::Div1(x) => write!(f, "div({:.3})", x),
            Lib::Mod1(x) => write!(f, "mod({:.3})", x),
            Lib::Lt1(x) => write!(f, "lt({:.3})", x),
            Lib::Le1(x) => write!(f, "le({:.3})", x),
            Lib::Gt1(x) => write!(f, "gt({:.3})", x),
            Lib::Ge1(x) => write!(f, "ge({:.3})", x),
            Lib::AddI(x) => write!(f, "add({})", x),
            Lib::SubI(x) => write!(f, "sub({})", x),
            Lib::MulI(x) => write!(f, "mul({})", x),
            Lib::DivI(x) => write!(f, "div({})", x),
            Lib::ModI(x) => write!(f, "mod({})", x),
            Lib::LtI(x) => write!(f, "lt({})", x),
            Lib::LeI(x) => write!(f, "le({})", x),
            Lib::GtI(x) => write!(f, "gt({})", x),
            Lib::GeI(x) => write!(f, "ge({})", x),
        }
    }
}

impl Lib {
    pub fn apply<'a>(&self, arg: Value<'a>) -> Value<'a> {
        match arg {
            Value::Float(f) => {
                match self {
                    Lib::Add => Value::Lib(Lib::Add1(f)),
                    Lib::Sub => Value::Lib(Lib::Sub1(f)),
                    Lib::Mul => Value::Lib(Lib::Mul1(f)),
                    Lib::Div => Value::Lib(Lib::Div1(f)),
                    Lib::Mod => Value::Lib(Lib::Mod1(f)),
                    Lib::Sin => Value::Float(f.sin()),
                    Lib::Cos => Value::Float(f.cos()),
                    Lib::Lt => Value::Lib(Lib::Lt1(f)),
                    Lib::Le => Value::Lib(Lib::Le1(f)),
                    Lib::Gt => Value::Lib(Lib::Gt1(f)),
                    Lib::Ge => Value::Lib(Lib::Ge1(f)),
                    Lib::Add1(x) => Value::Float(x + f),
                    Lib::Sub1(x) => Value::Float(x - f),
                    Lib::Mul1(x) => Value::Float(x * f),
                    Lib::Div1(x) => Value::Float(if f == 0.0 { 0.0 } else { x / f }),
                    Lib::Mod1(x) => Value::Float(if f == 0.0 { 0.0 } else { x % f }),
                    Lib::Lt1(x) => Value::Bool(*x < f),
                    Lib::Le1(x) => Value::Bool(*x <= f),
                    Lib::Gt1(x) => Value::Bool(*x > f),
                    Lib::Ge1(x) => Value::Bool(*x >= f),
                    Lib::AddI(x) => Value::Float(*x as f32 + f),
                    Lib::SubI(x) => Value::Float(*x as f32 - f),
                    Lib::MulI(x) => Value::Float(*x as f32 * f),
                    Lib::DivI(x) => Value::Float(if f == 0.0 { 0.0 } else { *x as f32 / f }),
                    Lib::ModI(x) => Value::Float(if f == 0.0 { 0.0 } else { *x as f32 % f }),
                    Lib::LtI(x) => Value::Bool((*x as f32) < f),
                    Lib::LeI(x) => Value::Bool(*x as f32 <= f),
                    Lib::GtI(x) => Value::Bool(*x as f32 > f),
                    Lib::GeI(x) => Value::Bool(*x as f32 >= f),
                }
            }
            Value::Int(i) => {
                match self {
                    Lib::Add => Value::Lib(Lib::AddI(i)),
                    Lib::Sub => Value::Lib(Lib::SubI(i)),
                    Lib::Mul => Value::Lib(Lib::MulI(i)),
                    Lib::Div => Value::Lib(Lib::DivI(i)),
                    Lib::Mod => Value::Lib(Lib::ModI(i)),
                    Lib::Sin => Value::Float((i as f32).sin()),
                    Lib::Cos => Value::Float((i as f32).cos()),
                    Lib::Lt => Value::Lib(Lib::LtI(i)),
                    Lib::Le => Value::Lib(Lib::LeI(i)),
                    Lib::Gt => Value::Lib(Lib::GtI(i)),
                    Lib::Ge => Value::Lib(Lib::GeI(i)),
                    Lib::Add1(x) => Value::Float(x + i as f32),
                    Lib::Sub1(x) => Value::Float(x - i as f32),
                    Lib::Mul1(x) => Value::Float(x * i as f32),
                    Lib::Div1(x) => Value::Float(if i == 0 { 0.0 } else { x / i as f32 }),
                    Lib::Mod1(x) => Value::Float(if i == 0 { 0.0 } else { x % i as f32 }),
                    Lib::Lt1(x) => Value::Bool(*x < i as f32),
                    Lib::Le1(x) => Value::Bool(*x <= i as f32),
                    Lib::Gt1(x) => Value::Bool(*x > i as f32),
                    Lib::Ge1(x) => Value::Bool(*x >= i as f32),
                    Lib::AddI(x) => Value::Int(x + i),
                    Lib::SubI(x) => Value::Int(x - i),
                    Lib::MulI(x) => Value::Int(x * i),
                    Lib::DivI(x) => Value::Float(if i == 0 { 0.0 } else { *x as f32 / i as f32 }),
                    Lib::ModI(x) => Value::Int(if i == 0 { 0 } else { *x % i }),
                    Lib::LtI(x) => Value::Bool(*x < i),
                    Lib::LeI(x) => Value::Bool(*x <= i),
                    Lib::GtI(x) => Value::Bool(*x > i),
                    Lib::GeI(x) => Value::Bool(*x >= i),
                }
            }
            other => Value::Apply(Value::Lib(self.clone()).into(), vec![other]),
        }
    }
}

impl From<Lib> for ValueType {
    fn from(lib: Lib) -> Self {
        match lib {
            Lib::Add | Lib::Sub | Lib::Mul | Lib::Div | Lib::Mod => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)))),
            Lib::Lt | Lib::Le | Lib::Gt | Lib::Ge => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)))),
            Lib::Add1(_) | Lib::Sub1(_) | Lib::Mul1(_) | Lib::Div1(_) | Lib::Mod1(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
            Lib::Lt1(_) | Lib::Le1(_) | Lib::Gt1(_) | Lib::Ge1(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)),
            Lib::AddI(_) | Lib::SubI(_) | Lib::MulI(_) | Lib::DivI(_) | Lib::ModI(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
            Lib::LtI(_) | Lib::LeI(_) | Lib::GtI(_) | Lib::GeI(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)),
            Lib::Sin | Lib::Cos => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
        }
    }
}