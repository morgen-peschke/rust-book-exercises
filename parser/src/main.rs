mod common;
mod lexer;
mod parser;

use clap::Parser;
use std::fmt::Display;
use std::process::exit;
use std::str::FromStr;
use std::{eprintln, println};

use crate::common::ParsingError;
use crate::parser::expression::{BinaryOp, UnaryOp};
use crate::parser::{Expression, parse};
use bigdecimal::BigDecimal;
use lexer::lex;

fn main() {
    let cli = Cli::parse();
    if let Err(error) = cli.process() {
        eprintln!("Failed: {error}");
        exit(1)
    }
}

#[derive(Clone)]
enum BoolArg {
    True,
    False,
}
impl BoolArg {
    pub fn is_true(&self) -> bool {
        match self {
            BoolArg::True => true,
            BoolArg::False => false,
        }
    }
}
impl Display for BoolArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BoolArg::True => "true",
            BoolArg::False => "false",
        })
    }
}
impl FromStr for BoolArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normal = s.trim().to_ascii_lowercase();
        if normal == "true" || "true".starts_with(&normal) {
            Ok(BoolArg::True)
        } else if normal == "false" || "false".starts_with(&normal) {
            Ok(BoolArg::False)
        } else {
            Err("expected 'true' or 'false'".to_string())
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = Some("Parse an arithmetic expression"))]
struct Cli {
    #[arg(short = 'L', long, help = "Show the lexed tokens", default_value_t = BoolArg::False)]
    lexed: BoolArg,
    #[arg(short = 'P', long, help = "Show the parsed expression", default_value_t = BoolArg::True)]
    parsed: BoolArg,
    #[arg(short, long, help = "Normalize the parsed expression", default_value_t = BoolArg::True)]
    normalized: BoolArg,
    #[arg(short = 'l', long, help = "Print the expression in lispy style", default_value_t = BoolArg::False)]
    lispy: BoolArg,
    #[arg(short, long, help = "Print the result of evaluating the expression", default_value_t = BoolArg::False)]
    evaluated: BoolArg,
    #[arg(short = 'p', long, help = "Unparse the expression and print the result", default_value_t = BoolArg::False)]
    unparsed: BoolArg,
    #[arg(short, long, help = "The string to parse")]
    input: String,
}
impl Cli {
    pub fn process(self) -> Result<(), ParsingError> {
        if self.input.is_empty() {
            return Err(ParsingError {
                message: "Empty input".to_string(),
                index: 0,
                context: common::Context::NothingLeft,
            });
        };
        let tokens = lex(&self.input)?;
        let expr = {
            let raw = parse(&tokens)?;
            if self.normalized.is_true() {
                raw.normalized()
            } else {
                raw
            }
        };
        let result = evaluate(&expr);
        if self.parsed.is_true() {
            match &self {
                Cli {
                    lispy: BoolArg::False,
                    evaluated: BoolArg::False,
                    ..
                } => println!("{expr}"),
                Cli {
                    lispy: BoolArg::True,
                    evaluated: BoolArg::False,
                    ..
                } => println!("{expr:?}"),
                Cli {
                    lispy: BoolArg::False,
                    evaluated: BoolArg::True,
                    ..
                } => println!("{expr} = {result}"),
                Cli {
                    lispy: BoolArg::True,
                    evaluated: BoolArg::True,
                    ..
                } => println!("{expr:?} = {result}"),
            };
        }
        match &self {
            Cli {
                lexed: BoolArg::True,
                unparsed: BoolArg::False,
                ..
            } => {
                println!("Lexed: {tokens}");
            }
            Cli {
                lexed: BoolArg::False,
                unparsed: BoolArg::True,
                ..
            } => {
                println!("Unparsed: {}", expr.unparse());
            }
            Cli {
                lexed: BoolArg::True,
                unparsed: BoolArg::True,
                ..
            } => {
                println!("Lexed   : {tokens}");
                println!("Unparsed: {}", expr.unparse());
            }
            Cli {
                lexed: BoolArg::False,
                unparsed: BoolArg::False,
                ..
            } => (),
        }
        Ok(())
    }
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
