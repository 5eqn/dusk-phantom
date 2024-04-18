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
