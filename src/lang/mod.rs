pub mod elaborate;
pub mod eval;
pub mod quote;
pub mod library;
pub mod external;
pub mod parse;
pub mod syntax;
pub mod term;
pub mod value;
pub mod value_type;

use std::collections::HashMap;

use elaborate::*;
use eval::*;
pub use library::*;
use parse::*;
pub use quote::*;
pub use syntax::*;
pub use term::*;
pub use value::*;
pub use external::*;
pub use value_type::*;

pub type RunError = String;

fn target_type() -> ValueType {
    ValueType::Func(
        Box::new(ValueType::Func(
            Box::new(ValueType::Float),
            Box::new(ValueType::Float),
        )),
        Box::new(ValueType::Func(
            Box::new(ValueType::Float),
            Box::new(ValueType::Float),
        )),
    )
}

pub fn run<'a>(code: &str) -> Result<Value<'a>, RunError> {
    let mut env = Vec::new();
    let ctx = HashMap::new();
    let syntax = parse(code).map_err(|e| format!("Parse error: {}", e))?;
    let term = check(syntax, ctx, target_type(), 0).map_err(|e| format!("Elaborate error: {}", e))?;
    let simp_term = simp(term);
    Ok(eval(simp_term, &mut env))
}
