use super::*;

pub type Env<'a> = Vec<Value<'a>>;

/// Evaluate a term
/// `term` and `env` will only be temporarily mutated
pub fn ref_eval<'a>(term: &mut Term, env: &mut Env<'a>) -> Value<'a> {
    match term {
        Term::Float(x) => Value::Float(*x),
        Term::Bool(x) => Value::Bool(*x),
        Term::Var(v) => env
            .get(env.len() - *v as usize - 1)
            .unwrap()
            .clone(),
        Term::Apply(func, arg) => ref_eval(func, env).apply(ref_eval(arg, env)),
        Term::Lib(x) => Value::Lib(x.clone()),
        Term::Func(return_type, name, body) => Value::Func(
            return_type.clone(),
            Closure(body.clone(), env.clone(), name.clone()),
        ),
        Term::Let(_, _, body, next) => {
            let value = ref_eval(body, env);
            env.push(value);
            let result = ref_eval(next, env);
            env.pop();
            result
        }
        Term::Alt(cond, then, else_) => match ref_eval(cond, env) {
            Value::Bool(true) => ref_eval(then, env),
            Value::Bool(false) => ref_eval(else_, env),
            other => panic!("{} is not a boolean", other),
        },
    }
}

/// Evaluate a term
/// `env` will only be temporarily mutated
pub fn ref_env_eval<'a>(term: Term, env: &mut Env<'a>) -> Value<'a> {
    match term {
        Term::Float(x) => Value::Float(x),
        Term::Bool(x) => Value::Bool(x),
        Term::Var(v) => env
            .get(env.len() - v as usize - 1)
            .unwrap()
            .clone(),
        Term::Apply(func, arg) => ref_env_eval(*func, env).apply(ref_env_eval(*arg, env)),
        Term::Lib(x) => Value::Lib(x),
        Term::Func(return_type, name, body) => Value::Func(
            return_type,
            Closure(body, env.clone(), name),
        ),
        Term::Let(_, _, body, next) => {
            let value = ref_env_eval(*body, env);
            env.push(value);
            let result = ref_env_eval(*next, env);
            env.pop();
            result
        }
        Term::Alt(cond, then, else_) => match ref_env_eval(*cond, env) {
            Value::Bool(true) => ref_env_eval(*then, env),
            Value::Bool(false) => ref_env_eval(*else_, env),
            other => panic!("{} is not a boolean", other),
        },
    }
}

/// Evaluate a term
/// Consumes the environment
pub fn eval(term: Term, mut env: Env) -> Value {
    match term {
        Term::Float(x) => Value::Float(x),
        Term::Bool(x) => Value::Bool(x),
        Term::Var(v) => env.swap_remove(env.len() - v as usize - 1),
        Term::Apply(func, arg) => {
            let arg = ref_env_eval(*arg, &mut env);
            eval(*func, env).apply(arg)
        },
        Term::Lib(x) => Value::Lib(x),
        Term::Func(return_type, name, body) => Value::Func(
            return_type,
            Closure(body, env, name),
        ),
        Term::Let(_, _, body, next) => {
            let value = ref_env_eval(*body, &mut env);
            env.push(value);
            eval(*next, env)
        }
        Term::Alt(cond, then, else_) => match ref_env_eval(*cond, &mut env) {
            Value::Bool(true) => eval(*then, env),
            Value::Bool(false) => eval(*else_, env),
            other => panic!("{} is not a boolean", other),
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
        match ref_env_eval(code.clone(), &mut env) {
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
        match ref_env_eval(code.clone(), &mut env) {
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
        match ref_env_eval(code.clone(), &mut env) {
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
        match ref_env_eval(code.clone(), &mut env) {
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
        match ref_env_eval(code.clone(), &mut env) {
            Value::Float(x) => assert_eq!(x, 80.0),
            result => panic!("result of {} is not float: {}", code, result),
        }
    }
}