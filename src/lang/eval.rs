use super::*;

pub type Env = Vec<Value>;

/// Evaluate a reference to a term
/// `term` and `env` will only be temporarily mutated
pub fn eval(term: &mut Term, env: &mut Env, res: &Resource) -> Value {
    match term {
        Term::Float(x) => Value::Float(*x),
        Term::Bool(x) => Value::Bool(*x),
        Term::Var(v) => env
            .get(env.len() - *v as usize - 1)
            .unwrap()
            .clone(),
        Term::Apply(func, arg) => eval(func, env, res).apply(eval(arg, env, res), res),
        Term::Lib(x) => Value::Lib(x.clone()),
        Term::Tuple(terms) => Value::Tuple(terms.iter_mut().map(|t| eval(t, env, res)).collect()),
        Term::Func(return_type, name, body) => Value::Func(
            return_type.clone(),
            Closure(body.clone(), env.clone(), name.clone()),
        ),
        Term::Let(_, _, body, next) => {
            let value = eval(body, env, res);
            env.push(value);
            let result = eval(next, env, res);
            env.pop();
            result
        }
        Term::Alt(cond, then, else_) => match eval(cond, env, res) {
            Value::Bool(true) => eval(then, env, res),
            Value::Bool(false) => eval(else_, env, res),
            other => panic!("{} is not a boolean", other),
        },
    }
}

/// Partially evaluate a term
/// `env` will only be temporarily mutated
pub fn peval(term: Term, env: &mut Env) -> Value {
    match term {
        Term::Float(x) => Value::Float(x),
        Term::Bool(x) => Value::Bool(x),
        Term::Var(v) => env
            .get(env.len() - v as usize - 1)
            .unwrap()
            .clone(),
        Term::Apply(func, arg) => peval(*func, env).papply(peval(*arg, env)),
        Term::Lib(x) => Value::Lib(x),
        Term::Tuple(terms) => Value::Tuple(terms.into_iter().map(|t| peval(t, env)).collect()),
        Term::Func(return_type, name, body) => Value::Func(
            return_type,
            Closure(body, env.clone(), name),
        ),
        Term::Let(_, _, body, next) => {
            let value = peval(*body, env);
            env.push(value);
            let result = peval(*next, env);
            env.pop();
            result
        }
        Term::Alt(cond, then, else_) => match peval(*cond, env) {
            Value::Bool(true) => peval(*then, env),
            Value::Bool(false) => peval(*else_, env),
            other => {
                let then = peval(*then, env);
                let else_ = peval(*else_, env);
                Value::Alt(other.into(), then.into(), else_.into())
            }
        },
    }
}

/// Partially evaluate a closure (which includes owned env)
/// Consumes the environment
pub fn peval_closure(term: Term, mut env: Env) -> Value {
    match term {
        Term::Float(x) => Value::Float(x),
        Term::Bool(x) => Value::Bool(x),
        Term::Var(v) => env.swap_remove(env.len() - v as usize - 1),
        Term::Apply(func, arg) => {
            let arg = peval(*arg, &mut env);
            peval_closure(*func, env).papply(arg)
        },
        Term::Lib(x) => Value::Lib(x),
        Term::Tuple(terms) => Value::Tuple(terms.into_iter().map(|t| peval(t, &mut env)).collect()),
        Term::Func(return_type, name, body) => Value::Func(
            return_type,
            Closure(body, env, name),
        ),
        Term::Let(_, _, body, next) => {
            let value = peval(*body, &mut env);
            env.push(value);
            peval_closure(*next, env)
        }
        Term::Alt(cond, then, else_) => match peval(*cond, &mut env) {
            Value::Bool(true) => peval_closure(*then, env),
            Value::Bool(false) => peval_closure(*else_, env),
            other => {
                let then = peval(*then, &mut env);
                let else_ = peval_closure(*else_, env);
                Value::Alt(other.into(), then.into(), else_.into())
            }
        },
    }
}

// Unit tests
#[cfg(test)]
pub mod tests_eval {
    use super::*;

    #[test]
    fn test_minimal() {
        let code = Term::Float(80.0);
        let mut env = Env::new();
        match peval(code.clone(), &mut env) {
            Value::Float(x) => assert_eq!(x, 80.0),
            result => panic!("result of {} is not float: {}", code, result),
        }
    }

    #[test]
    fn test_numeric() {
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
        let mut env = Env::new();
        match peval(code.clone(), &mut env) {
            Value::Float(x) => assert_eq!(x, 7.0),
            result => panic!("result of {} is not float: {}", code, result),
        }
    }

    #[test]
    fn test_id() {
        let code = Term::Apply(
            Box::new(Term::Func(
                Box::new(ValueType::Float),
                "x".to_string(),
                Box::new(Term::Var(0)),
            )),
            Box::new(Term::Float(1.4)),
        );
        let mut env = Env::new();
        match peval(code.clone(), &mut env) {
            Value::Float(x) => assert_eq!(x, 1.4),
            result => panic!("result of {} is not float: {}", code, result),
        }
    }

    #[test]
    fn test_let() {
        let code = Term::Let(
            ValueType::Float.into(),
            "x".to_string(),
            Box::new(Term::Float(80.0)),
            Box::new(Term::Var(0)),
        );
        let mut env = Env::new();
        match peval(code.clone(), &mut env) {
            Value::Float(x) => assert_eq!(x, 80.0),
            result => panic!("result of {} is not float: {}", code, result),
        }
    }

    #[test]
    fn test_alt() {
        let code = Term::Alt(
            Box::new(Term::Bool(true)),
            Box::new(Term::Float(80.0)),
            Box::new(Term::Float(90.0)),
        );
        let mut env = Env::new();
        match peval(code.clone(), &mut env) {
            Value::Float(x) => assert_eq!(x, 80.0),
            result => panic!("result of {} is not float: {}", code, result),
        }
    }

    #[test]
    fn test_tuple() {
        let code = Term::Tuple(vec![
            Term::Float(80.0),
            Term::Float(90.0),
        ]);
        let mut env = Env::new();
        match peval(code.clone(), &mut env) {
            Value::Tuple(mut values) => {
                assert_eq!(values.len(), 2);
                match values.pop().unwrap() {
                    Value::Float(x) => assert_eq!(x, 90.0),
                    result => panic!("result of {} is not float: {}", code, result),
                }
                match values.pop().unwrap() {
                    Value::Float(x) => assert_eq!(x, 80.0),
                    result => panic!("result of {} is not float: {}", code, result),
                }
            }
            result => panic!("result of {} is not tuple: {}", code, result),
        }
    }
}