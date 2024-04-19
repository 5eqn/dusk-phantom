use super::*;
use std::{fmt::Display, sync::Arc};

#[derive(Clone, PartialEq)]
pub struct Closure(pub Box<Term>, pub Env, pub String);

impl Closure {
    pub fn apply(self, arg: Value) -> Value {
        let mut env = self.1;
        env.insert(self.2, arg);
        eval(*self.0, &env)
    }
}

impl Display for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closure({}.into(), HashMap::from([{}]), {}.into())", 
            self.0, 
            self.1.iter().map(|(k, v)| format!("({}.into(), {})", k, v)).collect::<Vec<_>>().join(", "),
            self.2
        )
    }
}

#[derive(Clone)]
pub enum Value {
    Float(f32),
    Int(i32),
    Bool(bool),
    Extern(Arc<V2V>),
    Apply(Box<Value>, Vec<Value>),
    Func(Box<ValueType>, Closure),
}

impl Value {
    pub fn apply(self, arg: Value) -> Value {
        match self {
            Value::Func(_, closure) => closure.apply(arg),
            Value::Extern(f) => f(arg),
            Value::Apply(func, mut args) => {
                args.push(arg);
                Value::Apply(func, args)
            }
            other => Value::Apply(other.into(), vec![arg]),
        }
    }
}

pub type V2V = dyn Fn(Value) -> Value + Send + Sync;
pub type I2F = dyn Fn(i32) -> f32 + Send + Sync;
pub type F2F = dyn Fn(f32) -> f32 + Send + Sync;
pub type F2B = dyn Fn(f32) -> bool + Send + Sync;
pub type FF2F = dyn Fn(f32, f32) -> f32 + Send + Sync;
pub type FF2B = dyn Fn(f32, f32) -> bool + Send + Sync;

impl From<Arc<F2F>> for Value {
    fn from(f: Arc<F2F>) -> Self {
        Value::Extern(Arc::new(move |arg| match arg {
            Value::Float(x) => Value::Float(f(x)),
            _ => panic!("Expected float"),
        }))
    }
}

impl From<Arc<I2F>> for Value {
    fn from(f: Arc<I2F>) -> Self {
        Value::Extern(Arc::new(move |arg| match arg {
            Value::Int(x) => Value::Float(f(x)),
            _ => panic!("Expected int"),
        }))
    }
}

impl From<Arc<F2B>> for Value {
    fn from(f: Arc<F2B>) -> Self {
        Value::Extern(Arc::new(move |arg| match arg {
            Value::Float(x) => Value::Bool(f(x)),
            _ => panic!("Expected float"),
        }))
    }
}

impl From<Arc<FF2F>> for Value {
    fn from(f: Arc<FF2F>) -> Self {
        Value::Extern(Arc::new(move |arg| match arg {
            Value::Float(x) => {
                let f: Arc<FF2F> = f.clone();
                let res: Arc<F2F> = Arc::new(move |y| f(x, y));
                res.into()
            }
            _ => panic!("Expected float"),
        }))
    }
}

impl From<Arc<FF2B>> for Value {
    fn from(f: Arc<FF2B>) -> Self {
        Value::Extern(Arc::new(move |arg| match arg {
            Value::Float(x) => {
                let f: Arc<FF2B> = f.clone();
                let res: Arc<F2B> = Arc::new(move |y| f(x, y));
                res.into()
            }
            _ => panic!("Expected float"),
        }))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Float(x), Value::Float(y)) => x == y,
            (Value::Int(x), Value::Int(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Apply(f1, a1), Value::Apply(f2, a2)) => f1 == f2 && a1 == a2,
            (Value::Func(p1, c1), Value::Func(p2, c2)) => p1 == p2 && c1 == c2,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(x) => write!(f, "Value::Float({:.3})", x),
            Value::Int(x) => write!(f, "Value::Int({})", x),
            Value::Bool(x) => write!(f, "Value::Bool({})", x),
            Value::Extern(_) => write!(f, "Value::Extern(_)"),
            Value::Apply(func, args) => write!(
                f,
                "Value::Apply({}.into(), vec![{}])",
                func,
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            Value::Func(param, body) => write!(f, "Value::Func({}.into(), {})", param, body),
        }
    }
}

impl Value {
    pub fn pretty_term(&self) -> String {
        match self {
            Value::Float(x) => format!("{:.3}", x),
            Value::Int(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::Extern(_) => "_".into(),
            Value::Apply(func, args) => format!(
                "{}({})",
                func.pretty_atom(),
                args.iter()
                    .map(|arg| arg.pretty_term())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            Value::Func(param, closure) => format!(
                "({}: {}) => {}", 
                closure.2, 
                param.pretty_term(), 
                closure.0.pretty_term(),
            ),
        }
    }

    pub fn pretty_atom(&self) -> String {
        match self {
            f @ Value::Func(_, _) => format!("({})", f.pretty_term()),
            _ => self.pretty_term(),
        }
    }
}