use super::*;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Closure(pub Box<Term>, pub Env, pub String);

impl Closure {
    pub fn apply(&self, arg: Value) -> Result<Value, String> {
        let mut env = self.1.clone();
        env.insert(self.2.clone(), arg);
        eval(*self.0.clone(), &env)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Float(f32),
    Lib(Lib),
    Apply(Box<Value>, Vec<Value>),
    Func(Box<ValueType>, Closure),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(x) => write!(f, "{}", x),
            Value::Lib(x) => write!(f, "{}", x),
            Value::Apply(func, args) => write!(
                f,
                "{}({})",
                func,
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            Value::Func(param, body) => write!(f, "({}: {}) => {}", body.2, param, body.0),
        }
    }
}
