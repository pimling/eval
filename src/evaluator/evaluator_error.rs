use std::fmt;

#[derive(Debug, Clone)]
pub struct EvaluatorError;

impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluator Failed")
    }
}