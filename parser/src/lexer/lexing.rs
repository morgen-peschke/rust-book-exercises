use std::{format, iter::Peekable};

use crate::{
    common::{Group, ParsingError},
    lexer::token::Tokens,
};

use super::token::Token;

pub fn lex(input: &str) -> Result<Tokens, ParsingError> {
    let mut result: Vec<Token> = Vec::new();

    let mut it = input.chars().enumerate().peekable();
    while let Some(&(start, c)) = it.peek() {
        match c {
            ' ' => {
                it.next();
            }
            '0'..='9' => {
                let num = get_number(start, &mut it)?;
                result.push(Token::Number { value: num, start });
            }
            '+' => {
                result.push(Token::Plus { start });
                it.next();
            }
            '-' => {
                result.push(Token::Minus { start });
                it.next();
            }
            '/' => {
                result.push(Token::Divide { start });
                it.next();
            }
            '*' => {
                result.push(Token::Multiply { start });
                it.next();
            }
            '(' => {
                result.push(Token::OpenGroup {
                    group: Group::Paren,
                    start,
                });
                it.next();
            }
            ')' => {
                result.push(Token::CloseGroup {
                    group: Group::Paren,
                    start,
                });
                it.next();
            }
            '{' => {
                result.push(Token::OpenGroup {
                    group: Group::Brace,
                    start,
                });
                it.next();
            }
            '}' => {
                result.push(Token::CloseGroup {
                    group: Group::Brace,
                    start,
                });
                it.next();
            }
            '[' => {
                result.push(Token::OpenGroup {
                    group: Group::Bracket,
                    start,
                });
                it.next();
            }
            ']' => {
                result.push(Token::CloseGroup {
                    group: Group::Bracket,
                    start,
                });
                it.next();
            }
            _ => {
                return Err(ParsingError {
                    message: format!("unexpected character {c}"),
                    index: start,
                    context: it.collect(),
                });
            }
        }
    }
    Ok(Tokens(result))
}

fn get_number<T: Iterator<Item = (usize, char)>>(
    index: usize,
    iter: &mut Peekable<T>,
) -> Result<u64, ParsingError> {
    let mut buffer: Vec<char> = Vec::new();
    while let Some(&(_, c @ '0'..='9')) = iter.peek() {
        buffer.push(c);
        iter.next();
    }
    let str: String = buffer.into_iter().collect();
    str.parse().map_err(|e| ParsingError {
        message: format!("unable to parse '{str}' as number: {e}"),
        index,
        context: iter.collect(),
    })
}
