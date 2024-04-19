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

impl Display for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closure({}.into(), HashMap::from([{}]), {}.into())", 
            self.0, 
            self.1.iter().map(|(k, v)| format!("({}.into(), {})", k, v)).collect::<Vec<_>>().join(", "),
            self.2
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Float(f32),
    Bool(bool),
    Lib(Lib),
    Apply(Box<Value>, Vec<Value>),
    Func(Box<ValueType>, Closure),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(x) => write!(f, "Value::Float({:.3})", x),
            Value::Bool(x) => write!(f, "Value::Bool({})", x),
            Value::Lib(x) => write!(f, "Value::Lib({})", x),
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
            Value::Lib(x) => x.to_string(),
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