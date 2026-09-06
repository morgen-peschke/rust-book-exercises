use std::fmt::Display;

use super::Context;

pub struct ParsingError {
    pub message: String,
    pub index: usize,
    pub context: Context,
}
impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parsing failure at index {} (before: {}) - {}",
            self.index, self.context, self.message
        )
    }
}
