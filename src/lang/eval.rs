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

// Unit tests
#[cfg(test)]
pub mod tests_eval {
    use super::*;

    #[test]
    fn test_minimal() {
        let code = Term::Float(80.0);
        let env = Env::new();
        match eval(code.clone(), &env) {
            Ok(result) => assert_eq!(result, Value::Float(80.0)),
            Err(err) => panic!("failed to eval {:?}: {}", code, err),
        }
    }

    #[test]
    fn test_complex() {
        let code = Term::Apply(
            Term::Apply(
                Box::new(Term::Lib(Lib::Mul)),
                Box::new(Term::Float(1.4)),
            ).into(),
            Term::Apply(
                Term::Apply(
                    Box::new(Term::Lib(Lib::Add)),
                    Box::new(Term::Float(2.0)),
                ).into(),
                Term::Float(3.0).into(),
            ).into(),
        );
        let env = Env::new();
        match eval(code.clone(), &env) {
            Ok(result) => assert_eq!(result, Value::Float(7.0)),
            Err(err) => panic!("failed to eval {:?}: {}", code, err),
        }
    }
}