use std::fmt::Display;
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Extern<'a> {
    Idx(&'a [f32]),
}

impl<'a> Display for Extern<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extern::Idx(values) => write!(f, "idx({:?})", values),
        }
    }
}

impl<'a> Extern<'a> {
    pub fn apply(&self, arg: Value<'a>) -> Value<'a> {
        match arg {
            Value::Float(f) => {
                match self {
                    Extern::Idx(values) => {
                        let floor = f.floor() as usize;
                        let ceil = f.ceil() as usize;
                        if ceil >= values.len() || floor >= values.len() {
                            Value::Float(0.0)
                        } else {
                            let lower = values[floor];
                            let upper = values[ceil];
                            let fraction = f.fract();
                            let fraction = (1.0 - (fraction * std::f32::consts::PI).cos()) * 0.5;
                            let interpolated_value = lower + (upper - lower) * fraction;
                            Value::Float(interpolated_value)
                        }
                    }
                }
            }
            Value::Int(i) => {
                match self {
                    Extern::Idx(values) => {
                        let i = i as usize;
                        if i >= values.len() {
                            Value::Float(0.0)
                        } else {
                            Value::Float(values[i])
                        }
                    }
                }
            }
            _ => panic!("{} is not a float", arg),
        }
    }
}

impl<'a> From<Extern<'a>> for ValueType {
    fn from(lib: Extern<'a>) -> Self {
        match lib {
            Extern::Idx(_) => ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)),
        }
    }
}