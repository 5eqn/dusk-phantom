use super::*;
use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub struct Closure(pub Box<Term>, pub Env, pub String);

impl Closure {
    pub fn apply(self, arg: Value) -> Value {
        let mut env = self.1;
        env.push(arg);
        eval(*self.0, &mut env)
    }
}

impl Display for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closure({}.into(), vec![{}], {}.into())", 
            self.0, 
            self.1.iter().map(|v| format!("{}.into()", v)).collect::<Vec<_>>().join(", "),
            self.2
        )
    }
}

#[derive(Clone)]
pub enum Value {
    Float(f32),
    Bool(bool),
    Lib(Lib),
    Apply(Box<Value>, Vec<Value>),
    Func(Box<ValueType>, Closure),
}

impl Value {
    pub fn apply(self, arg: Value) -> Value {
        match self {
            Value::Func(_, closure) => closure.apply(arg),
            Value::Lib(l) => l.apply(arg),
            Value::Apply(func, mut args) => {
                args.push(arg);
                Value::Apply(func, args)
            }
            other => Value::Apply(other.into(), vec![arg]),
        }
    }

    pub fn collect(self, range: impl Iterator<Item = usize>) -> Vec<Value> {
        range.map(move |i| self.clone().apply(Value::Float(i as f32))).collect()
    }
}

impl From<Vec<f32>> for Value {
    fn from(values: Vec<f32>) -> Self {
        Value::Lib(Lib::Idx(values))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Float(x), Value::Float(y)) => x == y,
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
            Value::Bool(x) => write!(f, "Value::Bool({})", x),
            Value::Lib(_) => write!(f, "Value::Lib(_)"),
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
            Value::Bool(x) => x.to_string(),
            Value::Lib(_) => "_".into(),
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