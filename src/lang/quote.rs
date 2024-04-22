use super::*;

pub fn quote(env_len: usize, val: Value) -> Term {
    match val {
        Value::Float(i) => Term::Float(i),
        Value::Bool(i) => Term::Bool(i),
        Value::Lib(i) => Term::Lib(i),
        Value::Func(return_type, closure) => {
            let temp_val = closure.apply(Value::Var(env_len as i32));
            Term::Func(return_type, "".into(), quote(env_len + 1, temp_val).into())
        },
        Value::Apply(func, args) => {
            let func = quote(env_len, *func);
            let args = args.into_iter().map(|a| quote(env_len, a));
            let mut result = func;
            for arg in args {
                result = Term::Apply(result.into(), arg.into());
            }
            result
        },
        Value::Var(i) => {
            let var_index = env_len - i as usize - 1;
            Term::Var(var_index as i32)
        },
        Value::Alt(cond, then, else_) => {
            let cond = quote(env_len, *cond);
            let then = quote(env_len, *then);
            let else_ = quote(env_len, *else_);
            Term::Alt(cond.into(), then.into(), else_.into())
        },
        _ => panic!("Cannot quote extern or unsupported value type"),
    }
}

pub fn simp(term: Term) -> Term {
    let val = eval(term, &mut Vec::new());
    quote(0, val)
}

// Unit tests
#[cfg(test)]
pub mod tests_eval {
    use super::*;

    #[test]
    fn test_quote() {
        let code = Term::Float(80.0);
        let result = quote(0, Value::Float(80.0));
        assert_eq!(code, result);
    }

    #[test]
    fn test_simp() {
        let code = Term::Func(
            ValueType::Float.into(),
            "".into(),
            Term::Float(800.0).into(),
        );
        let result = simp(Term::Func(
            ValueType::Float.into(),
            "".into(),
            Term::Apply(
                Term::Func(
                    ValueType::Float.into(),
                    "".into(),
                    Box::new(Term::Var(0)),
                ).into(),
                Box::new(Term::Float(800.0)),
            ).into(),
        ));
        assert_eq!(code, result);
    }
}