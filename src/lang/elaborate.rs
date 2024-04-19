use super::*;

pub type ElaborateError = String;

pub type Ctx = HashMap<String, ValueType>;

/// float -> float -> float
fn type_op() -> ValueType {
    ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float))))
}

/// float -> float -> bool
fn type_cmp() -> ValueType {
    ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Bool))))
}

pub fn infer(syntax: Syntax, ctx: Ctx) -> Result<(Term, ValueType), ElaborateError> {
    match syntax {
        Syntax::Float(value) => Ok((Term::Float(value), ValueType::Float)),
        Syntax::Bool(value) => Ok((Term::Bool(value), ValueType::Bool)),
        Syntax::Var(name) => match ctx.get(&name) {
            Some(value_type) => Ok((Term::Var(name), value_type.clone())),
            None => Err(format!("Variable not found: {}", name)),
        },
        Syntax::Lib(lib) => match lib {
            Lib::Add => Ok((Term::Lib(Lib::Add), type_op())),
            Lib::Sub => Ok((Term::Lib(Lib::Sub), type_op())),
            Lib::Mul => Ok((Term::Lib(Lib::Mul), type_op())),
            Lib::Div => Ok((Term::Lib(Lib::Div), type_op())),
            Lib::Lt => Ok((Term::Lib(Lib::Lt), type_cmp())),
            Lib::Le => Ok((Term::Lib(Lib::Le), type_cmp())),
            Lib::Gt => Ok((Term::Lib(Lib::Gt), type_cmp())),
            Lib::Ge => Ok((Term::Lib(Lib::Ge), type_cmp())),
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
        Syntax::Let(value_type, name, body, next) => {
            let body_term = check(*body, ctx.clone(), *value_type.clone())?;
            let new_ctx = {
                let mut new_ctx = ctx.clone();
                new_ctx.insert(name.clone(), *value_type.clone());
                new_ctx
            };
            let (next_term, next_type) = infer(*next, new_ctx)?;
            Ok((Term::Let(value_type, name, Box::new(body_term), Box::new(next_term)), next_type))
        }
        Syntax::Alt(cond, then, else_) => {
            let cond_term = check(*cond, ctx.clone(), ValueType::Bool)?;
            let (then_term, then_type) = infer(*then, ctx.clone())?;
            let (else_term, else_type) = infer(*else_, ctx.clone())?;
            let ty = unify(then_type, else_type)?;
            Ok((Term::Alt(Box::new(cond_term), Box::new(then_term), Box::new(else_term)), ty))
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

pub fn unify(t1: ValueType, t2: ValueType) -> Result<ValueType, ElaborateError> {
    match (t1, t2) {
        (ValueType::Float, ValueType::Float) => Ok(ValueType::Float),
        (ValueType::Bool, ValueType::Bool) => Ok(ValueType::Bool),
        (ValueType::Func(p1, r1), ValueType::Func(p2, r2)) => {
            let p = unify(*p1, *p2)?;
            let r = unify(*r1, *r2)?;
            Ok(ValueType::Func(Box::new(p), Box::new(r)))
        }
        (t1, t2) => Err(format!("Unification failed: {} != {}", t1, t2)),
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

    #[test]
    fn test_let() {
        let code = Syntax::Let(
            Box::new(ValueType::Float),
            "x".to_string(),
            Box::new(Syntax::Float(80.0)),
            Box::new(Syntax::Var("x".to_string())),
        );
        let ctx = Ctx::new();
        match infer(code.clone(), ctx) {
            Ok((term, value_type)) => {
                assert_eq!(term, Term::Let(
                    ValueType::Float.into(),
                    "x".to_string(),
                    Box::new(Term::Float(80.0)),
                    Box::new(Term::Var("x".to_string())),
                ));
                assert_eq!(value_type, ValueType::Float);
            }
            Err(err) => panic!("failed to infer {:?}: {}", code, err),
        }
    }

    #[test]
    fn test_alt() {
        let code = Syntax::Alt(
            Box::new(Syntax::Bool(true)),
            Box::new(Syntax::Float(80.0)),
            Box::new(Syntax::Float(90.0)),
        );
        let ctx = Ctx::new();
        match infer(code.clone(), ctx) {
            Ok((term, value_type)) => {
                assert_eq!(term, Term::Alt(
                    Box::new(Term::Bool(true)),
                    Box::new(Term::Float(80.0)),
                    Box::new(Term::Float(90.0)),
                ));
                assert_eq!(value_type, ValueType::Float);
            }
            Err(err) => panic!("failed to infer {:?}: {}", code, err),
        }
    }
}