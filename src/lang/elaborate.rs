use super::*;

pub type ElaborateError = String;

pub fn elaborate(syntax: Syntax) -> Result<Term, ElaborateError> {
    match syntax {
        Syntax::Float(x) => Ok(Term::Float(x)),
        Syntax::Var(x) => Ok(Term::Var(x)),
        Syntax::Apply(func, arg) => Ok(Term::Apply(
            elaborate(*func)?.into(),
            elaborate(*arg)?.into(),
        )),
        Syntax::Lib(x) => Ok(Term::Lib(x)),
    }
}
