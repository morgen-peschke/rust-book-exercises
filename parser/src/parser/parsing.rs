use std::{fmt::Display, iter::Peekable};

use super::expression::Expression;
use crate::{
    common::{Context, ParsingError},
    lexer::token::{Token, Tokens},
};

pub fn parse(tokens: &Tokens) -> Result<Expression, ParsingError> {
    let mut state = ParseState {
        index: 0,
        iter: tokens.0.iter().peekable(),
    };
    let exp = state.parse_expr()?;
    match state.peek_opt() {
        Some(t) => Err(ParsingError {
            message: format!("Expected end, but was {t}"),
            index: *t.start(),
            context: state.context(),
        }),
        None => Ok(exp),
    }
}

struct ParseState<'a, I: Clone + Iterator<Item = &'a Token>> {
    index: usize,
    iter: Peekable<I>,
}
// Error Functions
impl<'a, I: Clone + Iterator<Item = &'a Token>> ParseState<'a, I> {
    fn context(&mut self) -> Context {
        self.iter.clone().copied().collect()
    }

    fn unexpected_end_of_input_error(&self) -> ParsingError {
        ParsingError {
            message: "Unexpected end of input".to_string(),
            index: self.index,
            context: Context::NothingLeft,
        }
    }

    fn unexpected_token<E: Display>(&mut self, expected: E, actual: &Token) -> ParsingError {
        ParsingError {
            message: format!("expected {expected}, but was {actual}"),
            index: *actual.start(),
            context: self.context(),
        }
    }
}
// Advancement Functions
impl<'a, I: Clone + Iterator<Item = &'a Token>> ParseState<'a, I> {
    fn next_opt(&mut self) -> Option<&'a Token> {
        self.index += 1;
        self.iter.next()
    }

    fn peek_opt(&mut self) -> Option<&'a Token> {
        self.iter.peek().copied()
    }

    fn next(&mut self) -> Result<&'a Token, ParsingError> {
        self.next_opt()
            .ok_or_else(|| self.unexpected_end_of_input_error())
    }

    fn skip(&mut self, expected: Token) -> Result<&'a Token, ParsingError> {
        let actual = self.next()?;
        if !actual.equivalent_to(&expected) {
            Err(self.unexpected_token(expected, actual))
        } else {
            Ok(actual)
        }
    }
}
// Parse Functions
impl<'a, I: Clone + Iterator<Item = &'a Token>> ParseState<'a, I> {
    // Tightest binding: unary -, grouping, bare numbers
    // Grammar:
    // term -> NUMBER | ( expr ) | [ expr ] | { expr } | - term
    fn parse_term(&mut self) -> Result<Expression, ParsingError> {
        match self.next()? {
            Token::OpenGroup { group, .. } => {
                let expression = self.parse_expr()?;
                self.skip(Token::CloseGroup {
                    group: *group,
                    start: 0,
                })?;
                Ok(expression.bracket_with(*group))
            }
            Token::Number { value, .. } => Ok((*value).into()),
            Token::Minus { .. } => Ok(-self.parse_term()?),
            actual => Err(self.unexpected_token("number, unary -, or start of group", actual)),
        }
    }

    // Because */ have tighter binding than -+, they're "the things getting added",
    // or the "summand" (even if it's being subtracted)
    // Grammar:
    // summand -> term * summand | term / summand | term
    fn parse_summand(&mut self) -> Result<Expression, ParsingError> {
        let lhs: Expression = self.parse_term()?;
        match self.peek_opt() {
            Some(Token::Divide { .. }) => {
                self.next()?;
                Ok(lhs / self.parse_summand()?)
            }
            Some(Token::Multiply { .. }) => {
                self.next()?;
                Ok(lhs * self.parse_summand()?)
            }
            _ => {
                // Just the term itself
                Ok(lhs)
            }
        }
    }

    // Grammar:
    // expr -> summand + expr | summand - expr | summand
    fn parse_expr(&mut self) -> Result<Expression, ParsingError> {
        let lhs: Expression = self.parse_summand()?;
        match self.peek_opt() {
            Some(Token::Plus { .. }) => {
                self.next()?;
                Ok(lhs + self.parse_summand()?)
            }
            Some(Token::Minus { .. }) => {
                self.next()?;
                Ok(lhs - self.parse_summand()?)
            }
            _ => {
                // Just the summand itself
                Ok(lhs)
            }
        }
    }
}
