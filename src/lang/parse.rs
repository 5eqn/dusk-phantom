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
    fn test_complex() {
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
}
