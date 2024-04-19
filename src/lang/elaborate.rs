use super::*;

pub type ElaborateError = String;

pub type Ctx = HashMap<String, ValueType>;

/// float -> float -> float
fn magma_float() -> ValueType {
    ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float))))
}

pub fn infer(syntax: Syntax, ctx: Ctx) -> Result<(Term, ValueType), ElaborateError> {
    match syntax {
        Syntax::Float(value) => Ok((Term::Float(value), ValueType::Float)),
        Syntax::Var(name) => match ctx.get(&name) {
            Some(value_type) => Ok((Term::Var(name), value_type.clone())),
            None => Err(format!("Variable not found: {}", name)),
        },
        Syntax::Lib(lib) => match lib {
            Lib::Add => Ok((Term::Lib(Lib::Add), magma_float())),
            Lib::Sub => Ok((Term::Lib(Lib::Sub), magma_float())),
            Lib::Mul => Ok((Term::Lib(Lib::Mul), magma_float())),
            Lib::Div => Ok((Term::Lib(Lib::Div), magma_float())),
        },
        Syntax::Apply(func, arg) => {
            let (func_term, func_type) = infer(*func, ctx.clone())?;
            match func_type {
                ValueType::Func(param_type, ret_type) => {
                    let arg_term = check(*arg, ctx.clone(), *param_type)?;
                    Ok((Term::Apply(Box::new(func_term), Box::new(arg_term)), *ret_type))
                }
                _ => Err(format!("Not a function: {}", func_type)),
            }
        }
        Syntax::Func(param_type, name, body) => {
            let new_ctx = {
                let mut new_ctx = ctx.clone();
                new_ctx.insert(name.clone(), *param_type.clone());
                new_ctx
            };
            let (body_term, body_type) = infer(*body, new_ctx)?;
            Ok((Term::Func(param_type.clone(), name, Box::new(body_term)), ValueType::Func(param_type, Box::new(body_type))))
        }
    }
}

pub fn check(syntax: Syntax, ctx: Ctx, expected: ValueType) -> Result<Term, ElaborateError> {
    let (term, inferred_type) = infer(syntax, ctx)?;
    if inferred_type == expected {
        Ok(term)
    } else {
        Err(format!("Type mismatch: {} != {}", inferred_type, expected))
    }
}

#[cfg(test)]
pub mod tests_elaborate {
    use super::*;

    #[test]
    fn test_minimal() {
        let code = Syntax::Float(80.0);
        let ctx = Ctx::new();
        match infer(code.clone(), ctx) {
            Ok((term, value_type)) => {
                assert_eq!(term, Term::Float(80.0));
                assert_eq!(value_type, ValueType::Float);
            }
            Err(err) => panic!("failed to infer {:?}: {}", code, err),
        }
    }

    #[test]
    fn test_func() {
        let code = Syntax::Func(
            Box::new(ValueType::Float),
            "x".to_string(),
            Box::new(Syntax::Var("x".to_string())),
        );
        let ctx = Ctx::new();
        match infer(code.clone(), ctx) {
            Ok((term, value_type)) => {
                assert_eq!(term, Term::Func(
                    Box::new(ValueType::Float),
                    "x".to_string(),
                    Box::new(Term::Var("x".to_string())),
                ));
                assert_eq!(value_type, ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float)));
            }
            Err(err) => panic!("failed to infer {:?}: {}", code, err),
        }
    }
}