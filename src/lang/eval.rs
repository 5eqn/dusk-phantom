use std::collections::HashMap;

use super::*;

pub type Env = HashMap<String, Value>;

pub type EvalError = String;

pub fn eval(term: Term, env: &Env) -> Result<Value, EvalError> {
    match term {
        Term::Float(x) => Ok(Value::Float(x)),
        Term::Var(v) => env
            .get(&v)
            .map_or(Err(format!("{} is not in env", v)), |v| Ok(v.clone())),
        Term::Apply(func, arg) => match eval(*func, env)? {
            Value::Float(x) => Err(format!("{} is not a function", x)),
            Value::Apply(func, mut args) => match (*func, &args[0], eval(*arg, env)?) {
                (Value::Lib(Lib::Add), Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
                (Value::Lib(Lib::Sub), Value::Float(x), Value::Float(y)) => Ok(Value::Float(x - y)),
                (Value::Lib(Lib::Mul), Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
                (Value::Lib(Lib::Div), Value::Float(x), Value::Float(y)) => Ok(Value::Float(x / y)),
                (func, _, arg) => {
                    args.push(arg);
                    Ok(Value::Apply(func.into(), args))
                }
            },
            other => Ok(Value::Apply(other.into(), vec![eval(*arg, env)?])),
        },
        Term::Lib(x) => Ok(Value::Lib(x)),
    }
}
