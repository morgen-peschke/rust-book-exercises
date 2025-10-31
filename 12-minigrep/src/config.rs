use anyhow::{Result, bail};
use thiserror::Error;

pub struct Config {
    pub target: String,
    pub pattern: String,
}

#[derive(Error, Debug)]
enum ConfigParseErr {
    #[error("Parsing failed: not enough arguments (expected 2, but was {0})")]
    NotEnoughArguments(usize),
    #[error("Parsing failed: too many arguments (expected 2, but was {0})")]
    TooManyArguments(usize),
}

impl Config {
    pub fn parse<T>(args: T) -> Result<Config>
    where
        T: IntoIterator<Item = String>,
    {
        let mut iter = args.into_iter().skip(1);
        match (iter.next(), iter.next(), iter.collect::<Vec<String>>()) {
            (Some(pattern), Some(target), remaining) if remaining.is_empty() => {
                Ok(Config { target, pattern })
            }
            (Some(_), Some(_), remaining) => {
                bail!(ConfigParseErr::TooManyArguments(remaining.len() + 2))
            }
            (a, b, remaining) => {
                let arg_count = [a.iter().count(), b.iter().count(), remaining.len()]
                    .iter()
                    .sum();
                bail!(ConfigParseErr::NotEnoughArguments(arg_count))
            }
        }
    }
}
