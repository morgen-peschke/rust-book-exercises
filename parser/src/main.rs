mod common;
mod lexer;
mod parser;

use std::{env, println};

use crate::common::ParsingError;
use crate::parser::expression::{BinaryOp, UnaryOp};
use crate::parser::{Expression, parse};
use bigdecimal::BigDecimal;
use lexer::lex;

/// Based on post + code from https://adriann.github.io/rust_parser.html
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let raw = args.join(" ");
    if !raw.is_empty() {
        match parse_loud(&raw) {
            Ok(_) => (),
            Err(error) => println!("Failed: {error}"),
        }
    }
}

fn parse_loud(input: &str) -> Result<(), ParsingError> {
    println!("Input: {input}");
    let tokens = lex(input)?;
    let raw_expr = parse(&tokens)?;
    let normal_expr = raw_expr.clone().normalized();
    println!();
    println!(" :: Parsed ::");
    println!("Raw   : {raw_expr}");
    println!("Normal: {normal_expr}");
    println!(" :: Lispy ::");
    println!("Raw   : {raw_expr:?}");
    println!("Normal: {normal_expr:?}");

    println!();
    println!("Evaluated: {}", evaluate(&raw_expr));
    println!();
    println!("Lexed   : {tokens}");
    println!("Unparsed: {}", normal_expr.unparse());
    Ok(())
}

fn evaluate(expression: &Expression) -> BigDecimal {
    match expression {
        Expression::Number { value } => BigDecimal::from(value),
        Expression::Unary { op, child } => match op {
            UnaryOp::Group { .. } => evaluate(child),
            UnaryOp::Negate => -evaluate(child),
        },
        Expression::Binary { op, lhs, rhs } => match op {
            BinaryOp::Plus => evaluate(lhs) + evaluate(rhs),
            BinaryOp::Minus => evaluate(lhs) - evaluate(rhs),
            BinaryOp::Divide => evaluate(lhs) / evaluate(rhs),
            BinaryOp::Multiply => evaluate(lhs) * evaluate(rhs),
        },
    }
}
