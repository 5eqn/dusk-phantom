use super::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lalr); // synthesized by LALRPOP

pub type ParseError = String;

pub fn parse(input: &str) -> Result<Syntax, ParseError> {
    match lalr::SyntaxParser::new().parse(input) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

// Unit tests
#[cfg(test)]
pub mod tests_expr {
    use super::*;

    #[test]
    fn test_minimal() {
        let code = "80";
        match parse(code) {
            Ok(result) => assert_eq!(result, Syntax::Float(80.0)),
            Err(err) => panic!("failed to parse {}: {}", code, err),
        }
    }

    #[test]
    fn test_numeric() {
        let code = "1.4*(2+3)";
        match parse(code) {
            Ok(result) => assert_eq!(result, Syntax::Apply(
                Syntax::Apply(
                    Box::new(Syntax::Lib(Lib::Mul)),
                    Box::new(Syntax::Float(1.4)),
                ).into(),
                Syntax::Apply(
                    Syntax::Apply(
                        Box::new(Syntax::Lib(Lib::Add)),
                        Box::new(Syntax::Float(2.0)),
                    ).into(),
                    Syntax::Float(3.0).into(),
                ).into(),
            )),
            Err(err) => panic!("failed to parse {}: {}", code, err),
        }
    }

    #[test]
    fn test_func() {
        let code = "(x: float) => x";
        match parse(code) {
            Ok(result) => assert_eq!(result, Syntax::Func(
                Box::new(ValueType::Float),
                "x".to_string(),
                Box::new(Syntax::Var("x".to_string())),
            )),
            Err(err) => panic!("failed to parse {}: {}", code, err),
        }
    }

    #[test]
    fn test_id() {
        let code = "((x: float) => x)(1.4)";
        match parse(code) {
            Ok(result) => assert_eq!(result, Syntax::Apply(
                Box::new(Syntax::Func(
                    Box::new(ValueType::Float),
                    "x".to_string(),
                    Box::new(Syntax::Var("x".to_string())),
                )),
                Box::new(Syntax::Float(1.4)),
            )),
            Err(err) => panic!("failed to parse {}: {}", code, err),
        }
    }

    #[test]
    fn test_apply() {
        let code = "(f: float -> float) => (x: float) => f(x)";
        match parse(code) {
            Ok(result) => assert_eq!(result, Syntax::Func(
                Box::new(ValueType::Func(Box::new(ValueType::Float), Box::new(ValueType::Float))),
                "f".to_string(),
                Box::new(Syntax::Func(
                    Box::new(ValueType::Float),
                    "x".to_string(),
                    Box::new(Syntax::Apply(
                        Box::new(Syntax::Var("f".to_string())),
                        Box::new(Syntax::Var("x".to_string())),
                    )),
                )),
            )),
            Err(err) => panic!("failed to parse {}: {}", code, err),
        }
    }

    #[test]
    fn test_let() {
        let code = "let x: float = 80 in x";
        match parse(code) {
            Ok(result) => assert_eq!(result, Syntax::Let(
                Box::new(ValueType::Float),
                "x".to_string(),
                Box::new(Syntax::Float(80.0)),
                Box::new(Syntax::Var("x".to_string())),
            )),
            Err(err) => panic!("failed to parse {}: {}", code, err),
        }
    }

    #[test]
    fn test_alt() {
        let code = "if 1.4 < 2.0 then 1.4 else 2.0";
        match parse(code) {
            Ok(result) => assert_eq!(result, Syntax::Alt(
                Box::new(Syntax::Apply(
                    Box::new(Syntax::Apply(
                        Box::new(Syntax::Lib(Lib::Lt)),
                        Box::new(Syntax::Float(1.4)),
                    )),
                    Box::new(Syntax::Float(2.0)),
                )),
                Box::new(Syntax::Float(1.4)),
                Box::new(Syntax::Float(2.0)),
            )),
            Err(err) => panic!("failed to parse {}: {}", code, err),
        }
    }
}
