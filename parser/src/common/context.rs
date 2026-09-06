use std::fmt::Display;

use crate::lexer::token::{Token, Tokens};

#[derive(Debug)]
pub enum Context {
    RemainingTokens(Tokens),
    RemainingString(String),
    NothingLeft,
}
impl From<Vec<Token>> for Context {
    fn from(value: Vec<Token>) -> Self {
        Context::RemainingTokens(Tokens(value))
    }
}
impl FromIterator<Token> for Context {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Context::RemainingTokens(Tokens(iter.into_iter().collect()))
    }
}
impl FromIterator<(usize, char)> for Context {
    fn from_iter<T: IntoIterator<Item = (usize, char)>>(iter: T) -> Self {
        let (_, ctx): (Vec<_>, String) = iter.into_iter().unzip();
        Context::RemainingString(ctx)
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Context::RemainingTokens(tokens) => write!(f, "{tokens}"),
            Context::RemainingString(string) => write!(f, "'{string}'"),
            Context::NothingLeft => write!(f, "<END>"),
        }
    }
}
