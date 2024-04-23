use realfft::num_complex::Complex32;

use super::*;
use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub struct Closure<'a>(pub Box<Term>, pub Env<'a>, pub String);

impl<'a> Closure<'a> {
    pub fn apply_ref(&mut self, arg: Value<'a>) -> Value<'a> {
        self.1.push(arg);
        let result = eval_ref(&mut self.0, &mut self.1);
        self.1.pop();
        result
    }

    pub fn apply(self, arg: Value<'a>) -> Value<'a> {
        let mut env = self.1;
        env.push(arg);
        eval_closure(*self.0, env)
    }
}

impl<'a> Display for Closure<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Closure({}.into(), vec![{}], {}.into())",
            self.0,
            self.1
                .iter()
                .map(|v| format!("{}.into()", v))
                .collect::<Vec<_>>()
                .join(", "),
            self.2
        )
    }
}

#[derive(Clone)]
pub enum Value<'a> {
    /// Although there's no integer type,
    /// an integer can be seen as a float.
    /// Operation involving pure int will accelerate.
    Int(i32),
    Float(f32),
    Bool(bool),
    Lib(Lib),
    Var(Level),
    Tuple(Vec<Value<'a>>),
    Extern(Extern<'a>),
    Apply(Box<Value<'a>>, Vec<Value<'a>>),
    Func(Box<ValueType>, Closure<'a>),
    Alt(Box<Value<'a>>, Box<Value<'a>>, Box<Value<'a>>),
}

impl<'a> Value<'a> {
    /// Remove dependency on external array.
    /// This can decouple the lifetime of the value from external array.
    pub fn remove_depend<'b>(self) -> Value<'b> {
        match self {
            Value::Float(x) => Value::Float(x),
            Value::Int(x) => Value::Int(x),
            Value::Bool(x) => Value::Bool(x),
            Value::Lib(l) => Value::Lib(l),
            Value::Var(l) => Value::Var(l),
            Value::Tuple(xs) => Value::Tuple(xs.into_iter().map(|x| x.remove_depend()).collect()),
            Value::Extern(_) => panic!("Extern value is not allowed"),
            Value::Func(_, _) => panic!("Func value is not allowed"),
            Value::Apply(func, args) => Value::Apply(func.remove_depend().into(), args.into_iter().map(|x| x.remove_depend()).collect()),
            Value::Alt(cond, then, else_) => Value::Alt(cond.remove_depend().into(), then.remove_depend().into(), else_.remove_depend().into()),
        }
    }

    /// Check if value is a symbol.
    /// If it is, lib function will not be evaluated.
    pub fn is_symbol(&self) -> bool {
        match self {
            Value::Var(_) => true,
            Value::Apply(_, _) => true,
            Value::Tuple(xs) => xs.iter().any(|x| x.is_symbol()),
            _ => false,
        }
    }

    /// Apply an argument to a mutable reference of a value.
    pub fn apply_ref(&mut self, arg: Value<'a>) -> Value<'a> {
        match self {
            Value::Func(_, closure) => closure.apply_ref(arg),
            Value::Lib(l) => l.apply(arg),
            Value::Extern(e) => e.apply(arg),
            Value::Apply(func, args) => {
                let mut args = args.clone();
                args.push(arg);
                Value::Apply(func.clone(), args)
            }
            other => Value::Apply((*other).clone().into(), vec![arg]),
        }
    }

    /// Apply an argument to a value.
    pub fn apply(self, arg: Value<'a>) -> Value<'a> {
        match self {
            Value::Func(_, closure) => closure.apply(arg),
            Value::Lib(l) => l.apply(arg),
            Value::Extern(e) => e.apply(arg),
            Value::Apply(func, mut args) => {
                args.push(arg);
                Value::Apply(func, args)
            }
            other => Value::Apply(other.into(), vec![arg]),
        }
    }

    /// Treat a value as an array, collect its values at all indicies.
    pub fn collect<'b>(mut self, range: impl Iterator<Item = usize>) -> Vec<Value<'b>>
    {
        let mut values = Vec::new();
        for i in range {
            values.push(self.apply_ref(Value::Int(i as i32)).remove_depend());
        }
        values
    }
}

impl<'a> From<&Value<'a>> for f32 {
    fn from(val: &Value<'a>) -> Self {
        match val {
            Value::Float(x) => *x,
            Value::Int(x) => *x as f32,
            _ => panic!("{} is not a float", val),
        }
    }
}

impl<'a> From<Value<'a>> for f32 {
    fn from(val: Value<'a>) -> Self {
        match val {
            Value::Float(x) => x,
            Value::Int(x) => x as f32,
            _ => panic!("{} is not a float", val),
        }
    }
}

impl<'a> From<&Value<'a>> for Complex32 {
    fn from(val: &Value<'a>) -> Self {
        match val {
            Value::Tuple(xs) => {
                let re: f32 = (&xs[0]).into();
                let im: f32 = (&xs[1]).into();
                Complex32::new(re, im)
            }
            _ => panic!("{} is not a complex number", val),
        }
    }
}

impl<'a> From<Value<'a>> for Complex32 {
    fn from(val: Value<'a>) -> Self {
        match val {
            Value::Tuple(mut xs) => {
                let im: f32 = xs.pop().unwrap().into();
                let re: f32 = xs.pop().unwrap().into();
                Complex32::new(re, im)
            }
            _ => panic!("{} is not a complex number", val),
        }
    }
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Float(x), Value::Float(y)) => x == y,
            (Value::Int(x), Value::Int(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Lib(x), Value::Lib(y)) => x == y,
            (Value::Var(x), Value::Var(y)) => x == y,
            (Value::Tuple(xs), Value::Tuple(ys)) => xs == ys,
            (Value::Apply(f1, a1), Value::Apply(f2, a2)) => f1 == f2 && a1 == a2,
            (Value::Func(p1, c1), Value::Func(p2, c2)) => p1 == p2 && c1 == c2,
            (Value::Alt(c1, t1, e1), Value::Alt(c2, t2, e2)) => c1 == c2 && t1 == t2 && e1 == e2,
            _ => false,
        }
    }
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(x) => write!(f, "Value::Float({:.3})", x),
            Value::Int(x) => write!(f, "Value::Int({})", x),
            Value::Bool(x) => write!(f, "Value::Bool({})", x),
            Value::Lib(_) => write!(f, "Value::Lib(_)"),
            Value::Var(l) => write!(f, "Value::Var({})", l),
            Value::Tuple(xs) => write!(
                f,
                "Value::Tuple(vec![{}])",
                xs.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            Value::Extern(e) => write!(f, "Value::Extern({})", e),
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
            Value::Alt(cond, then, else_) => write!(
                f,
                "Value::Alt({}.into(), {}.into(), {}.into())",
                cond,
                then,
                else_
            ),
        }
    }
}

impl<'a> Value<'a> {
    pub fn pretty_term(&self) -> String {
        match self {
            Value::Float(x) => format!("{:.3}", x),
            Value::Int(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::Lib(_) => "_".into(),
            Value::Var(l) => format!("var_{}", l),
            Value::Tuple(xs) => format!(
                "({})",
                xs.iter()
                    .map(|x| x.pretty_term())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            Value::Extern(e) => e.to_string(),
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
            Value::Alt(cond, then, else_) => format!(
                "if {} then {} else {}",
                cond.pretty_term(),
                then.pretty_term(),
                else_.pretty_term(),
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

