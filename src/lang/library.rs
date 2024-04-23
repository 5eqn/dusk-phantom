use std::fmt::Display;
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Lib {
    Fft,
    Beat,
    Sec,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Sin,
    Cos,
    Re,
    Im,
    Norm,
    Angle,
    Polar,
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
            Lib::Fft => write!(f, "fft"),
            Lib::Beat => write!(f, "beat"),
            Lib::Sec => write!(f, "sec"),
            Lib::Add => write!(f, "add"),
            Lib::Sub => write!(f, "sub"),
            Lib::Mul => write!(f, "mul"),
            Lib::Div => write!(f, "div"),
            Lib::Mod => write!(f, "mod"),
            Lib::Sin => write!(f, "sin"),
            Lib::Cos => write!(f, "cos"),
            Lib::Re => write!(f, "re"),
            Lib::Im => write!(f, "im"),
            Lib::Norm => write!(f, "norm"),
            Lib::Angle => write!(f, "angle"),
            Lib::Polar => write!(f, "polar"),
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
    // Check if library function is a symbol
    pub fn is_symbol(&self) -> bool {
        matches!(self, Lib::Fft | Lib::Beat | Lib::Sec)
    }

    // Apply in evaluation
    pub fn apply(&self, arg: Value, res: &Resource) -> Value {
        match self {
            Lib::Beat => Value::Float(res.beat as f32),
            Lib::Sec => Value::Float(res.second as f32),
            Lib::Fft => {
                match arg {
                    Value::Float(f) => {
                        let floor = f.floor() as usize;
                        let ceil = f.ceil() as usize;
                        if ceil >= res.fft.len() || floor >= res.fft.len() {
                            Value::Tuple(vec![Value::Float(0.0), Value::Float(0.0)])
                        } else {
                            let lower = res.fft[floor];
                            let upper = res.fft[ceil];
                            let fraction = f.fract();
                            let fraction = (1.0 - (fraction * std::f32::consts::PI).cos()) * 0.5;
                            let interpolated_value = lower + (upper - lower) * fraction;
                            Value::Tuple(vec![Value::Float(interpolated_value.re), Value::Float(interpolated_value.im)])
                        }
                    }
                    Value::Int(i) => {
                        let i = i as usize;
                        if i >= res.fft.len() {
                            Value::Tuple(vec![Value::Float(0.0), Value::Float(0.0)])
                        } else {
                            let value = res.fft[i];
                            Value::Tuple(vec![Value::Float(value.re), Value::Float(value.im)])
                        }
                    }
                    _ => panic!("lib function {} does not accept {}", self, arg)
                }
            }
            _ => self.clone().papply(arg)
        }
    }

    /// Apply in partial evaluation
    pub fn papply(self, arg: Value) -> Value {
        // Refuse to apply lib function to symbol (during partial eval stage)
        if arg.is_symbol() || self.is_symbol() {
            return Value::Apply(Value::Lib(self).into(), vec![arg]);
        }

        // Otherwise the application is typechecked, thus it should be valid
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
                    Lib::Lt1(x) => Value::Bool(x < f),
                    Lib::Le1(x) => Value::Bool(x <= f),
                    Lib::Gt1(x) => Value::Bool(x > f),
                    Lib::Ge1(x) => Value::Bool(x >= f),
                    Lib::AddI(x) => Value::Float(x as f32 + f),
                    Lib::SubI(x) => Value::Float(x as f32 - f),
                    Lib::MulI(x) => Value::Float(x as f32 * f),
                    Lib::DivI(x) => Value::Float(if f == 0.0 { 0.0 } else { x as f32 / f }),
                    Lib::ModI(x) => Value::Float(if f == 0.0 { 0.0 } else { x as f32 % f }),
                    Lib::LtI(x) => Value::Bool((x as f32) < f),
                    Lib::LeI(x) => Value::Bool(x as f32 <= f),
                    Lib::GtI(x) => Value::Bool(x as f32 > f),
                    Lib::GeI(x) => Value::Bool(x as f32 >= f),
                    _ => panic!("lib function {} does not accept float", self)
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
                    Lib::Lt1(x) => Value::Bool(x < i as f32),
                    Lib::Le1(x) => Value::Bool(x <= i as f32),
                    Lib::Gt1(x) => Value::Bool(x > i as f32),
                    Lib::Ge1(x) => Value::Bool(x >= i as f32),
                    Lib::AddI(x) => Value::Int(x + i),
                    Lib::SubI(x) => Value::Int(x - i),
                    Lib::MulI(x) => Value::Int(x * i),
                    Lib::DivI(x) => Value::Float(if i == 0 { 0.0 } else { x as f32 / i as f32 }),
                    Lib::ModI(x) => Value::Int(if i == 0 { 0 } else { x % i }),
                    Lib::LtI(x) => Value::Bool(x < i),
                    Lib::LeI(x) => Value::Bool(x <= i),
                    Lib::GtI(x) => Value::Bool(x > i),
                    Lib::GeI(x) => Value::Bool(x >= i),
                    _ => panic!("lib function {} does not accept int", self)
                }
            }
            Value::Tuple(xs) => {
                match self {
                    Lib::Re => Value::Float((&xs[0]).into()),
                    Lib::Im => Value::Float((&xs[1]).into()),
                    Lib::Norm => {
                        let re: f32 = (&xs[0]).into();
                        let im: f32 = (&xs[1]).into();
                        Value::Float((re * re + im * im).sqrt())
                    }
                    Lib::Angle => {
                        let re: f32 = (&xs[0]).into();
                        let im: f32 = (&xs[1]).into();
                        Value::Float(im.atan2(re))
                    }
                    Lib::Polar => {
                        let norm: f32 = (&xs[0]).into();
                        let angle: f32 = (&xs[1]).into();
                        Value::Tuple(vec![Value::Float(norm * angle.cos()), Value::Float(norm * angle.sin())])
                    }
                    _ => panic!("lib function {} does not accept tuple", self)
                }
            }
            other => panic!("lib function {} does not accept {}", self, other)
        }
    }
}

impl From<Lib> for ValueType {
    fn from(lib: Lib) -> Self {
        match lib {
            Lib::Fft => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Tuple(vec![ValueType::Float, ValueType::Float]))),
            Lib::Beat | Lib::Sec => ValueType::Float,
            Lib::Add | Lib::Sub | Lib::Mul | Lib::Div | Lib::Mod => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)))),
            Lib::Lt | Lib::Le | Lib::Gt | Lib::Ge => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)))),
            Lib::Add1(_) | Lib::Sub1(_) | Lib::Mul1(_) | Lib::Div1(_) | Lib::Mod1(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
            Lib::Lt1(_) | Lib::Le1(_) | Lib::Gt1(_) | Lib::Ge1(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)),
            Lib::AddI(_) | Lib::SubI(_) | Lib::MulI(_) | Lib::DivI(_) | Lib::ModI(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
            Lib::LtI(_) | Lib::LeI(_) | Lib::GtI(_) | Lib::GeI(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool)),
            Lib::Sin | Lib::Cos => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
            Lib::Re | Lib::Im | Lib::Norm | Lib::Angle => ValueType::Func(Box::new(ValueType::Tuple(vec![ValueType::Float, ValueType::Float])), Box::new(ValueType::Float)),
            Lib::Polar => ValueType::Func(Box::new(ValueType::Tuple(vec![ValueType::Float, ValueType::Float])), Box::new(ValueType::Tuple(vec![ValueType::Float, ValueType::Float]))),
        }
    }
}