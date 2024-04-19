pub mod elaborate;
pub mod eval;
pub mod library;
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
pub use syntax::*;
pub use term::*;
pub use value::*;
pub use value_type::*;

pub type RunError = String;

pub fn run(code: &str) -> Result<Value, RunError> {
    let env = HashMap::new();
    let ctx = HashMap::new();
    let syntax = parse(code).map_err(|e| format!("Parse error: {}", e))?;
    let (term, _) = infer(syntax, ctx).map_err(|e| format!("Elaborate error: {}", e))?;
    eval(term, &env).map_err(|e| format!("Evaluation error: {}", e))
}
