use super::*;

pub type Env = Vec<Value>;

pub fn eval(term: Term, env: &mut Env) -> Value {
    match term {
        Term::Float(x) => Value::Float(x),
        Term::Bool(x) => Value::Bool(x),
        Term::Var(v) => env
            .get(env.len() - v as usize - 1)
            .unwrap()
            .clone(),
        Term::Apply(func, arg) => eval(*func, env).apply(eval(*arg, env)),
        Term::Extern(x) => x.into(),
        Term::Func(return_type, name, body) => Value::Func(
            return_type,
            Closure(body, env.clone(), name),
        ),
        Term::Let(_, _, body, next) => {
            let value = eval(*body, env);
            env.push(value);
            eval(*next, env)
        }
        Term::Alt(cond, then, else_) => match eval(*cond, env) {
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
        match eval(code.clone(), &mut env) {
            Value::Float(x) => assert_eq!(x, 80.0),
            result => panic!("result of {} is not float: {}", code, result),
        }
    }

    #[test]
    fn test_numeric() {
        let code = Term::Apply(
            Term::Apply(
                Box::new(Term::Extern(Extern::Mul)),
                Box::new(Term::Float(1.4)),
            ).into(),
            Term::Apply(
                Term::Apply(
                    Box::new(Term::Extern(Extern::Add)),
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
}