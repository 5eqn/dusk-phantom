use super::*;

pub type Env<'a> = Vec<Value<'a>>;

/// Evaluate a reference to a term
/// `term` and `env` will only be temporarily mutated
pub fn eval_ref<'a>(term: &mut Term, env: &mut Env<'a>) -> Value<'a> {
    match term {
        Term::Float(x) => Value::Float(*x),
        Term::Bool(x) => Value::Bool(*x),
        Term::Var(v) => env
            .get(env.len() - *v as usize - 1)
            .unwrap()
            .clone(),
        Term::Apply(func, arg) => eval_ref(func, env).apply(eval_ref(arg, env)),
        Term::Lib(x) => Value::Lib(x.clone()),
        Term::Tuple(terms) => Value::Tuple(terms.iter_mut().map(|t| eval_ref(t, env)).collect()),
        Term::Func(return_type, name, body) => Value::Func(
            return_type.clone(),
            Closure(body.clone(), env.clone(), name.clone()),
        ),
        Term::Let(_, _, body, next) => {
            let value = eval_ref(body, env);
            env.push(value);
            let result = eval_ref(next, env);
            env.pop();
            result
        }
        Term::Alt(cond, then, else_) => match eval_ref(cond, env) {
            Value::Bool(true) => eval_ref(then, env),
            Value::Bool(false) => eval_ref(else_, env),
            other => panic!("{} is not a boolean", other),
        },
    }
}

/// Evaluate a term
/// `env` will only be temporarily mutated
pub fn eval<'a>(term: Term, env: &mut Env<'a>) -> Value<'a> {
    match term {
        Term::Float(x) => Value::Float(x),
        Term::Bool(x) => Value::Bool(x),
        Term::Var(v) => env
            .get(env.len() - v as usize - 1)
            .unwrap()
            .clone(),
        Term::Apply(func, arg) => eval(*func, env).apply(eval(*arg, env)),
        Term::Lib(x) => Value::Lib(x),
        Term::Tuple(terms) => Value::Tuple(terms.into_iter().map(|t| eval(t, env)).collect()),
        Term::Func(return_type, name, body) => Value::Func(
            return_type,
            Closure(body, env.clone(), name),
        ),
        Term::Let(_, _, body, next) => {
            let value = eval(*body, env);
            env.push(value);
            let result = eval(*next, env);
            env.pop();
            result
        }
        Term::Alt(cond, then, else_) => match eval(*cond, env) {
            Value::Bool(true) => eval(*then, env),
            Value::Bool(false) => eval(*else_, env),
            other => {
                let then = eval(*then, env);
                let else_ = eval(*else_, env);
                Value::Alt(other.into(), then.into(), else_.into())
            }
        },
    }
}

/// Evaluate a closure (which includes owned env)
/// Consumes the environment
pub fn eval_closure(term: Term, mut env: Env) -> Value {
    match term {
        Term::Float(x) => Value::Float(x),
        Term::Bool(x) => Value::Bool(x),
        Term::Var(v) => env.swap_remove(env.len() - v as usize - 1),
        Term::Apply(func, arg) => {
            let arg = eval(*arg, &mut env);
            eval_closure(*func, env).apply(arg)
        },
        Term::Lib(x) => Value::Lib(x),
        Term::Tuple(terms) => Value::Tuple(terms.into_iter().map(|t| eval(t, &mut env)).collect()),
        Term::Func(return_type, name, body) => Value::Func(
            return_type,
            Closure(body, env, name),
        ),
        Term::Let(_, _, body, next) => {
            let value = eval(*body, &mut env);
            env.push(value);
            eval_closure(*next, env)
        }
        Term::Alt(cond, then, else_) => match eval(*cond, &mut env) {
            Value::Bool(true) => eval_closure(*then, env),
            Value::Bool(false) => eval_closure(*else_, env),
            other => {
                let then = eval(*then, &mut env);
                let else_ = eval_closure(*else_, env);
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
        match eval(code.clone(), &mut env) {
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
        match eval(code.clone(), &mut env) {
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
        match eval(code.clone(), &mut env) {
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
        match eval(code.clone(), &mut env) {
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
        match eval(code.clone(), &mut env) {
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
        match eval(code.clone(), &mut env) {
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