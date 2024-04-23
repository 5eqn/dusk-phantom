use std::fmt::Display;
use realfft::num_complex::Complex;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Extern<'a> {
    FloatArray(&'a [f32]),
    ComplexArray(&'a [Complex<f32>]),
}

impl<'a> Display for Extern<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extern::FloatArray(values) => write!(f, "Extern::FloatArray({})", 
                values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
            ),
            Extern::ComplexArray(values) => write!(f, "Extern::ComplexArray({})", 
                values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
            ),
        }
    }
}

impl<'a> Extern<'a> {
    pub fn apply(&self, arg: Value<'a>) -> Value<'a> {
        match arg {
            Value::Float(f) => {
                match self {
                    Extern::FloatArray(values) => {
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
                    Extern::ComplexArray(values) => {
                        let floor = f.floor() as usize;
                        let ceil = f.ceil() as usize;
                        if ceil >= values.len() || floor >= values.len() {
                            Value::Tuple(vec![Value::Float(0.0), Value::Float(0.0)])
                        } else {
                            let lower = values[floor];
                            let upper = values[ceil];
                            let fraction = f.fract();
                            let fraction = (1.0 - (fraction * std::f32::consts::PI).cos()) * 0.5;
                            let interpolated_value = lower + (upper - lower) * fraction;
                            Value::Tuple(vec![Value::Float(interpolated_value.re), Value::Float(interpolated_value.im)])
                        }
                    }
                }
            }
            Value::Int(i) => {
                match self {
                    Extern::FloatArray(values) => {
                        let i = i as usize;
                        if i >= values.len() {
                            Value::Float(0.0)
                        } else {
                            Value::Float(values[i])
                        }
                    }
                    Extern::ComplexArray(values) => {
                        let i = i as usize;
                        if i >= values.len() {
                            Value::Tuple(vec![Value::Float(0.0), Value::Float(0.0)])
                        } else {
                            let value = values[i];
                            Value::Tuple(vec![Value::Float(value.re), Value::Float(value.im)])
                        }
                    }
                }
            }
            _ => panic!("{} is not a float", arg),
        }
    }
}