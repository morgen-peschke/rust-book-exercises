use std::{assert_eq, format, panic};

use crate::lexer::lex;

use super::token::Token;
use super::token::test::tokens_strategy;
use proptest::prelude::*;

proptest! {
    #[test]
    fn token_lexing_works (tokens in tokens_strategy()) {
        let input: String =
         tokens.0.iter()
            .map(|t| {
                match t {
                    Token::OpenGroup { group, .. } => group.open().to_string(),
                    Token::CloseGroup { group, .. } => group.close().to_string(),
                    Token::Plus { .. } => "+".to_string(),
                    Token::Minus { .. } => "-".to_string(),
                    Token::Divide { .. } => "/".to_string(),
                    Token::Multiply { .. } => "*".to_string(),
                    Token::Number { value, .. } => format!("{value}"),
                }
            })
            .map(|s| s + " ")
            .collect();
        let input = input.trim();
        let actual = match lex(input) {
            Ok(tokens) => tokens,
            Err(error) => panic!("Input failed to lex: {error}"),
        };
        assert_eq!(
            actual.with_zero_starts(),
            tokens.with_zero_starts()
        )
    }
}
